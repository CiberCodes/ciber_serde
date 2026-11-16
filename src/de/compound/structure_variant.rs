//! Structure variant deserialization support for CBR values.
//!
//! Provides deserialization support for enum variants containing
//! named fields through Serde's enum access interfaces.

use ciber_cbr::Value;
use serde::de::Visitor;

use super::structure::deserialize_structure;
use crate::error::{Error, Result};

/// Deserializes a CBR structure variant through Serde's enum interface.
///
/// The expected representation is an object containing exactly
/// one variant name associated with an object of named fields.
///
/// # Arguments
///
/// - `entries`: The entries representing the enum variant.
/// - `visitor`: The visitor responsible for constructing the requested Rust
///   enum value.
///
/// # Returns
///
/// Returns the value produced by the visitor.
///
/// # Errors
///
/// Returns an error if the representation does not contain
/// exactly one variant entry or if the visitor fails.
pub fn deserialize_structure_variant<'de, V>(
  value: Value,
  fields: &'static [&'static str],
  visitor: V,
) -> Result<V::Value>
where
  V: Visitor<'de>,
{
  match value {
    Value::Object(entries) => {
      deserialize_structure("enum structure variant", fields, entries, visitor)
    }
    value => Err(Error::new(format!(
      "invalid type: {}, expected a structure variant",
      super::deserializer::ValueDescription(&value),
    ))),
  }
}
