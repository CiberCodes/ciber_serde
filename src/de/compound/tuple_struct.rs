//! Tuple structure deserialization support for CBR values.
//!
//! Provides deserialization support for Rust tuple structures
//! through Serde's tuple structure deserialization interface.

use ciber_cbr::Value;
use serde::de::Visitor;

use super::tuple::deserialize_tuple;
use crate::error::{Error, Result};

/// Deserializes a CBR sequence into a Rust tuple structure.
///
/// Tuple structures contain a fixed number of positional fields.
/// Their CBR representation is therefore handled in the same way
/// as a regular tuple.
///
/// # Arguments
///
/// - `name`: The name of the tuple structure being deserialized.
/// - `elements`: The elements belonging to the tuple structure.
/// - `length`: The expected number of fields.
/// - `visitor`: The visitor responsible for constructing the requested Rust
///   value.
///
/// # Returns
///
/// Returns the value produced by the visitor.
///
/// # Errors
///
/// Returns an error if the sequence length does not match the
/// expected number of fields, or if a field cannot be deserialized.
pub(super) fn deserialize_tuple_struct<'de, V>(
  _name: &'static str,
  elements: Vec<Value>,
  length: usize,
  visitor: V,
) -> Result<V::Value>
where
  V: Visitor<'de>,
{
  deserialize_tuple(elements, length, visitor)
}

/// Deserializes a CBR array or set into a Rust tuple structure.
///
/// Arrays preserve their original element order. Sets are converted
/// into sequences using their natural iteration order.
///
/// # Arguments
///
/// - `name`: The name of the tuple structure being deserialized.
/// - `value`: The CBR value to deserialize.
/// - `length`: The expected number of fields.
/// - `visitor`: The visitor responsible for constructing the requested Rust
///   value.
///
/// # Returns
///
/// Returns the value produced by the visitor.
///
/// # Errors
///
/// Returns an error if the supplied value is neither an array nor
/// a set, if its length does not match the expected number of fields,
/// or if a field cannot be deserialized.
pub fn deserialize_tuple_struct_value<'de, V>(
  name: &'static str,
  value: Value,
  length: usize,
  visitor: V,
) -> Result<V::Value>
where
  V: Visitor<'de>,
{
  match value {
    Value::Array(elements) => {
      deserialize_tuple_struct(name, elements, length, visitor)
    }
    Value::Set(elements) => deserialize_tuple_struct(
      name,
      elements.into_iter().collect(),
      length,
      visitor,
    ),
    value => Err(Error::new(format!(
      "invalid type: {}, expected tuple struct {name}",
      super::deserializer::ValueDescription(&value),
    ))),
  }
}
