//! Tuple deserialization support for CBR values.
//!
//! Provides deserialization support for fixed-length tuples
//! through Serde's tuple deserialization interface.

use ciber_cbr::Value;
use serde::de::Visitor;

use super::sequence::deserialize_sequence;
use crate::error::{Error, Result};

/// Deserializes a CBR sequence into a Rust tuple.
///
/// Tuples have a fixed number of elements. The supplied sequence
/// must therefore contain exactly the number of elements expected
/// by the target tuple type.
///
/// # Arguments
///
/// - `elements`: The elements belonging to the tuple.
/// - `length`: The expected number of tuple elements.
/// - `visitor`: The visitor responsible for constructing the requested Rust
///   tuple.
///
/// # Returns
///
/// Returns the value produced by the visitor.
///
/// # Errors
///
/// Returns an error if the sequence length does not match the
/// expected tuple length, or if an element cannot be deserialized.
pub(super) fn deserialize_tuple<'de, V>(
  elements: Vec<Value>,
  length: usize,
  visitor: V,
) -> Result<V::Value>
where
  V: Visitor<'de>,
{
  if elements.len() != length {
    return Err(Error::new(format!(
      "invalid length {}, expected a tuple of length {length}",
      elements.len(),
    )));
  }

  deserialize_sequence(elements, visitor)
}

/// Deserializes a CBR array or set into a Rust tuple.
///
/// Arrays preserve their original element order. Sets are converted
/// into sequences using their natural iteration order.
///
/// # Arguments
///
/// - `value`: The CBR value to deserialize.
/// - `length`: The expected number of tuple elements.
/// - `visitor`: The visitor responsible for constructing the requested Rust
///   tuple.
///
/// # Returns
///
/// Returns the value produced by the visitor.
///
/// # Errors
///
/// Returns an error if the supplied value is neither an array nor
/// a set, if its length does not match the expected tuple length,
/// or if an element cannot be deserialized.
pub fn deserialize_tuple_value<'de, V>(
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
      "invalid type: {}, expected a tuple",
      super::deserializer::ValueDescription(&value),
    ))),
  }
}
