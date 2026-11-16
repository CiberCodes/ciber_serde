//! Tests for CBR tuple serialization.

use ciber_cbr::Value;
use ciber_serde::CbrSerializer;
use serde::Serialize;

/// Serializes a tuple containing multiple primitive values.
#[test]
fn serializes_tuples() -> Result<(), Box<dyn std::error::Error>> {
  let value = (42_i32, "Cirious", true);

  assert_eq!(
    value.serialize(CbrSerializer)?,
    Value::Array(vec![
      Value::I64(42),
      Value::String("Cirious".to_owned()),
      Value::Bool(true),
    ]),
  );

  Ok(())
}

/// Serializes an empty tuple.
#[test]
fn serializes_empty_tuples() -> Result<(), Box<dyn std::error::Error>> {
  let value = ();

  assert_eq!(value.serialize(CbrSerializer)?, Value::Null,);

  Ok(())
}

/// Serializes a tuple containing a single element.
#[test]
fn serializes_single_element_tuples() -> Result<(), Box<dyn std::error::Error>>
{
  let value = (42_i32,);

  assert_eq!(
    value.serialize(CbrSerializer)?,
    Value::Array(vec![Value::I64(42)]),
  );

  Ok(())
}

/// Serializes nested tuples.
#[test]
fn serializes_nested_tuples() -> Result<(), Box<dyn std::error::Error>> {
  let value = ((10_i32, 20_i32), (30_i32, 40_i32));

  assert_eq!(
    value.serialize(CbrSerializer)?,
    Value::Array(vec![
      Value::Array(vec![Value::I64(10), Value::I64(20)]),
      Value::Array(vec![Value::I64(30), Value::I64(40)]),
    ]),
  );

  Ok(())
}

/// Serializes a tuple containing optional values.
#[test]
fn serializes_tuples_with_optional_values()
-> Result<(), Box<dyn std::error::Error>> {
  let value = (Some(42_i32), None::<i32>);

  assert_eq!(
    value.serialize(CbrSerializer)?,
    Value::Array(vec![Value::I64(42), Value::Null,]),
  );

  Ok(())
}

/// Serializes a tuple containing a sequence.
#[test]
fn serializes_tuples_with_sequences() -> Result<(), Box<dyn std::error::Error>>
{
  let value = ("numbers", vec![1_i32, 2, 3]);

  assert_eq!(
    value.serialize(CbrSerializer)?,
    Value::Array(vec![
      Value::String("numbers".to_owned()),
      Value::Array(vec![Value::I64(1), Value::I64(2), Value::I64(3),]),
    ]),
  );

  Ok(())
}
