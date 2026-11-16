//! Tests for CBR tuple struct deserialization.

use ciber_cbr::Value;
use ciber_serde::CbrDeserializer;
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
struct User(u64, String, bool);

#[derive(Debug, Deserialize, PartialEq)]
struct Identifier(u64);

#[derive(Debug, Deserialize, PartialEq)]
struct OptionalData(Option<i32>, Option<String>);

#[derive(Debug, Deserialize, PartialEq)]
struct NestedData(Identifier, String);

/// Deserializes a tuple struct with multiple fields.
#[test]
fn deserializes_tuple_structs() -> Result<(), Box<dyn std::error::Error>> {
  let value = Value::Array(vec![
    Value::U64(42),
    Value::String("Cirious".to_owned()),
    Value::Bool(true),
  ]);
  let user = User::deserialize(CbrDeserializer::new(value))?;

  assert_eq!(user, User(42, "Cirious".to_owned(), true));

  Ok(())
}

/// Deserializes a tuple struct with a single field.
#[test]
fn deserializes_single_field_tuple_structs()
-> Result<(), Box<dyn std::error::Error>> {
  let identifier =
    Identifier::deserialize(CbrDeserializer::new(Value::U64(42)))?;

  assert_eq!(identifier, Identifier(42));

  Ok(())
}

/// Deserializes a tuple struct containing optional values.
#[test]
fn deserializes_tuple_structs_with_optional_values()
-> Result<(), Box<dyn std::error::Error>> {
  let value = Value::Array(vec![Value::I64(42), Value::Null]);
  let data = OptionalData::deserialize(CbrDeserializer::new(value))?;

  assert_eq!(data, OptionalData(Some(42), None));

  Ok(())
}

/// Deserializes a tuple struct containing another tuple struct.
#[test]
fn deserializes_nested_tuple_structs() -> Result<(), Box<dyn std::error::Error>>
{
  let value =
    Value::Array(vec![Value::U64(42), Value::String("Cirious".to_owned())]);
  let data = NestedData::deserialize(CbrDeserializer::new(value))?;

  assert_eq!(data, NestedData(Identifier(42), "Cirious".to_owned()));

  Ok(())
}
