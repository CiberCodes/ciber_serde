# Changelog

Notable changes to `ciber_serde` are documented here.

## [Unreleased]

### Added

- Serde serialization and deserialization integration tests covering maps,
  sequences, structures, tuples, tuple structs, and enum variants.
- Examples demonstrating CBR text conversion and file read/write round trips.
- Release validation for formatting, tests, Clippy, documentation, and package
  contents in GitHub Actions.

### Changed

- Documented the public serialization and deserialization APIs and updated the
  README to describe this crate's role in the CBR ecosystem.
- Accepted nonnegative signed CBR integers when deserializing unsigned Rust
  integer types, with checked range conversion.
- Corrected package license metadata and configured the license policy to allow
  Unicode-3.0 for the transitive `unicode-ident` dependency.

### Fixed

- Rejected negative and out-of-range values when converting to unsigned types.
- Fixed formatting, documentation, and lint issues reported by the release
  checks.

## [0.1.0] - 2026-11-16

- Initial release of Serde serialization and deserialization support for CBR.
- Includes `CbrSerializer`, `CbrDeserializer`, and convenience functions for
  reading, writing, and converting CBR documents.
