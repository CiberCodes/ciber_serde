//! Tests for CBR tuple variant serialization.

use ciber_cbr::Value;
use ciber_serde::CbrSerializer;
use serde::Serialize;

/// Represents events with positional fields.
#[derive(Serialize)]

enum UserEvent {
  Created(u64, String),
  Deleted(u64),
  StatusChanged(bool),
}

/// Represents a tuple struct used in nested variant tests.
#[derive(Serialize)]

struct Coordinates(i32, i32);

/// Represents events containing nested tuple structs.
#[derive(Serialize)]

enum LocationEvent {
  CoordinatesUpdated(u64, Coordinates),
}

/// Creates the expected CBR representation of an enum variant.
fn expected_variant(name: &str, value: Value) -> Value {
  Value::Object(std::collections::BTreeMap::from([(name.to_owned(), value)]))
}

/// Serializes a tuple variant containing multiple fields.
#[test]
fn serializes_tuple_variants() -> Result<(), Box<dyn std::error::Error>> {
  let value = UserEvent::Created(42, "Cirious".to_owned());

  assert_eq!(
    value.serialize(CbrSerializer)?,
    expected_variant(
      "Created",
      Value::Array(vec![Value::U64(42), Value::String("Cirious".to_owned()),]),
    ),
  );

  Ok(())
}

/// Serializes a tuple variant containing a single field.
#[test]
fn serializes_single_field_tuple_variants()
-> Result<(), Box<dyn std::error::Error>> {
  let value = UserEvent::Deleted(42);

  assert_eq!(
    value.serialize(CbrSerializer)?,
    expected_variant("Deleted", Value::U64(42)),
  );

  Ok(())
}

/// Serializes a tuple variant containing a boolean field.
#[test]
fn serializes_tuple_variants_with_primitive_fields()
-> Result<(), Box<dyn std::error::Error>> {
  let value = UserEvent::StatusChanged(true);

  assert_eq!(
    value.serialize(CbrSerializer)?,
    expected_variant("StatusChanged", Value::Bool(true)),
  );

  Ok(())
}

/// Serializes a tuple variant containing a nested tuple struct.
#[test]
fn serializes_nested_tuple_variants() -> Result<(), Box<dyn std::error::Error>>
{
  let value = LocationEvent::CoordinatesUpdated(42, Coordinates(10, 20));

  assert_eq!(
    value.serialize(CbrSerializer)?,
    expected_variant(
      "CoordinatesUpdated",
      Value::Array(vec![
        Value::U64(42),
        Value::Array(vec![Value::I64(10), Value::I64(20),]),
      ],),
    )
  );

  Ok(())
}
