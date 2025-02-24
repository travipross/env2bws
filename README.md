# env2bws

[![Crates.io Version](https://img.shields.io/crates/v/env2bws)](https://crates.io/crates/env2bws)
[![GitHub Release](https://img.shields.io/github/v/release/travipross/env2bws?label=Github%20Release)](https://github.com/travipross/env2bws/releases/latest)
[![docs.rs](https://img.shields.io/docsrs/env2bws)](https://docs.rs/env2bws/latest/env2bws/)
[![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/travipross/env2bws/rust.yml)](https://github.com/travipross/env2bws)
[![Crates.io Total Downloads](https://img.shields.io/crates/d/env2bws)](https://crates.io/crates/env2bws)
[![Crates.io Size](https://img.shields.io/crates/size/env2bws)](https://crates.io/crates/env2bws)
[![GitHub License](https://img.shields.io/github/license/travipross/env2bws)](https://github.com/travipross/env2bws/releases/latest)

A CLI tool for converting a `.env` file into a JSON format that can be imported by [Bitwarden Secrets Manager](https://bitwarden.com/help/import-secrets-data/).

**This tool does not perform any actual import operation on Bitwarden Secrets Manager.** It is just designed to help in conditioning a JSON import file from a `.env` file. Please use the web interface to import the resulting JSON file.

## Installing

### From source

Run `cargo install env2bws`

## Using the tool

### Preparing your `.env` file

This tool is capable of parsing a "dotenv" file which:

- is made primarily of a series of key-value pairs in plain text, **one per line**[^one-per-line], separated by `=`
- **optionally** may contain comments to the right of the key-value pair, preceded with `#`
- may contain blank lines, or additional comments (ignored)
- may have any filename and extension

An example `.env` file:

```bash
# Sample Environment Variables
#
# These comment lines are ignored by env2bws because they are not following a variable declaration

# Service 1 - This comment line is also ignored
SERVICE_1_API_PORT=8001  # THIS IS A SAMPLE COMMENT
SERVICE_1_WEB_PORT=8002
SERVICE_1_DATA=/path/to/data/service_1

# Service 2
SERVICE_2_API_PORT=8003
SERVICE_2_WEB_PORT=8004
SERVICE_2_DATA=/path/to/data/service_2 # Another comment
```

[^one-per-line]: Though supported in some systems, multiline values are not supported by this tool. Consider converting to a single-line string with explicit newline characters (`\n`).

### Writing output

By setting the `-o`/`--output-file` argument to a given path, a file containing "pretty" JSON will be written:

```bash
# Write output to file
env2bws .env -o secrets-to-import.json
```

Without the above option, `env2bws` will default to output to `stdout`. This also allowing for piping and redirecting:

```bash
# Print to stdout to view
env2bws .env

# Redirect to a file
env2bws .env > secrets-to-import.json

# Pipe to another program
env2bws .env | jq
```

### Parsing Comments

By supplying the `-c`/`--parse-comments` argument, `env2bws` will attempt to parse comments that follow each key-value pair in the `.env` file.

For example, with the following `.env` file:

```bash
SECRET_1=secretval  # This is a comment
```

That would be parsed as a secret with the comment being stored as a "note":

```json
{
    "projects": ...
    "secrets": [
        {
            "key": "SECRET_1",
            "value": "secretval",
            "note": "This is a comment",
            "projectIds": [],
            "id": "7a81e22c-24fd-4ea6-bf55-e7db7b3073e8"
        }
    ]
}
```

### Assigning secrets to projects

As outlined in the [BWS documentation](https://bitwarden.com/help/import-secrets-data/#condition-an-import-file), secrets may optionally be assigned to projects in one of multiple ways:

1. Assigning to a new project
2. Assigning to an existing project

By default, `env2bws` does not assign secrets to any project, and they will appear in BWS as "unassigned".

**Currently `env2bws` only allows for a single project assignment setting to apply for all secrets in the provided `.env` file.** However, Bitwarden Secrets Manager supports granular assignment of secrets to individual projects, as well as creation of multiple projects. To do this, you will need to manually edit the generated JSON file from this tool before import.

#### Assigning to a new project

In order to create a new project and assign all secrets to it, pass the `-n`/`--new-project-name` argument, and give the project a name. `env2bws` will prepare the resulting JSON in the correct format to do so.

```bash
# Assign to new project with the name "My New Project"
env2bws .env -n "My New Project"
```

#### Assigning to an existing project

Secrets can be assigned to an existing project with the `-p`/`--project-id` argument. You can obtain the project ID from the BWS web interface, or via the [`bws` CLI tool](https://bitwarden.com/help/secrets-manager-cli/).

```bash
# Assign to existing project having the id <my-project-id>
env2bws .env -p <my-project-id>
```

### CLI help output

For more help regarding using the tool, see the CLI help output:

```bash
# Print detailed help
env2bws --help

# Print shortened help
env2bws -h
```
