//! Tests for CBR map serialization.

use std::{collections::BTreeMap, error::Error};

use ciber_cbr::Value;
use ciber_serde::{CbrSerializer, ser::compound::CbrSerializeMap};
use serde::{Serialize, ser::SerializeMap};

/// Serializes a map containing string keys and primitive values.
#[test]
fn serializes_maps() -> Result<(), Box<dyn Error>> {
  let mut map = BTreeMap::new();

  map.insert("name", "Cirious");

  map.insert("version", "0.1.0");

  let mut expected = BTreeMap::new();

  expected.insert("name".to_owned(), Value::String("Cirious".to_owned()));

  expected.insert("version".to_owned(), Value::String("0.1.0".to_owned()));

  assert_eq!(map.serialize(CbrSerializer)?, Value::Object(expected));

  Ok(())
}

/// Serializes an empty map.
#[test]
fn serializes_empty_maps() -> Result<(), Box<dyn Error>> {
  let map = BTreeMap::<String, i32>::new();

  assert_eq!(map.serialize(CbrSerializer)?, Value::Object(BTreeMap::new()),);

  Ok(())
}

/// Rejects map keys that do not serialize into strings.
#[test]
fn rejects_non_string_keys() {
  let mut serializer = CbrSerializeMap::new();

  assert!(SerializeMap::serialize_key(&mut serializer, &42_i32).is_err());
}

/// Rejects a value when no key is pending.
#[test]
fn rejects_value_without_key() {
  let mut serializer = CbrSerializeMap::new();

  assert!(SerializeMap::serialize_value(&mut serializer, &"value").is_err());
}

/// Rejects a second key when the previous entry is incomplete.
#[test]
fn rejects_key_before_previous_value() -> Result<(), Box<dyn Error>> {
  let mut serializer = CbrSerializeMap::new();

  SerializeMap::serialize_key(&mut serializer, "first")?;

  assert!(SerializeMap::serialize_key(&mut serializer, "second").is_err());

  Ok(())
}

/// Rejects ending a map with a pending key.
#[test]
fn rejects_pending_key_on_end() -> Result<(), Box<dyn Error>> {
  let mut serializer = CbrSerializeMap::new();

  SerializeMap::serialize_key(&mut serializer, "key")?;

  assert!(SerializeMap::end(serializer).is_err());

  Ok(())
}

/// Serializes a map entry after receiving its key and value.
#[test]
fn serializes_map_entries() -> Result<(), Box<dyn Error>> {
  let mut serializer = CbrSerializeMap::new();

  SerializeMap::serialize_key(&mut serializer, "name")?;

  SerializeMap::serialize_value(&mut serializer, &"Cirious")?;

  let mut expected = BTreeMap::new();

  expected.insert("name".to_owned(), Value::String("Cirious".to_owned()));

  assert_eq!(SerializeMap::end(serializer)?, Value::Object(expected),);

  Ok(())
}
