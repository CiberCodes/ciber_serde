//! Tuple variant deserialization support for CBR values.
//!
//! Provides deserialization support for enum variants containing
//! positional fields through Serde's enum access interfaces.

use ciber_cbr::Value;
use serde::de::Visitor;

use super::tuple::deserialize_tuple;
use crate::error::{Error, Result};

/// Deserializes a CBR tuple variant through Serde's enum interface.
///
/// The expected representation is an object containing exactly
/// one variant name associated with an array or set of positional
/// values.
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
pub fn deserialize_tuple_variant<'de, V>(
  value: Value,
  length: usize,
  visitor: V,
) -> Result<V::Value>
where
  V: Visitor<'de>,
{
  match value {
    Value::Array(elements) => deserialize_tuple(elements, length, visitor),
    Value::Set(elements) => {
      deserialize_tuple(elements.into_iter().collect(), length, visitor)
    }
    value => Err(Error::new(format!(
      "invalid type: {}, expected a tuple variant",
      super::deserializer::ValueDescription(&value),
    ))),
  }
}
