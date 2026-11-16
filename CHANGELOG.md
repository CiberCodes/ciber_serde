# Changelog

Notable changes to `ciber_serde` are documented here.

## [0.1.0] - 2026-11-16

### Added

- Serde serialization and deserialization integration tests covering maps,
  sequences, structures, tuples, tuple structs, and enum variants.
- Examples demonstrating CBR text conversion and file read/write round trips.

### Changed

- Documented the public serialization and deserialization APIs and updated the
  README to describe this crate's role in the CBR ecosystem.
- Accepted nonnegative signed CBR integers when deserializing unsigned Rust
  integer types, with checked range conversion.

### Fixed

- Rejected negative and out-of-range values when converting to unsigned types.
- Corrected the package license metadata and allowed the `Unicode-3.0` license
  used by the transitive `unicode-ident` dependency.
