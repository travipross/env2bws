use anyhow::anyhow;
use clap::Parser;
use cli::Cli;
use env2bws::{DotEnvFile, ImportPayload, ProjectAssignment};
use std::{
    fs::OpenOptions,
    io::{self, Write},
};

/// Module used for handling CLI behaviour with [`clap`]
///
/// Defined in this main module so as to not expose via public API
mod cli {
    use clap::{
        builder::{styling::AnsiColor, Styles},
        Args, Parser,
    };
    use std::path::PathBuf;

    /// Styling used for help output
    pub(crate) const STYLES: Styles = Styles::styled()
        .header(AnsiColor::Yellow.on_default())
        .usage(AnsiColor::Green.on_default())
        .literal(AnsiColor::Cyan.on_default())
        .placeholder(AnsiColor::Cyan.on_default())
        .invalid(AnsiColor::Red.on_default());

    /// Parse the given .env file and output in a JSON format that is compatible with Bitwarden Secrets
    /// Manager's import feature.
    #[derive(Debug, Clone, Parser, PartialEq, Eq)]
    #[command(styles = STYLES, arg_required_else_help = true)]
    pub(crate) struct Cli {
        /// Path to the .env file to parse
        ///
        /// Note: The file must be in the format of a .env file, with each line containing a key-value
        /// pair separated by an equals sign (and followed by an optional comment). For example:
        ///
        /// SECRET_VALUE_1=12345
        /// SECRET_VALUE_2=abcde  # Optional comment
        ///
        /// The file may have any name as long as it follows this format.
        #[arg(verbatim_doc_comment)]
        pub(crate) dotenv_path: PathBuf,

        #[command(flatten)]
        pub(crate) project_assignment: ProjectAssignmentArgs,

        /// Output file path
        ///
        /// If not provided, the output will be printed to stdout.
        #[arg(short, long)]
        pub(crate) output_file: Option<PathBuf>,

        /// Interpret comment lines directly above or directly beside a variable as notes on the secret
        ///
        /// If a comment exists above the line, it takes precedence over any comment that is inline with
        /// the variable. In order for comments to be associated with a variable defined under it, there
        /// must be no whitespace between the comment line and the variable declaration line
        #[arg(short = 'c', long)]
        pub(crate) parse_comments: bool,

        /// Enable verbose output
        ///
        /// All verbose logging is written to stderr so that it doesn't interfere with the ability to
        /// pipe or redirect processed JSON output from stdout.
        #[arg(short, long)]
        pub(crate) verbose: bool,

        /// If the chosen output file already exists, force it to be overwritten
        ///
        /// Requires -o/--output-file option
        #[arg(short, long, requires = "output_file")]
        pub(crate) force_overwrite: bool,
    }

    /// An [`ArgGroup`][clap::ArgGroup] that is used to determine which project a secret should be
    /// assigned to.
    #[derive(Debug, Clone, Args, PartialEq, Eq)]
    #[group(required = false, multiple = false)]
    pub(crate) struct ProjectAssignmentArgs {
        /// Assign all parsed secrets to an existing project having the given ID.
        ///
        /// Conflicts with --new-project-name.
        #[arg(short, long)]
        pub(crate) project_id: Option<uuid::Uuid>,

        /// Define new project with the given name, to which all secrets will be assigned.
        ///
        /// Conflicts with --project-id.
        #[arg(short = 'n', long)]
        pub(crate) new_project_name: Option<String>,
    }

    #[cfg(test)]
    mod cli_tests {
        use clap::error::ErrorKind;

        use super::*;

        #[test]
        fn verify_cli() {
            use clap::CommandFactory;
            Cli::command().debug_assert();
        }

        #[test_case::test_case(&mut [".env"] => matches Ok(_); "happy path minimum")]
        #[test_case::test_case(&mut [".env", "--output-file", "out.json", "--verbose", "--parse-comments"] => matches Ok(_); "happy path no project")]
        #[test_case::test_case(&mut [".env", "--output-file", "out.json", "--new-project-name", "my-new-project", "--verbose", "--parse-comments"] => matches Ok(_); "happy path new project")]
        #[test_case::test_case(&mut [".env", "--project-id", &uuid::Uuid::new_v4().to_string(), "--output-file", "out.json", "--verbose", "--parse-comments"] => matches Ok(_); "happy path existing project")]
        #[test_case::test_case(&mut [".env", "--project-id", &uuid::Uuid::new_v4().to_string(), "--new-project-name", "my-new-project"] => matches Err(ErrorKind::ArgumentConflict); "fails when conflicting new/existing project")]
        #[test_case::test_case(&mut [] => matches Err(ErrorKind::DisplayHelpOnMissingArgumentOrSubcommand); "help on missing args")]
        #[test_case::test_case(&mut ["--help"] => matches Err(ErrorKind::DisplayHelp); "help when requested")]
        #[test_case::test_case(&mut ["-h"] => matches Err(ErrorKind::DisplayHelp); "help when requested short")]
        fn parse_args(args: &mut [&str]) -> Result<Cli, ErrorKind> {
            // Combine test case args with full command string
            let mut cmd_and_args = vec!["first-arg-is-ignored-by-parser"];
            cmd_and_args.extend_from_slice(args);

            Cli::try_parse_from(cmd_and_args).map_err(|e| e.kind())
        }
    }
}

fn main() -> anyhow::Result<()> {
    // Process CLI args
    let cli = Cli::parse();

    // Load dotenv struct from file
    let dotenv = DotEnvFile::parse_from_file(cli.dotenv_path, cli.parse_comments, cli.verbose)?;

    // Determine type of project assignment for secrets based on provided arguments
    let project_assignment = match (
        cli.project_assignment.project_id,
        cli.project_assignment.new_project_name,
    ) {
        (None, Some(name)) => ProjectAssignment::New(name),
        (Some(id), None) => ProjectAssignment::Existing(id),
        (None, _) => ProjectAssignment::None,
        _ => unreachable!(), // Should not be possible due to conflicts_with attribute on parser
    };

    // Prepare import payload in format expected by Bitwarden Secrets Manager
    let payload = ImportPayload::from_dotenv(dotenv, project_assignment);

    // Depending on whether an output path is provided, either write out JSON result, or print to stdout
    if let Some(path) = cli.output_file {
        // Ensure path has .json extension and add it if not
        let path = match path.extension() {
            Some(ext) if ext == "json" => Ok(path),
            Some(_) => Err(anyhow::anyhow!("Output file must have .json extension")),
            _ => Ok(path.with_extension("json")),
        }?;

        // Write the JSON payload to the output file
        eprintln!("Writing to file at {}", path.to_string_lossy());

        let mut file = match OpenOptions::new()
            .write(true)
            .create_new(true) // Only create if it doesn't exist
            .open(&path)
        {
            Ok(f) => f, // Success on first try
            Err(e) if e.kind() == io::ErrorKind::AlreadyExists => {
                if cli.force_overwrite {
                    eprintln!("File already exists in current location. Writing over contents because --force-overwrite was provided");
                    // Retry with overwrite if allowed
                    OpenOptions::new()
                        .write(true)
                        .truncate(true) // Overwrite existing file
                        .open(&path)?
                } else {
                    return Err(anyhow!("File already exists at the current location. Try selecting a different output file or re-running with --force-overwrite"));
                }
            }
            Err(e) => return Err(anyhow::anyhow!("Failed to create file: {}", e)),
        };

        file.write_all(serde_json::to_string_pretty(&payload)?.as_bytes())?;
    } else {
        println!("{}", serde_json::to_string_pretty(&payload)?);
    }

    Ok(())
}
