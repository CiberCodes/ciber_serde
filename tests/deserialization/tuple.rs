//! Tests for CBR tuple deserialization.

use ciber_cbr::Value;
use ciber_serde::CbrDeserializer;
use serde::Deserialize;

/// Deserializes a tuple containing primitive values.
#[test]
fn deserializes_tuples() -> Result<(), Box<dyn std::error::Error>> {
  let value = Value::Array(vec![
    Value::I64(42),
    Value::String("Cirious".to_owned()),
    Value::Bool(true),
  ]);

  let tuple = <(i32, String, bool)>::deserialize(CbrDeserializer::new(value))?;

  assert_eq!(tuple, (42, "Cirious".to_owned(), true));

  Ok(())
}

/// Deserializes a one-element tuple.
#[test]
fn deserializes_single_element_tuples() -> Result<(), Box<dyn std::error::Error>>
{
  let value = Value::Array(vec![Value::I64(42)]);
  let tuple = <(i32,)>::deserialize(CbrDeserializer::new(value))?;

  assert_eq!(tuple, (42,));

  Ok(())
}

/// Deserializes nested tuples.
#[test]
fn deserializes_nested_tuples() -> Result<(), Box<dyn std::error::Error>> {
  let value = Value::Array(vec![
    Value::Array(vec![Value::I64(10), Value::I64(20)]),
    Value::Array(vec![Value::I64(30), Value::I64(40)]),
  ]);
  let tuple =
    <((i32, i32), (i32, i32))>::deserialize(CbrDeserializer::new(value))?;

  assert_eq!(tuple, ((10, 20), (30, 40)));

  Ok(())
}

/// Deserializes a tuple containing optional values.
#[test]
fn deserializes_tuples_with_optional_values()
-> Result<(), Box<dyn std::error::Error>> {
  let value = Value::Array(vec![Value::I64(42), Value::Null]);
  let tuple =
    <(Option<i32>, Option<i32>)>::deserialize(CbrDeserializer::new(value))?;

  assert_eq!(tuple, (Some(42), None));

  Ok(())
}

/// Checks unsigned integer conversion and rejects negative or out-of-range values.
#[test]
fn checks_unsigned_integer_ranges() {
  assert_eq!(
    u8::deserialize(CbrDeserializer::new(Value::I64(0))).ok(),
    Some(0),
  );
  assert_eq!(
    u8::deserialize(CbrDeserializer::new(Value::I64(255))).ok(),
    Some(255),
  );
  assert!(u8::deserialize(CbrDeserializer::new(Value::I64(256))).is_err());
  assert!(u8::deserialize(CbrDeserializer::new(Value::I64(-1))).is_err());
  assert!(usize::deserialize(CbrDeserializer::new(Value::I128(-1))).is_err());
}
