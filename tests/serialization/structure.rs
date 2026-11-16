//! Tests for CBR struct serialization.

use std::collections::BTreeMap;

use ciber_cbr::Value;
use ciber_serde::CbrSerializer;
use serde::Serialize;

/// Represents a user for serialization tests.
#[derive(Serialize)]

struct User {
  id: u64,
  name: String,
  active: bool,
}

/// Represents an address for nested struct tests.
#[derive(Serialize)]

struct Address {
  city: String,
  number: u32,
}

/// Represents a user with an optional field.
#[derive(Serialize)]

struct OptionalUser {
  name: String,
  email: Option<String>,
}

/// Represents an empty struct.
#[derive(Serialize)]

struct EmptyStruct {}

/// Serializes a struct containing primitive fields.
#[test]
fn serializes_structs() -> Result<(), Box<dyn std::error::Error>> {
  let user = User { id: 42, name: "Cirious".to_owned(), active: true };

  let mut expected = BTreeMap::new();

  expected.insert("id".to_owned(), Value::U64(42));

  expected.insert("name".to_owned(), Value::String("Cirious".to_owned()));

  expected.insert("active".to_owned(), Value::Bool(true));

  assert_eq!(user.serialize(CbrSerializer)?, Value::Object(expected),);

  Ok(())
}

/// Serializes an empty struct.
#[test]
fn serializes_empty_structs() -> Result<(), Box<dyn std::error::Error>> {
  assert_eq!(
    EmptyStruct {}.serialize(CbrSerializer)?,
    Value::Object(BTreeMap::new()),
  );

  Ok(())
}

/// Serializes a struct containing an optional field.
#[test]
fn serializes_structs_with_optional_fields()
-> Result<(), Box<dyn std::error::Error>> {
  let user = OptionalUser { name: "Cirious".to_owned(), email: None };

  let mut expected = BTreeMap::new();

  expected.insert("name".to_owned(), Value::String("Cirious".to_owned()));

  expected.insert("email".to_owned(), Value::Null);

  assert_eq!(user.serialize(CbrSerializer)?, Value::Object(expected),);

  Ok(())
}

/// Serializes nested structs.
#[test]
fn serializes_nested_structs() -> Result<(), Box<dyn std::error::Error>> {
  #[derive(Serialize)]

  struct Profile {
    user: User,
    address: Address,
  }

  let user = User { id: 42, name: "Cirious".to_owned(), active: true };

  let address = Address { city: "Itaúna".to_owned(), number: 42 };

  let profile = Profile { user, address };

  let mut user = BTreeMap::new();

  user.insert("id".to_owned(), Value::U64(42));

  user.insert("name".to_owned(), Value::String("Cirious".to_owned()));

  user.insert("active".to_owned(), Value::Bool(true));

  let mut address = BTreeMap::new();

  address.insert("city".to_owned(), Value::String("Itaúna".to_owned()));

  address.insert("number".to_owned(), Value::U64(42));

  let mut expected = BTreeMap::new();

  expected.insert("user".to_owned(), Value::Object(user));

  expected.insert("address".to_owned(), Value::Object(address));

  assert_eq!(profile.serialize(CbrSerializer)?, Value::Object(expected),);

  Ok(())
}

/// Serializes a struct containing a sequence field.
#[test]
fn serializes_structs_with_sequence_fields()
-> Result<(), Box<dyn std::error::Error>> {
  #[derive(Serialize)]

  struct Project {
    name: String,
    tags: Vec<String>,
  }

  let project = Project {
    name: "CiberCodes".to_owned(),
    tags: vec!["rust".to_owned(), "cbr".to_owned()],
  };

  let mut expected = BTreeMap::new();

  expected.insert("name".to_owned(), Value::String("CiberCodes".to_owned()));

  expected.insert(
    "tags".to_owned(),
    Value::Array(vec![
      Value::String("rust".to_owned()),
      Value::String("cbr".to_owned()),
    ]),
  );

  assert_eq!(project.serialize(CbrSerializer)?, Value::Object(expected),);

  Ok(())
}
