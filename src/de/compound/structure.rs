//! Structure deserialization support for CBR values.
//!
//! Provides deserialization support for Rust structures
//! represented as CBR objects.

use std::collections::BTreeMap;

use ciber_cbr::Value;
use serde::de::Visitor;

use super::map::deserialize_map;
use crate::error::Result;

/// Deserializes a CBR object into a Rust structure.
///
/// Rust structures are represented as CBR objects, where each
/// field name corresponds to a map key and each field value
/// corresponds to its associated CBR value.
///
/// The underlying map access implementation is reused to
/// deserialize structure fields, avoiding duplicated logic.
///
/// # Arguments
///
/// - `name`: The name of the structure being deserialized.
/// - `fields`: The fields expected by the structure.
/// - `entries`: The entries contained in the CBR object.
/// - `visitor`: The visitor responsible for constructing the requested Rust
///   structure.
///
/// # Returns
///
/// Returns the value produced by the visitor.
///
/// # Errors
///
/// Returns an error if the visitor or any structure field
/// cannot be deserialized.
pub(super) fn deserialize_structure<'de, V>(
  _name: &'static str,
  _fields: &'static [&'static str],
  entries: BTreeMap<String, Value>,
  visitor: V,
) -> Result<V::Value>
where
  V: Visitor<'de>,
{
  deserialize_map(entries, visitor)
}

/// Deserializes a CBR value into a Rust structure.
///
/// # Arguments
///
/// - `name`: The name of the structure being deserialized.
/// - `fields`: The fields expected by the structure.
/// - `value`: The CBR value to deserialize.
/// - `visitor`: The visitor responsible for constructing the requested Rust
///   structure.
///
/// # Returns
///
/// Returns the value produced by the visitor.
///
/// # Errors
///
/// Returns an error if the supplied value is not a CBR object,
/// or if a structure field cannot be deserialized.
pub fn deserialize_structure_value<'de, V>(
  name: &'static str,
  fields: &'static [&'static str],
  value: Value,
  visitor: V,
) -> Result<V::Value>
where
  V: Visitor<'de>,
{
  match value {
    Value::Object(entries) => {
      deserialize_structure(name, fields, entries, visitor)
    }
    value => Err(crate::error::Error::new(format!(
      "invalid type: {}, expected struct {}",
      super::deserializer::ValueDescription(&value),
      name,
    ))),
  }
}
