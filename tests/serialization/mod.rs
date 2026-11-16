//! Integration tests for CBR serialization.
//!
//! This module verifies the conversion of Rust values into the
//! CBR data model through `CbrSerializer`.
//!
//! The test modules cover the serialization of:
//!
//! - Maps and key-value collections.
//! - Sequences and nested collections.
//! - Structures and nested structures.
//! - Struct variants of enumerations.
//! - Tuples and nested tuples.
//! - Tuple structs.
//! - Tuple variants of enumerations.
//!
//! These tests ensure that serialized values preserve their expected
//! structure, field ordering, and value representations.

mod map;
mod sequence;
mod structure;
mod structure_variant;
mod tuple;
mod tuple_struct;
mod tuple_variant;
