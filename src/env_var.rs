//! Representation of individual environment variables

/// Represents a single environment variable with an optional comment
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(test, derive(fake::Dummy))]
pub struct EnvVar {
    pub key: String,
    pub value: String,
    pub comment: Option<String>,
    pub temp_id: uuid::Uuid,
}

impl EnvVar {
    /// Attempt to parse environment variable from a single line of text.
    ///
    /// Returns None if variable cannot be parsed
    pub fn parse_from_str(s: &str, parse_comments: bool) -> Option<Self> {
        // Trim the line for easier parsing
        let trimmed_line = s.trim();

        // If the line is a comment, return None
        if trimmed_line.starts_with('#') {
            return None;
        }

        // If the line is blank, return None
        if trimmed_line.is_empty() {
            return None;
        }

        // Split the line into content and comment (if one exists)
        let (content, comment) = match trimmed_line.split_once("#") {
            Some((content, comment)) => (content.trim(), Some(comment.trim())),
            None => (trimmed_line, None),
        };

        // Split the content into key and value, then construct `EnvVar`
        content.split_once('=').map(|(key, value)| EnvVar {
            key: key.trim().to_string(),
            value: value.trim().to_string(),
            comment: if parse_comments {
                comment.map(|c| c.trim().to_owned())
            } else {
                None
            },
            temp_id: uuid::Uuid::new_v4(),
        })
    }
}

#[cfg(test)]
mod env_parsing_tests {

    use super::*;

    #[test_case::test_case("# comment", false => None; "ignores lines that look like a comment when comment parsing is disabled")]
    #[test_case::test_case("# comment", true => None; "ignores lines that look like a comment when comment parsing is enabled")]
    #[test_case::test_case("invalid string", false => None; "ignores lines with no equals sign when comment parsing is disabled")]
    #[test_case::test_case("invalid string", true => None; "ignores lines with no equals sign when comment parsing is enabled")]
    #[test_case::test_case("KEY=VALUE", false =>  matches Some(EnvVar{key, value, comment, ..}) if key=="KEY".to_owned() && value=="VALUE".to_owned() && comment==None; "parses key value pairs with no comments")]
    #[test_case::test_case("KEY=VALUE", true =>  matches Some(EnvVar{key, value, comment, ..}) if key=="KEY".to_owned() && value=="VALUE".to_owned() && comment==None; "parses key value pairs with no comments when comment parsing enabled")]
    #[test_case::test_case("KEY=VALUE # Comment", false =>  matches Some(EnvVar{key, value, comment, ..}) if key=="KEY".to_owned() && value=="VALUE".to_owned() && comment==None; "ignores comments when disabled")]
    #[test_case::test_case("KEY=VALUE # Comment", true =>  matches Some(EnvVar{key, value, comment, ..}) if key=="KEY".to_owned() && value=="VALUE".to_owned() && comment==Some("Comment".to_owned()); "parses comments when enabled")]
    #[test_case::test_case("    KEY            =   VALUE  #              New Comment     ", true =>  matches Some(EnvVar{key, value, comment, ..}) if key=="KEY".to_owned() && value=="VALUE".to_owned() && comment==Some("New Comment".to_owned()); "trims whitespace in all segments")]
    fn parse_test(s: &str, parse_comments: bool) -> Option<EnvVar> {
        let parsed = EnvVar::parse_from_str(s, parse_comments);
        parsed
    }
}
