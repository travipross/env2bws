//! Structured representation of `.env` files
use std::{fs, ops::Deref, path::PathBuf};

use anyhow::anyhow;

/// Represents a single environment variable with an optional comment
#[derive(Debug, Clone)]
#[cfg_attr(test, derive(fake::Dummy))]
pub struct EnvVar {
    pub key: String,
    pub value: String,
    pub comment: Option<String>,
    pub temp_id: uuid::Uuid,
}

/// Represents a file's worth of environment variables
#[derive(Debug, Clone)]
#[cfg_attr(test, derive(fake::Dummy))]
pub struct DotEnvFile(Vec<EnvVar>);

impl DotEnvFile {
    /// Parses variables from a given filepath pointing at a valid `.env` file.
    ///
    /// # Errors
    ///
    /// Will return error if file cannot be read (corrupt, not text, not found, etc)
    pub fn parse_from_file(
        path: PathBuf,
        parse_comments: bool,
        verbose: bool,
    ) -> anyhow::Result<Self> {
        if verbose {
            eprintln!("Reading from file at {}", path.to_string_lossy());
        }

        let raw = fs::read_to_string(&path).map_err(|e| {
            anyhow!(
                "Failed to load file at {path}: {e}",
                path = path.to_string_lossy()
            )
        })?;

        // Map over all lines of the file, extracting variables while ignoring / filtering out empty lines and comments
        let envs = raw
            .lines()
            .filter_map(|line| {
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

                // Split the content into key and value, then construct `EnvVar`
                content.split_once('=').map(|(key, value)| EnvVar {
                    key: key.to_string(),
                    value: value.to_string(),
                    comment: if parse_comments {
                        comment.map(ToOwned::to_owned)
                    } else {
                        None
                    },
                    temp_id: uuid::Uuid::new_v4(),
                })
            }) // Filter out any invalid line and unwrap the Option for the rest
            .collect::<Vec<EnvVar>>();

        if verbose {
            eprintln!("Found {} variables", envs.len());
        }

        Ok(Self(envs))
    }
}

/// Allows [`DotEnvFile`] to be iterated over like a [`Vec<EnvVar>`]
impl Deref for DotEnvFile {
    type Target = Vec<EnvVar>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod env_parsing_tests {
    use std::io::Write;

    use tempfile::NamedTempFile;

    use super::*;

    const FILE_WITH_COMMENTS: &str = r#"#This is a comment at the top of the file
ENV_1="env 1" # Comment 1
# Comments above variables should be ignored
ENV_2="env 2"


# Whitespace is ignored
ENV_3=env3 # No quotes
ENV_4=env 4 # With spaces"#;

    const FILE_WITHOUT_COMMENTS: &str = r#"ENV_1="env 1"
ENV_2="env 2"
ENV_3=env3
ENV_4=env 4"#;

    #[test_case::test_matrix(
        [FILE_WITHOUT_COMMENTS, FILE_WITH_COMMENTS],
        [true, false],
        false
    )]
    fn can_parse_happy_path(input: &str, parse_comments: bool, verbose: bool) {
        let mut tmp_file = NamedTempFile::new().expect("could not create temp file");
        tmp_file
            .write_all(input.as_bytes())
            .expect("could not write to temp file");

        let res = DotEnvFile::parse_from_file(tmp_file.path().to_owned(), parse_comments, verbose);
        assert!(res.is_ok(), "{res:?}");
    }

    #[test]
    fn parses_comments_if_present() {
        let mut tmp_file = NamedTempFile::new().expect("could not create temp file");
        tmp_file
            .write_all(FILE_WITH_COMMENTS.as_bytes())
            .expect("could not write to temp file");

        let parsed = DotEnvFile::parse_from_file(tmp_file.path().to_owned(), true, false)
            .expect("failed to parse file");

        assert_eq!(parsed.len(), 4);
        if let Some(env_var) = parsed.get(0) {
            assert_eq!(env_var.comment, Some("Comment 1".to_owned()));
            assert_eq!(env_var.key, "ENV_1".to_owned());
            assert_eq!(env_var.value, "\"env 1\"".to_owned());
        }
        if let Some(env_var) = parsed.get(1) {
            assert_eq!(env_var.comment, None);
            assert_eq!(env_var.key, "ENV_2".to_owned());
            assert_eq!(env_var.value, "\"env 2\"".to_owned());
        }
        if let Some(env_var) = parsed.get(2) {
            assert_eq!(env_var.comment, Some("No quotes".to_owned()));
            assert_eq!(env_var.key, "ENV_3".to_owned());
            assert_eq!(env_var.value, "env3".to_owned());
        }
        if let Some(env_var) = parsed.get(3) {
            assert_eq!(env_var.comment, Some("With spaces".to_owned()));
            assert_eq!(env_var.key, "ENV_4".to_owned());
            assert_eq!(env_var.value, "env 4".to_owned());
        }
    }

    #[test]
    fn ignores_comments_if_not_enabled() {
        let mut tmp_file = NamedTempFile::new().expect("could not create temp file");
        tmp_file
            .write_all(FILE_WITH_COMMENTS.as_bytes())
            .expect("could not write to temp file");

        let parsed = DotEnvFile::parse_from_file(tmp_file.path().to_owned(), false, false)
            .expect("failed to parse file");

        assert_eq!(parsed.len(), 4);
        parsed
            .iter()
            .for_each(|env_var| assert_eq!(env_var.comment, None));
    }
}
