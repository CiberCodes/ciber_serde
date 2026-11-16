//! Tuple serialization support.

use ciber_cbr::Value;
use serde::ser::SerializeTuple;

use super::super::serializer::CbrSerializer;
use crate::error::Error;

/// Serializer for tuple values.
#[derive(Debug, Default)]
pub struct CbrSerializeTuple {
  elements: Vec<Value>,
}

impl CbrSerializeTuple {
  /// Creates a tuple serializer with the expected capacity.
  #[must_use]
  pub fn new(len: usize) -> Self { Self { elements: Vec::with_capacity(len) } }
}

impl SerializeTuple for CbrSerializeTuple {
  type Error = Error;
  type Ok = Value;

  fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
  where
    T: serde::Serialize + ?Sized,
  {
    self.elements.push(value.serialize(CbrSerializer)?);

    Ok(())
  }

  fn end(self) -> Result<Self::Ok, Self::Error> {
    Ok(Value::Array(self.elements))
  }
}
