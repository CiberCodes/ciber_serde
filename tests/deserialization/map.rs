//! Tests for CBR map deserialization.

use std::{collections::BTreeMap, error::Error};

use ciber_cbr::Value;
use ciber_serde::CbrDeserializer;
use serde::Deserialize;

/// Deserializes a map containing string keys and primitive values.
#[test]
fn deserializes_maps() -> Result<(), Box<dyn Error>> {
  let value = Value::Object(BTreeMap::from([
    ("name".to_owned(), Value::String("Cirious".to_owned())),
    ("version".to_owned(), Value::String("0.1.0".to_owned())),
  ]));

  let map =
    BTreeMap::<String, String>::deserialize(CbrDeserializer::new(value))?;

  assert_eq!(map.get("name").map(String::as_str), Some("Cirious"));
  assert_eq!(map.get("version").map(String::as_str), Some("0.1.0"));

  Ok(())
}

/// Deserializes an empty map.
#[test]
fn deserializes_empty_maps() -> Result<(), Box<dyn Error>> {
  let value = Value::Object(BTreeMap::new());
  let map = BTreeMap::<String, i32>::deserialize(CbrDeserializer::new(value))?;

  assert!(map.is_empty());

  Ok(())
}

/// Deserializes a map whose values contain nested CBR objects.
#[test]
fn deserializes_maps_with_nested_values() -> Result<(), Box<dyn Error>> {
  let value = Value::Object(BTreeMap::from([(
    "settings".to_owned(),
    Value::Object(BTreeMap::from([("enabled".to_owned(), Value::Bool(true))])),
  )]));

  let map = BTreeMap::<String, BTreeMap<String, bool>>::deserialize(
    CbrDeserializer::new(value),
  )?;

  assert!(map["settings"]["enabled"]);
  Ok(())
}

/// Deserializes maps containing optional values, mapping CBR null to `None`.
#[test]
fn deserializes_maps_with_optional_values() -> Result<(), Box<dyn Error>> {
  let value = Value::Object(BTreeMap::from([
    ("present".to_owned(), Value::I64(42)),
    ("missing".to_owned(), Value::Null),
  ]));

  let map =
    BTreeMap::<String, Option<i32>>::deserialize(CbrDeserializer::new(value))?;

  assert_eq!(map.get("present"), Some(&Some(42)));
  assert_eq!(map.get("missing"), Some(&None));

  Ok(())
}
