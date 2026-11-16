//! Compound deserialization support for CBR values.
//!
//! Provides specialized implementations for deserializing
//! compound data structures through Serde.

pub(crate) mod map;
pub(crate) mod sequence;
pub(crate) mod structure;
pub(crate) mod structure_variant;
pub(crate) mod tuple;
pub(crate) mod tuple_struct;
pub(crate) mod tuple_variant;

pub(super) use super::{access::deserialize_value, deserializer};
