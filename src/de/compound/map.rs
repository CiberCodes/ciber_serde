//! Map deserialization support for CBR values.
//!
//! Provides map access for CBR objects through Serde's map
//! deserialization interface.

use std::collections::btree_map::IntoIter;

use ciber_cbr::Value;
use serde::de::{DeserializeSeed, MapAccess, Visitor};

use crate::error::Result;

/// Provides access to the entries of a CBR object.
///
/// This structure implements [`MapAccess`] and allows Serde
/// visitors to deserialize map keys and values individually.
///
/// The underlying iterator preserves the key ordering defined
/// by the object's `BTreeMap`.
#[derive(Debug)]
pub(super) struct CbrMapAccess {
  entries: IntoIter<String, Value>,
  pending_value: Option<Value>,
}

impl CbrMapAccess {
  /// Creates a new map access instance.
  ///
  /// # Arguments
  ///
  /// - `entries`: The entries to make available for deserialization.
  ///
  /// # Returns
  ///
  /// Returns a map access instance containing the supplied
  /// entries.
  #[must_use]
  pub(super) fn new(
    entries: std::collections::BTreeMap<String, Value>,
  ) -> Self {
    Self { entries: entries.into_iter(), pending_value: None }
  }
}

impl<'de> MapAccess<'de> for CbrMapAccess {
  type Error = crate::error::Error;

  /// Deserializes the next map key using the supplied seed.
  ///
  /// The corresponding value is retained until
  /// [`next_value_seed`](MapAccess::next_value_seed) is called.
  ///
  /// # Arguments
  ///
  /// - `seed`: The deserialization seed used to deserialize the next key.
  ///
  /// # Returns
  ///
  /// Returns `Some` containing the deserialized key when an
  /// entry is available, or `None` when the map is exhausted.
  ///
  /// # Errors
  ///
  /// Returns an error if the key cannot be deserialized.
  fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
  where
    K: DeserializeSeed<'de>,
  {
    match self.entries.next() {
      Some((key, value)) => {
        self.pending_value = Some(value);

        seed
          .deserialize(
            serde::de::value::StringDeserializer::<Self::Error>::new(key),
          )
          .map(Some)
      }
      None => Ok(None),
    }
  }

  /// Deserializes the value associated with the previously
  /// requested key.
  ///
  /// # Arguments
  ///
  /// - `seed`: The deserialization seed used to deserialize the value.
  ///
  /// # Returns
  ///
  /// Returns the deserialized map value.
  ///
  /// # Errors
  ///
  /// Returns an error if no key has been requested or if
  /// the value cannot be deserialized.
  fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
  where
    V: DeserializeSeed<'de>,
  {
    let value = self.pending_value.take().ok_or_else(|| {
      crate::error::Error::new(
        "value requested before its corresponding map key",
      )
    })?;

    super::deserialize_value(value, seed)
  }

  /// Returns the number of entries remaining in the map.
  ///
  /// # Returns
  ///
  /// Returns the exact number of entries that can still be
  /// consumed from the underlying iterator.
  fn size_hint(&self) -> Option<usize> { Some(self.entries.len()) }
}

/// Deserializes a CBR map using the supplied visitor.
///
/// Creates a [`CbrMapAccess`] instance and passes it to the
/// visitor through Serde's map interface.
///
/// # Arguments
///
/// - `entries`: The entries belonging to the map.
/// - `visitor`: The visitor responsible for constructing the requested Rust
///   value.
///
/// # Returns
///
/// Returns the value produced by the visitor.
///
/// # Errors
///
/// Returns an error if the visitor, a key, or a value
/// cannot be deserialized.
pub fn deserialize_map<'de, V>(
  entries: std::collections::BTreeMap<String, Value>,
  visitor: V,
) -> Result<V::Value>
where
  V: Visitor<'de>,
{
  visitor.visit_map(CbrMapAccess::new(entries))
}

/// Deserializes a CBR object using the supplied visitor.
///
/// # Arguments
///
/// - `value`: The CBR value to deserialize.
/// - `visitor`: The visitor responsible for constructing the requested Rust
///   value.
///
/// # Returns
///
/// Returns the value produced by the visitor.
///
/// # Errors
///
/// Returns an error if the supplied value is not an object,
/// or if an entry cannot be deserialized.
pub fn deserialize_map_value<'de, V>(
  value: Value,
  visitor: V,
) -> Result<V::Value>
where
  V: Visitor<'de>,
{
  match value {
    Value::Object(entries) => deserialize_map(entries, visitor),
    value => Err(crate::error::Error::new(format!(
      "invalid type: {}, expected a map",
      super::deserializer::ValueDescription(&value),
    ))),
  }
}
