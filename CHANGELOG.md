# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

### Changed
- The `reference` and `constant` properties in a source's Field is replaced by a single `expression` property, whose value is a function similar to the function in the Extend operator.
- Removed unused crate `weaver`

### Added
- CI: check if CHANGELOG.md is updated when committing to a new branch.

## [0.6.9] - 2026-07-15

### Added
- Regression tests for the RML-IO, RML-CC, RML-FNML and RML-STAR specifications.

### Fixed
- Base IRI extraction from an `@base` statement: the line is now trimmed before parsing, so the base IRI is extracted correctly on CRLF line endings (previously a trailing `" ."` could leak into it).

## [0.6.8] - 2026-07-07

### Fixed
- `bump-version.sh` script to include `Cargo.lock`

## [0.6.7] - 2026-07-07

### Fixed
- RML: Self-joins are detected and removed when logical sources are "effectively equal".
- RML template parsing.
- RML: Don't require an expression map if the term type is `rml:BlankNode`.

### Changed
- Refactored crate `operator` to be more modular and easier to maintain.

## [0.6.6] - 2026-03-18

### Fixed
- `SerializerOperatorTranslator`: don't apply graph mapping in case of a join because that's already handled by the `JoinTranslator`. Fixes <https://gitlab.ilabt.imec.be/rml/proc/algemaploom-rs/-/issues/47>
- Use official rust Docker image in GitLab CI

[0.6.9]: https://github.com/RMLio/mappingloom-rs/compare/v0.6.8...v0.6.9
[0.6.8]: https://github.com/RMLio/mappingloom-rs/compare/v0.6.7...v0.6.8
[0.6.7]: https://github.com/RMLio/mappingloom-rs/compare/v0.6.6...v0.6.7
[0.6.6]: https://github.com/RMLio/mappingloom-rs/compare/v0.6.6...v0.6.5
