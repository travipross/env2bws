use std::path::PathBuf;

use clap::{
    builder::{styling::AnsiColor, Styles},
    Args, Parser,
};

// Set help output colors
const STYLES: Styles = Styles::styled()
    .header(AnsiColor::Yellow.on_default())
    .usage(AnsiColor::Green.on_default())
    .literal(AnsiColor::Cyan.on_default())
    .placeholder(AnsiColor::Cyan.on_default())
    .invalid(AnsiColor::Red.on_default());

/// Parse the given .env file and output in a JSON format that is compatible with Bitwarden Secrets
/// Manager's import feature.
#[derive(Debug, Clone, Parser)]
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
    pub(crate) project_assignment: ProjectAssignment,

    /// Output file path
    ///
    /// If not provided, the output will be printed to stdout
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
    #[arg(short, long)]
    pub(crate) verbose: bool,
}

#[derive(Debug, Clone, Args)]
#[group(required = false, multiple = false)]
pub(crate) struct ProjectAssignment {
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
