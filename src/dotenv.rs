use std::{fs, path::PathBuf};

#[derive(Debug, Clone)]
pub(crate) struct EnvVar {
    pub(crate) key: String,
    pub(crate) value: String,
    pub(crate) comment: Option<String>,
    pub(crate) temp_id: uuid::Uuid,
}
pub(crate) struct DotEnvFile(Vec<EnvVar>);

impl DotEnvFile {
    pub(crate) fn parse_from_file(
        path: PathBuf,
        parse_comments: bool,
        verbose: bool,
    ) -> anyhow::Result<Self> {
        if verbose {
            eprintln!("Reading from file at {}", path.to_string_lossy());
        }

        let raw = fs::read_to_string(path)?;

        // Map over all lines of the file, extracting variables while ignoring / filtering out empty lines and comments
        let envs = raw
            .lines()
            .into_iter()
            .map(|line| {
                // Trim the line for easier parsing
                let trimmed_line = line.trim();

                // If the line is a comment, return None
                if trimmed_line.starts_with('#') {
                    return None;
                }

                // If the line is blank, return None
                if trimmed_line.is_empty() {
                    return None;
                }

                // Split the line into content and comment (if one exists)
                let (content, comment) = match line.split_once("#") {
                    Some((content, comment)) => (content.trim(), Some(comment.trim())),
                    None => (trimmed_line, None),
                };

                // Split the content into key and value
                match content.split_once('=') {
                    Some((key, value)) => Some(EnvVar {
                        key: key.to_string(),
                        value: value.to_string(),
                        comment: if parse_comments {
                            comment.map(ToOwned::to_owned)
                        } else {
                            None
                        },
                        temp_id: uuid::Uuid::new_v4(),
                    }),
                    None => None,
                }
            })
            .filter_map(|x| x) // Filter out any invalid line and unwrap the Option for the rest
            .collect::<Vec<EnvVar>>();

        if verbose {
            eprintln!("Found {} variables", envs.len());
        }

        Ok(Self(envs))
    }

    pub(crate) fn vars(&self) -> Vec<EnvVar> {
        self.0.clone()
    }
}
