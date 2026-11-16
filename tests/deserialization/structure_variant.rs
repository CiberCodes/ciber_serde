//! Tests for CBR struct variant deserialization.

use std::collections::BTreeMap;

use ciber_cbr::Value;
use ciber_serde::CbrDeserializer;
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
enum UserEvent {
  Created { id: u64, name: String },
  Deleted { id: u64 },
  Updated { id: u64, active: bool },
}

/// Deserializes a struct variant with several fields.
#[test]
fn deserializes_struct_variants() -> Result<(), Box<dyn std::error::Error>> {
  let value = Value::Object(BTreeMap::from([(
    "Created".to_owned(),
    Value::Object(BTreeMap::from([
      ("id".to_owned(), Value::U64(42)),
      ("name".to_owned(), Value::String("Cirious".to_owned())),
    ])),
  )]));

  let event = UserEvent::deserialize(CbrDeserializer::new(value))?;

  assert_eq!(event, UserEvent::Created { id: 42, name: "Cirious".to_owned() });

  Ok(())
}

/// Deserializes a struct variant containing one field.
#[test]
fn deserializes_single_field_variants() -> Result<(), Box<dyn std::error::Error>>
{
  let value = Value::Object(BTreeMap::from([(
    "Deleted".to_owned(),
    Value::Object(BTreeMap::from([("id".to_owned(), Value::U64(42))])),
  )]));

  let event = UserEvent::deserialize(CbrDeserializer::new(value))?;

  assert_eq!(event, UserEvent::Deleted { id: 42 });

  Ok(())
}

/// Deserializes a struct variant containing fields of different types.
#[test]
fn deserializes_variants_with_mixed_fields()
-> Result<(), Box<dyn std::error::Error>> {
  let value = Value::Object(BTreeMap::from([(
    "Updated".to_owned(),
    Value::Object(BTreeMap::from([
      ("id".to_owned(), Value::U64(42)),
      ("active".to_owned(), Value::Bool(true)),
    ])),
  )]));

  let event = UserEvent::deserialize(CbrDeserializer::new(value))?;

  assert_eq!(event, UserEvent::Updated { id: 42, active: true });

  Ok(())
}
