//! Tuple struct serialization support.

use ciber_cbr::Value;
use serde::ser::SerializeTupleStruct;

use super::super::serializer::CbrSerializer;
use crate::error::Error;

/// Serializer for tuple struct values.
#[derive(Debug, Default)]
pub struct CbrSerializeTupleStruct {
  elements: Vec<Value>,
}

impl CbrSerializeTupleStruct {
  /// Creates a tuple struct serializer with the expected capacity.
  #[must_use]
  pub fn new(len: usize) -> Self { Self { elements: Vec::with_capacity(len) } }
}

impl SerializeTupleStruct for CbrSerializeTupleStruct {
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
    Ok(Value::Array(self.elements))
  }
}
