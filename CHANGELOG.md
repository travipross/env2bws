# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.2](https://github.com/travipross/env2bws/compare/v0.2.1...v0.2.2) - 2025-03-02

### Added

- break out functionality for parsing individual env var

### Fixed

- re-export EnvVar struct from dotenv crate to preserve API compatibility
- break out functionality to parse from str

## [0.2.1](https://github.com/travipross/env2bws/compare/v0.2.0...v0.2.1) - 2025-02-24

### Other

- enable build artifact publishing in CI

## [0.2.0](https://github.com/travipross/env2bws/compare/v0.1.2...v0.2.0) - 2025-02-24

### Added

- [**breaking**] remove cli module from public API of crate
- add option to prevent overwriting existing output files unless overridden

### Fixed

- give better error message when input file can't be found

### Other

- publish build artifacts as release assets
- fix various typos in README and module documentation
- report typos in Github Actions

## [0.1.2](https://github.com/travipross/env2bws/compare/v0.1.1...v0.1.2) - 2025-02-23

### Added

- expose as lib

### Fixed

- set keys to camelCase during import payload serialization
- address lint errors from clippy

### Other

- populate missing docstrings throughout crate
- update README with more detailed usage information
- add test to assert validity of sample input and output in project
- fix secret name in release-plz workflows
- deny warnings in rust jobs
- add check and lint jobs
- pull pipeline secrets from BWS instead of storing in Github directly

## [0.1.1](https://github.com/travipross/env2bws/compare/v0.1.0...v0.1.1) - 2025-02-23

### Added

- Initial Release

### Fixed

- Use color output for help

### Other

- add test coverage for all modules
- Configure Github Actions for building and testing crate
- Configure Github Actions for release-plz

## [0.1.0](https://github.com/travipross/env2bws/releases/tag/v0.1.0) - 2025-02-23

### Added

- Initial Release

### Other

- Configure Github Actions for building and testing crate
- Configure Github Actions for release-plz
