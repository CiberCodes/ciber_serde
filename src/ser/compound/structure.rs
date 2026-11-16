//! Struct serialization support.

use std::collections::BTreeMap;

use ciber_cbr::Value;
use serde::ser::SerializeStruct;

use super::super::serializer::CbrSerializer;
use crate::error::Error;

/// Serializer for struct values.
#[derive(Debug, Default)]
pub struct CbrSerializeStruct {
  fields: BTreeMap<String, Value>,
}

impl CbrSerializeStruct {
  /// Creates a struct serializer.
  #[must_use]
  pub const fn new(_name: &'static str, _len: usize) -> Self {
    Self { fields: BTreeMap::new() }
  }
}

impl SerializeStruct for CbrSerializeStruct {
  type Error = Error;
  type Ok = Value;

  fn serialize_field<T>(
    &mut self,
    key: &'static str,
    value: &T,
  ) -> Result<(), Self::Error>
  where
    T: serde::Serialize + ?Sized,
  {
    let value = value.serialize(CbrSerializer)?;

    self.fields.insert(key.to_owned(), value);

    Ok(())
  }

  fn end(self) -> Result<Self::Ok, Self::Error> {
    Ok(Value::Object(self.fields))
  }
}
