//! Tuple variant serialization support.

use std::collections::BTreeMap;

use ciber_cbr::Value;
use serde::ser::SerializeTupleVariant;

use super::super::serializer::CbrSerializer;
use crate::error::Error;

/// Serializer for tuple enum variants.
#[derive(Debug, Default)]
pub struct CbrSerializeTupleVariant {
  name: String,
  elements: Vec<Value>,
}

impl CbrSerializeTupleVariant {
  /// Creates a tuple variant serializer.
  #[must_use]
  pub fn new(name: &'static str, len: usize) -> Self {
    Self { name: name.to_owned(), elements: Vec::with_capacity(len) }
  }
}

impl SerializeTupleVariant for CbrSerializeTupleVariant {
  type Error = Error;
  type Ok = Value;

  fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
  where
    T: serde::Serialize + ?Sized,
  {
    self.elements.push(value.serialize(CbrSerializer)?);

    Ok(())
  }

  fn end(self) -> Result<Self::Ok, Self::Error> {
    let mut object = BTreeMap::new();

    object.insert(self.name, Value::Array(self.elements));

    Ok(Value::Object(object))
  }
}
