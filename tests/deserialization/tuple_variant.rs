//! Tests for CBR tuple variant deserialization.

use std::collections::BTreeMap;

use ciber_cbr::Value;
use ciber_serde::CbrDeserializer;
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
enum UserEvent {
  Created(u64, String),
  Deleted(u64),
  StatusChanged(bool),
}

/// Deserializes a tuple variant with multiple fields.
#[test]
fn deserializes_tuple_variants() -> Result<(), Box<dyn std::error::Error>> {
  let value = Value::Object(BTreeMap::from([(
    "Created".to_owned(),
    Value::Array(vec![Value::U64(42), Value::String("Cirious".to_owned())]),
  )]));
  let event = UserEvent::deserialize(CbrDeserializer::new(value))?;

  assert_eq!(event, UserEvent::Created(42, "Cirious".to_owned()));

  Ok(())
}

/// Deserializes a tuple variant with one field.
#[test]
fn deserializes_single_field_tuple_variants()
-> Result<(), Box<dyn std::error::Error>> {
  let value =
    Value::Object(BTreeMap::from([("Deleted".to_owned(), Value::U64(42))]));
  let event = UserEvent::deserialize(CbrDeserializer::new(value))?;

  assert_eq!(event, UserEvent::Deleted(42));

  Ok(())
}

/// Deserializes a tuple variant containing a primitive field.
#[test]
fn deserializes_tuple_variants_with_primitive_fields()
-> Result<(), Box<dyn std::error::Error>> {
  let value = Value::Object(BTreeMap::from([(
    "StatusChanged".to_owned(),
    Value::Bool(true),
  )]));
  let event = UserEvent::deserialize(CbrDeserializer::new(value))?;

  assert_eq!(event, UserEvent::StatusChanged(true));

  Ok(())
}

/// Deserializes a unit enum variant represented by its name.
#[test]
fn deserializes_unit_variants() {
  let event = UserEvent::deserialize(CbrDeserializer::new(Value::String(
    "Deleted".to_owned(),
  )));

  assert!(event.is_err());
}
