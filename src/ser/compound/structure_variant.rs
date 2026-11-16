//! Struct variant serialization support.

use std::collections::BTreeMap;

use ciber_cbr::Value;
use serde::ser::SerializeStructVariant;

use super::super::serializer::CbrSerializer;
use crate::error::Error;

/// Serializer for struct enum variants.
#[derive(Debug, Default)]
pub struct CbrSerializeStructVariant {
  name: String,
  fields: BTreeMap<String, Value>,
}

impl CbrSerializeStructVariant {
  /// Creates a struct variant serializer.
  #[must_use]
  pub fn new(name: &'static str, _len: usize) -> Self {
    Self { name: name.to_owned(), fields: BTreeMap::new() }
  }
}

impl SerializeStructVariant for CbrSerializeStructVariant {
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
    let mut object = BTreeMap::new();

    object.insert(self.name, Value::Object(self.fields));

    Ok(Value::Object(object))
  }
}
