//! Tests for CBR struct variant serialization.

use std::collections::BTreeMap;

use ciber_cbr::Value;
use ciber_serde::CbrSerializer;
use serde::Serialize;

/// Represents application events for serialization tests.
#[derive(Serialize)]

enum UserEvent {
  Created { id: u64, name: String },

  Deleted { id: u64 },

  Updated { id: u64, active: bool },
}

/// Represents an address used in nested variant tests.
#[derive(Serialize)]

struct Address {
  city: String,
  number: u32,
}

/// Represents events containing nested structures.
#[derive(Serialize)]

enum ProfileEvent {
  AddressUpdated { user_id: u64, address: Address },
}

/// Creates an expected CBR object for a struct variant.
fn expected_variant(variant: &str, fields: BTreeMap<String, Value>) -> Value {
  let mut object = BTreeMap::new();

  object.insert(variant.to_owned(), Value::Object(fields));

  Value::Object(object)
}

/// Serializes a struct variant containing multiple fields.
#[test]
fn serializes_struct_variants() -> Result<(), Box<dyn std::error::Error>> {
  let event = UserEvent::Created { id: 42, name: "Cirious".to_owned() };

  let mut fields = BTreeMap::new();

  fields.insert("id".to_owned(), Value::U64(42));

  fields.insert("name".to_owned(), Value::String("Cirious".to_owned()));

  assert_eq!(
    event.serialize(CbrSerializer)?,
    expected_variant("Created", fields),
  );

  Ok(())
}

/// Serializes a struct variant containing a single field.
#[test]
fn serializes_single_field_variants() -> Result<(), Box<dyn std::error::Error>>
{
  let event = UserEvent::Deleted { id: 42 };

  let mut fields = BTreeMap::new();

  fields.insert("id".to_owned(), Value::U64(42));

  assert_eq!(
    event.serialize(CbrSerializer)?,
    expected_variant("Deleted", fields),
  );

  Ok(())
}

/// Serializes struct variants with different field types.
#[test]
fn serializes_variants_with_mixed_fields()
-> Result<(), Box<dyn std::error::Error>> {
  let event = UserEvent::Updated { id: 42, active: true };

  let mut fields = BTreeMap::new();

  fields.insert("id".to_owned(), Value::U64(42));

  fields.insert("active".to_owned(), Value::Bool(true));

  assert_eq!(
    event.serialize(CbrSerializer)?,
    expected_variant("Updated", fields),
  );

  Ok(())
}

/// Serializes a struct variant containing a nested struct.
#[test]
fn serializes_nested_struct_variants() -> Result<(), Box<dyn std::error::Error>>
{
  let event = ProfileEvent::AddressUpdated {
    user_id: 42,
    address: Address { city: "Itaúna".to_owned(), number: 100 },
  };

  let mut address = BTreeMap::new();

  address.insert("city".to_owned(), Value::String("Itaúna".to_owned()));

  address.insert("number".to_owned(), Value::U64(100));

  let mut fields = BTreeMap::new();

  fields.insert("user_id".to_owned(), Value::U64(42));

  fields.insert("address".to_owned(), Value::Object(address));

  assert_eq!(
    event.serialize(CbrSerializer)?,
    expected_variant("AddressUpdated", fields),
  );

  Ok(())
}
