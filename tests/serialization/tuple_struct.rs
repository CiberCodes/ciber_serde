//! Tests for CBR tuple struct serialization.

use ciber_cbr::Value;
use ciber_serde::CbrSerializer;
use serde::Serialize;

/// Represents a tuple struct with multiple fields.
#[derive(Serialize)]

struct User(u64, String, bool);

/// Represents a tuple struct with a single field.
#[derive(Serialize)]

struct Identifier(u64);

/// Represents an empty tuple struct.
#[derive(Serialize)]

struct Empty;

/// Represents a tuple struct containing optional values.
#[derive(Serialize)]

struct OptionalData(Option<i32>, Option<String>);

/// Represents a tuple struct containing another tuple struct.
#[derive(Serialize)]

struct NestedData(Identifier, String);

/// Serializes a tuple struct containing multiple fields.
#[test]
fn serializes_tuple_structs() -> Result<(), Box<dyn std::error::Error>> {
  let value = User(42, "Cirious".to_owned(), true);

  assert_eq!(
    value.serialize(CbrSerializer)?,
    Value::Array(vec![
      Value::U64(42),
      Value::String("Cirious".to_owned()),
      Value::Bool(true),
    ]),
  );

  Ok(())
}

/// Serializes a tuple struct containing a single field.
#[test]
fn serializes_single_field_tuple_structs()
-> Result<(), Box<dyn std::error::Error>> {
  let value = Identifier(42);

  assert_eq!(value.serialize(CbrSerializer)?, Value::U64(42),);

  Ok(())
}

/// Serializes an empty tuple struct.
#[test]
fn serializes_empty_tuple_structs() -> Result<(), Box<dyn std::error::Error>> {
  let value = Empty;

  assert_eq!(value.serialize(CbrSerializer)?, Value::Null,);

  Ok(())
}

/// Serializes a tuple struct containing optional values.
#[test]
fn serializes_tuple_structs_with_optional_values()
-> Result<(), Box<dyn std::error::Error>> {
  let value = OptionalData(Some(42), None);

  assert_eq!(
    value.serialize(CbrSerializer)?,
    Value::Array(vec![Value::I64(42), Value::Null,]),
  );

  Ok(())
}

/// Serializes a tuple struct containing another tuple struct.
#[test]
fn serializes_nested_tuple_structs() -> Result<(), Box<dyn std::error::Error>> {
  let value = NestedData(Identifier(42), "Cirious".to_owned());

  assert_eq!(
    value.serialize(CbrSerializer)?,
    Value::Array(vec![Value::U64(42), Value::String("Cirious".to_owned()),]),
  );

  Ok(())
}
