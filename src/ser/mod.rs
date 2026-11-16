//! Serialization support for CBR values.

pub mod compound;

/// Provides the serializer implementation used to convert
/// Rust values into the CBR data model.
pub mod serializer;

pub use serializer::CbrSerializer;
