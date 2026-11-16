//! Tests for CBR sequence deserialization.

use ciber_cbr::Value;
use ciber_serde::CbrDeserializer;
use serde::Deserialize;

/// Deserializes a sequence of primitive values.
#[test]
fn deserializes_sequences() -> Result<(), Box<dyn std::error::Error>> {
  let value = Value::Array(vec![Value::I64(1), Value::I64(2), Value::I64(3)]);

  let values = Vec::<i32>::deserialize(CbrDeserializer::new(value))?;

  assert_eq!(values, vec![1, 2, 3]);

  Ok(())
}

/// Deserializes an empty sequence.
#[test]
fn deserializes_empty_sequences() -> Result<(), Box<dyn std::error::Error>> {
  let values =
    Vec::<i32>::deserialize(CbrDeserializer::new(Value::Array(Vec::new())))?;

  assert_eq!(values, []);
  Ok(())
}

/// Deserializes nested sequences.
#[test]
fn deserializes_nested_sequences() -> Result<(), Box<dyn std::error::Error>> {
  let value = Value::Array(vec![
    Value::Array(vec![Value::I64(1), Value::I64(2)]),
    Value::Array(vec![Value::I64(3), Value::I64(4)]),
  ]);

  let values = Vec::<Vec<i32>>::deserialize(CbrDeserializer::new(value))?;

  assert_eq!(values, vec![vec![1, 2], vec![3, 4]]);

  Ok(())
}

/// Deserializes a sequence containing optional values.
#[test]
fn deserializes_sequences_with_optional_values()
-> Result<(), Box<dyn std::error::Error>> {
  let value = Value::Array(vec![Value::I64(10), Value::Null, Value::I64(30)]);

  let values = Vec::<Option<i32>>::deserialize(CbrDeserializer::new(value))?;

  assert_eq!(values, vec![Some(10), None, Some(30)]);

  Ok(())
}

/// Deserializes a sequence represented by a CBR set.
#[test]
fn deserializes_sets_as_sequences() -> Result<(), Box<dyn std::error::Error>> {
  let value = Value::Set(
    [Value::String("cbr".to_owned()), Value::String("rust".to_owned())]
      .into_iter()
      .collect(),
  );

  let values = Vec::<String>::deserialize(CbrDeserializer::new(value))?;

  assert_eq!(values, vec!["cbr".to_owned(), "rust".to_owned()]);

  Ok(())
}
