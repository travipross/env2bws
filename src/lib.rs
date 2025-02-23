#![doc = include_str!("../README.md")]
pub use cli::Cli;
pub use dotenv::{DotEnvFile, EnvVar};
pub use import_payload::{ImportPayload, Project, ProjectAssignment, Secret};

pub mod cli;
pub mod dotenv;
pub mod import_payload;

#[cfg(test)]
mod test_sample {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn sample_env_parses_and_outputs_correctly() {
        let dotenv = DotEnvFile::parse_from_file(PathBuf::from("sample.env"), true, false)
            .expect("could not parse file");

        let expected_output = include_str!("../sample.json");
        let expected_payload = serde_json::from_str::<ImportPayload>(&expected_output)
            .expect("could not deserialize expected JSON");
        let import_payload = ImportPayload::from_dotenv(dotenv, ProjectAssignment::None);

        // Projects field matches
        assert_eq!(expected_payload.projects, import_payload.projects);

        // Same number of secrets
        assert_eq!(expected_payload.secrets.len(), import_payload.secrets.len());

        // Same secret contents
        expected_payload
            .secrets
            .iter()
            .zip(import_payload.secrets)
            .for_each(|(s1, s2)| {
                assert_eq!(s1.key, s2.key);
                assert_eq!(s1.value, s2.value);
                assert_eq!(s1.note, s2.note);
                assert_ne!(s1.id, s2.id);  // IDs are randomly generated and shouldn't match
            });
    }
}
