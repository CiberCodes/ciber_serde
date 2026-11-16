//! Tests for CBR sequence serialization.

use std::collections::BTreeMap;

use ciber_cbr::Value;
use ciber_serde::CbrSerializer;
use serde::Serialize;

/// Serializes a sequence of primitive values.
#[test]
fn serializes_sequences() -> Result<(), Box<dyn std::error::Error>> {
  let values = vec![1_i32, 2, 3];

  assert_eq!(
    values.serialize(CbrSerializer)?,
    Value::Array(vec![Value::I64(1), Value::I64(2), Value::I64(3),]),
  );

  Ok(())
}

/// Serializes an empty sequence.
#[test]
fn serializes_empty_sequences() -> Result<(), Box<dyn std::error::Error>> {
  let values: Vec<i32> = Vec::new();

  assert_eq!(values.serialize(CbrSerializer)?, Value::Array(Vec::new()),);

  Ok(())
}

/// Serializes a sequence containing different enum variants.
#[test]
fn serializes_mixed_sequences() -> Result<(), Box<dyn std::error::Error>> {
  let value = vec![
    ValueWrapper::Integer(42),
    ValueWrapper::Text("Cirious".to_owned()),
    ValueWrapper::Boolean(true),
  ];

  assert_eq!(
    value.serialize(CbrSerializer)?,
    Value::Array(vec![
      Value::Object(BTreeMap::from([("Integer".to_owned(), Value::I64(42),)])),
      Value::Object(BTreeMap::from([(
        "Text".to_owned(),
        Value::String("Cirious".to_owned()),
      )])),
      Value::Object(BTreeMap::from([(
        "Boolean".to_owned(),
        Value::Bool(true),
      )])),
    ]),
  );

  Ok(())
}

/// Serializes nested sequences.
#[test]
fn serializes_nested_sequences() -> Result<(), Box<dyn std::error::Error>> {
  let values = vec![vec![1_i32, 2], vec![3_i32, 4]];

  assert_eq!(
    values.serialize(CbrSerializer)?,
    Value::Array(vec![
      Value::Array(vec![Value::I64(1), Value::I64(2)]),
      Value::Array(vec![Value::I64(3), Value::I64(4)]),
    ]),
  );

  Ok(())
}

/// Serializes sequences containing optional values.
#[test]
fn serializes_sequences_with_optional_values()
-> Result<(), Box<dyn std::error::Error>> {
  let values = vec![Some(10_i32), None, Some(30_i32)];

  assert_eq!(
    values.serialize(CbrSerializer)?,
    Value::Array(vec![Value::I64(10), Value::Null, Value::I64(30),]),
  );

  Ok(())
}

/// Represents different values for mixed sequence testing.
#[derive(Serialize)]

enum ValueWrapper {
  Integer(i32),
  Text(String),
  Boolean(bool),
}
