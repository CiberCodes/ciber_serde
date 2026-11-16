use ciber_cbr::Value;
use serde::ser::SerializeSeq;

use super::super::serializer::CbrSerializer;
use crate::error::Error;

/// Serializer for sequence values.
#[derive(Debug, Default)]
pub struct CbrSerializeSeq {
  elements: Vec<Value>,
}

impl CbrSerializeSeq {
  /// Creates a new sequence serializer.
  #[must_use]
  pub const fn new() -> Self { Self { elements: Vec::new() } }

  /// Reserves capacity for the expected number of elements.
  pub fn reserve(&mut self, additional: usize) {
    self.elements.reserve(additional);
  }
}

impl SerializeSeq for CbrSerializeSeq {
  type Ok = Value;
  type Error = Error;

  fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
  where
    T: serde::Serialize + ?Sized,
  {
    let serialized = value.serialize(CbrSerializer)?;

    self.elements.push(serialized);

    Ok(())
  }

  fn end(self) -> Result<Self::Ok, Self::Error> {
    Ok(Value::Array(self.elements))
  }
}
