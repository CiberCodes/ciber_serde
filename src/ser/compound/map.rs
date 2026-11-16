//! Map serialization support.

use std::collections::BTreeMap;

use ciber_cbr::Value;
use serde::ser::SerializeMap;

use super::super::serializer::CbrSerializer;
use crate::error::Error;

/// Serializer for map values.
#[derive(Debug, Default)]
pub struct CbrSerializeMap {
  entries: BTreeMap<String, Value>,
  pending_key: Option<String>,
}

impl CbrSerializeMap {
  /// Creates a new map serializer.
  #[must_use]
  pub fn new() -> Self { Self::default() }

  /// Converts a serialized value into a CBR object key.
  fn serialize_key<T>(key: &T) -> Result<String, Error>
  where
    T: serde::Serialize + ?Sized,
  {
    match key.serialize(CbrSerializer)? {
      Value::String(key) => Ok(key),
      _ => Err(Error::new("CBR object keys must serialize to strings")),
    }
  }
}

impl SerializeMap for CbrSerializeMap {
  type Error = Error;
  type Ok = Value;

  fn serialize_key<T>(&mut self, key: &T) -> Result<(), Self::Error>
  where
    T: serde::Serialize + ?Sized,
  {
    if self.pending_key.is_some() {
      return Err(Error::new(
        "serialize_key called before completing the previous map entry",
      ));
    }

    self.pending_key = Some(Self::serialize_key(key)?);

    Ok(())
  }

  fn serialize_value<T>(&mut self, value: &T) -> Result<(), Self::Error>
  where
    T: serde::Serialize + ?Sized,
  {
    if self.pending_key.is_none() {
      return Err(Error::new("serialize_value called before serialize_key"));
    }

    let value = value.serialize(CbrSerializer)?;

    let key = self
      .pending_key
      .take()
      .ok_or_else(|| Error::new("missing pending map key"))?;

    self.entries.insert(key, value);

    Ok(())
  }

  fn end(self) -> Result<Self::Ok, Self::Error> {
    if self.pending_key.is_some() {
      return Err(Error::new(
        "serialize_key called without a corresponding serialize_value",
      ));
    }

    Ok(Value::Object(self.entries))
  }
}
