//! Tests for CBR structure deserialization.

use std::collections::BTreeMap;

use ciber_cbr::Value;
use ciber_serde::CbrDeserializer;
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
struct User {
  id: u64,
  name: String,
  active: bool,
}

/// Deserializes a structure containing primitive fields.
#[test]
fn deserializes_structures() -> Result<(), Box<dyn std::error::Error>> {
  let value = Value::Object(BTreeMap::from([
    ("id".to_owned(), Value::U64(42)),
    ("name".to_owned(), Value::String("Cirious".to_owned())),
    ("active".to_owned(), Value::Bool(true)),
  ]));

  let user = User::deserialize(CbrDeserializer::new(value))?;

  assert_eq!(user, User { id: 42, name: "Cirious".to_owned(), active: true });

  Ok(())
}

#[derive(Debug, Deserialize, PartialEq)]
struct OptionalUser {
  name: String,
  email: Option<String>,
}

/// Deserializes a structure containing a null optional field.
#[test]
fn deserializes_structures_with_optional_fields()
-> Result<(), Box<dyn std::error::Error>> {
  let value = Value::Object(BTreeMap::from([
    ("name".to_owned(), Value::String("Cirious".to_owned())),
    ("email".to_owned(), Value::Null),
  ]));

  let user = OptionalUser::deserialize(CbrDeserializer::new(value))?;

  assert_eq!(user, OptionalUser { name: "Cirious".to_owned(), email: None });

  Ok(())
}

#[derive(Debug, Deserialize, PartialEq)]
struct Address {
  city: String,
  number: u32,
}

#[derive(Debug, Deserialize, PartialEq)]
struct Profile {
  user: User,
  address: Address,
}

/// Deserializes nested structures.
#[test]
fn deserializes_nested_structures() -> Result<(), Box<dyn std::error::Error>> {
  let value = Value::Object(BTreeMap::from([
    (
      "user".to_owned(),
      Value::Object(BTreeMap::from([
        ("id".to_owned(), Value::U64(42)),
        ("name".to_owned(), Value::String("Cirious".to_owned())),
        ("active".to_owned(), Value::Bool(true)),
      ])),
    ),
    (
      "address".to_owned(),
      Value::Object(BTreeMap::from([
        ("city".to_owned(), Value::String("Itaúna".to_owned())),
        ("number".to_owned(), Value::U64(42)),
      ])),
    ),
  ]));

  let profile = Profile::deserialize(CbrDeserializer::new(value))?;

  assert_eq!(profile.user.id, 42);
  assert_eq!(profile.address.city, "Itaúna");
  assert_eq!(profile.address.number, 42);

  Ok(())
}
