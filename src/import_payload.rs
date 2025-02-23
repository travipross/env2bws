use uuid::Uuid;

use crate::{dotenv::EnvVar, DotEnvFile};

#[derive(Debug, Clone, serde::Serialize)]
pub(crate) struct Project {
    pub(crate) id: uuid::Uuid,
    pub(crate) name: String,
}

#[derive(Debug, Clone, serde::Serialize)]
pub(crate) struct Secret {
    key: String,
    value: String,
    note: String,
    project_ids: Vec<uuid::Uuid>,
    id: uuid::Uuid,
}

impl Secret {
    fn from_env_var(value: EnvVar, project_id: Option<uuid::Uuid>) -> Self {
        Self {
            key: value.key,
            value: value.value,
            note: value.comment.unwrap_or_default(),
            project_ids: project_id.map_or(vec![], |id| vec![id]),
            id: value.temp_id,
        }
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub(crate) struct ImportPayload {
    projects: Vec<Project>,
    secrets: Vec<Secret>,
}

pub(crate) enum ProjectAssignment {
    None,
    Existing(uuid::Uuid),
    New(String),
}

impl ImportPayload {
    pub(crate) fn from_dotenv(dotenv: DotEnvFile, project_assignment: ProjectAssignment) -> Self {
        // Empty vector of projects means no projects are to be created
        let mut projects: Vec<Project> = vec![];

        // Determine the ID of the project that all secrets will be assigned to (if any)
        let assigned_id = match project_assignment {
            // If existing case, assign the provided ID to the project
            ProjectAssignment::Existing(id) => Some(id),
            // If new case, create a new project declaration with random UUID and assign the ID to
            // the project
            ProjectAssignment::New(name) => {
                let id = Uuid::new_v4();
                projects.push(Project { id, name });
                Some(id)
            }
            // If none case, assign no project ID to the secrets
            ProjectAssignment::None => None,
        };

        Self {
            projects,
            secrets: dotenv
                .vars()
                .into_iter()
                .map(|v| Secret::from_env_var(v, assigned_id))
                .collect(),
        }
    }
}

#[cfg(test)]
mod payload_tests {
    use fake::{Fake, Faker};

    use super::*;

    #[test]
    fn leaves_project_blank_on_secrets_when_none_supplied() {
        let dotenv = Faker.fake::<DotEnvFile>();
        let payload = ImportPayload::from_dotenv(dotenv, ProjectAssignment::None);

        // No new projects listed
        assert_eq!(payload.projects.len(), 0);

        // No projects listed on any secret
        payload
            .secrets
            .iter()
            .for_each(|secret| assert_eq!(secret.project_ids.len(), 0));
    }

    #[test]
    fn defines_new_project_and_sets_for_secrets() {
        let dotenv = Faker.fake::<DotEnvFile>();
        let payload = ImportPayload::from_dotenv(dotenv, ProjectAssignment::New(Faker.fake()));

        // One new project listed
        assert_eq!(payload.projects.len(), 1);
        let project_id = payload
            .projects
            .first()
            .expect("could not get first project")
            .id;

        // All secrets assigned to new project's id
        payload.secrets.iter().for_each(|secret| {
            assert_eq!(secret.project_ids.len(), 1);
            assert_eq!(
                secret
                    .project_ids
                    .first()
                    .expect("could not get project id for secret"),
                &project_id
            )
        });
    }

    #[test]
    fn sets_existing_project_for_secrets() {
        let dotenv = Faker.fake::<DotEnvFile>();
        let project_id = Faker.fake::<Uuid>();
        let payload =
            ImportPayload::from_dotenv(dotenv, ProjectAssignment::Existing(project_id.clone()));

        // No new projects listed
        assert_eq!(payload.projects.len(), 0);

        // All secrets assigned to existing project id
        payload.secrets.iter().for_each(|secret| {
            assert_eq!(secret.project_ids.len(), 1);
            assert_eq!(
                secret
                    .project_ids
                    .first()
                    .expect("could not get project id for secret"),
                &project_id
            )
        });
    }
}
