# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Fixed
- `SerializerOperatorTranslator`: don't apply graph mapping in case of a join because that's already handled by the `JoinTranslator`. Fixes <https://gitlab.ilabt.imec.be/rml/proc/algemaploom-rs/-/issues/47>