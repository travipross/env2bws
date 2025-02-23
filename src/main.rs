use clap::Parser;
use env2bws::{Cli, DotEnvFile, ImportPayload, ProjectAssignment};

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
        std::fs::write(path, serde_json::to_string_pretty(&payload)?)?;
    } else {
        println!("{}", serde_json::to_string_pretty(&payload)?);
    }

    Ok(())
}
