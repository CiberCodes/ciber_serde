//! Deserialization access helpers.
//!
//! Provides shared utilities for converting CBR values into
//! Serde deserialization inputs.

use ciber_cbr::Value;
use serde::de::{DeserializeSeed, EnumAccess, VariantAccess, Visitor};

use super::deserializer::CbrDeserializer;
use crate::error::{Error, Result};

/// Deserializes a CBR value using the supplied seed.
///
/// # Arguments
///
/// - `value`: The CBR value to deserialize.
/// - `seed`: The deserialization seed.
///
/// # Returns
///
/// Returns the value produced by the seed.
///
/// # Errors
///
/// Returns an error if the value cannot be deserialized.
pub(super) fn deserialize_value<'de, S>(
  value: Value,
  seed: S,
) -> Result<S::Value>
where
  S: DeserializeSeed<'de>,
{
  seed.deserialize(CbrDeserializer::new(value))
}

/// Provides access to a CBR enum variant.
///
/// Stores the variant identifier and its optional associated value.
#[derive(Debug)]
pub(super) struct CbrEnumAccess {
  variant: String,
  value: Option<Value>,
}

impl CbrEnumAccess {
  /// Creates a new enum access instance.
  ///
  /// # Arguments
  ///
  /// - `variant`: The enum variant identifier.
  /// - `value`: The optional value associated with the variant.
  #[must_use]
  pub(super) const fn new(variant: String, value: Option<Value>) -> Self {
    Self { variant, value }
  }
}

impl<'de> EnumAccess<'de> for CbrEnumAccess {
  type Error = Error;
  type Variant = CbrVariantAccess;

  /// Deserializes the enum variant identifier.
  ///
  /// # Errors
  ///
  /// Returns an error if the identifier cannot be deserialized.
  fn variant_seed<S>(self, seed: S) -> Result<(S::Value, Self::Variant)>
  where
    S: DeserializeSeed<'de>,
  {
    let variant = seed.deserialize(serde::de::value::StringDeserializer::<
      Error,
    >::new(self.variant))?;

    Ok((variant, CbrVariantAccess::new(self.value)))
  }
}

/// Provides access to the contents of a CBR enum variant.
#[derive(Debug)]
pub(super) struct CbrVariantAccess {
  value: Option<Value>,
}

impl CbrVariantAccess {
  /// Creates a new variant access instance.
  ///
  /// # Arguments
  ///
  /// - `value`: The optional value associated with the variant.
  #[must_use]
  pub(super) const fn new(value: Option<Value>) -> Self { Self { value } }

  /// Takes the associated value from this variant.
  ///
  /// # Errors
  ///
  /// Returns an error if the variant has no associated value.
  fn take_value(self) -> Result<Value> {
    self.value.ok_or_else(|| {
      Error::new("expected a variant containing associated data")
    })
  }
}

impl<'de> VariantAccess<'de> for CbrVariantAccess {
  type Error = Error;

  /// Deserializes a unit variant.
  ///
  /// # Errors
  ///
  /// Returns an error if the variant contains associated data.
  fn unit_variant(self) -> Result<()> {
    match self.value {
      None | Some(Value::Null) => Ok(()),
      Some(value) => Err(Error::new(format!(
        "expected a unit variant, found {}",
        super::deserializer::ValueDescription(&value),
      ))),
    }
  }

  /// Deserializes a newtype variant.
  ///
  /// # Errors
  ///
  /// Returns an error if the variant has no associated value or
  /// if its value cannot be deserialized.
  fn newtype_variant_seed<S>(self, seed: S) -> Result<S::Value>
  where
    S: DeserializeSeed<'de>,
  {
    deserialize_value(self.take_value()?, seed)
  }

  /// Deserializes a tuple variant.
  ///
  /// # Errors
  ///
  /// Returns an error if the associated value is not a valid tuple
  /// representation or if one of its elements cannot be deserialized.
  fn tuple_variant<V>(self, length: usize, visitor: V) -> Result<V::Value>
  where
    V: Visitor<'de>,
  {
    super::compound::tuple_variant::deserialize_tuple_variant(
      self.take_value()?,
      length,
      visitor,
    )
  }

  /// Deserializes a structure variant.
  ///
  /// # Errors
  ///
  /// Returns an error if the associated value is not a CBR object
  /// or if one of its fields cannot be deserialized.
  fn struct_variant<V>(
    self,
    fields: &'static [&'static str],
    visitor: V,
  ) -> Result<V::Value>
  where
    V: Visitor<'de>,
  {
    super::compound::structure_variant::deserialize_structure_variant(
      self.take_value()?,
      fields,
      visitor,
    )
  }
}

/// Ensures that the enum representation contains exactly one variant.
pub(super) fn single_variant(
  entries: std::collections::BTreeMap<String, Value>,
) -> Result<(String, Value)> {
  if entries.len() != 1 {
    return Err(Error::new(
      "expected an enum representation containing exactly one variant",
    ));
  }

  entries.into_iter().next().ok_or_else(|| Error::new("missing enum variant"))
}
