//! Deserialization support for CBR values.
//!
//! Provides the deserializer implementation used to convert
//! CBR values into Rust types.

use std::fmt;

use ciber_cbr::Value;
use serde::de::{self, DeserializeOwned, Visitor};

use crate::error::{Error, Result};

macro_rules! deserialize_signed {
  ($deserializer:ident, $visitor:ident, $ty:ty, $visit:ident) => {
    match $deserializer.value {
      Value::I64(value) => {
        let value = <$ty>::try_from(value).map_err(|_| {
          Error::new(concat!("integer is out of range for ", stringify!($ty)))
        })?;

        $visitor.$visit(value)
      }
      Value::I128(value) => {
        let value = <$ty>::try_from(value).map_err(|_| {
          Error::new(concat!("integer is out of range for ", stringify!($ty)))
        })?;

        $visitor.$visit(value)
      }
      value => Err(Error::new(format!(
        "invalid type: {}, expected a signed integer",
        ValueDescription(&value),
      ))),
    }
  };
}

macro_rules! deserialize_unsigned {
  ($deserializer:ident, $visitor:ident, $ty:ty, $visit:ident) => {
    match $deserializer.value {
      Value::I64(value) => {
        let value = <$ty>::try_from(value).map_err(|_| {
          Error::new(concat!("integer is out of range for ", stringify!($ty)))
        })?;

        $visitor.$visit(value)
      }
      Value::U64(value) => {
        let value = <$ty>::try_from(value).map_err(|_| {
          Error::new(concat!("integer is out of range for ", stringify!($ty)))
        })?;

        $visitor.$visit(value)
      }
      Value::U128(value) => {
        let value = <$ty>::try_from(value).map_err(|_| {
          Error::new(concat!("integer is out of range for ", stringify!($ty)))
        })?;

        $visitor.$visit(value)
      }
      value => Err(Error::new(format!(
        "invalid type: {}, expected an unsigned integer",
        ValueDescription(&value),
      ))),
    }
  };
}

/// Deserializes CBR values into Rust types.
#[derive(Debug, Clone)]
pub struct CbrDeserializer {
  value: Value,
}

impl CbrDeserializer {
  /// Creates a new CBR deserializer.
  ///
  /// # Arguments
  ///
  /// - `value`: The CBR value to deserialize.
  #[must_use]
  pub const fn new(value: Value) -> Self { Self { value } }

  /// Deserializes the stored CBR value into the requested type.
  ///
  /// # Errors
  ///
  /// Returns an error if the value cannot be deserialized into `T`.
  pub fn deserialize<T>(self) -> Result<T>
  where
    T: DeserializeOwned,
  {
    T::deserialize(self)
  }

  /// Returns the stored CBR value.
  #[must_use]
  pub fn into_inner(self) -> Value { self.value }
}

impl<'de> de::Deserializer<'de> for CbrDeserializer {
  type Error = Error;

  fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
  where
    V: Visitor<'de>,
  {
    match self.value {
      Value::Null => visitor.visit_unit(),
      Value::Bool(value) => visitor.visit_bool(value),
      Value::I64(value) => visitor.visit_i64(value),
      Value::U64(value) => visitor.visit_u64(value),
      Value::F64(value) => visitor.visit_f64(value),
      Value::I128(value) => visitor.visit_i128(value),
      Value::U128(value) => visitor.visit_u128(value),
      Value::String(value) => visitor.visit_string(value),
      Value::Bytes(value) => visitor.visit_byte_buf(value),
      Value::Array(elements) => {
        super::compound::sequence::deserialize_sequence(elements, visitor)
      }
      Value::Set(elements) => super::compound::sequence::deserialize_sequence(
        elements.into_iter().collect(),
        visitor,
      ),
      Value::Object(entries) => {
        super::compound::map::deserialize_map(entries, visitor)
      }
    }
  }

  fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
  where
    V: Visitor<'de>,
  {
    match self.value {
      Value::Bool(value) => visitor.visit_bool(value),
      value => Err(invalid_type(&value, "a boolean")),
    }
  }

  fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value>
  where
    V: Visitor<'de>,
  {
    deserialize_signed!(self, visitor, i8, visit_i8)
  }

  fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value>
  where
    V: Visitor<'de>,
  {
    deserialize_signed!(self, visitor, i16, visit_i16)
  }

  fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value>
  where
    V: Visitor<'de>,
  {
    deserialize_signed!(self, visitor, i32, visit_i32)
  }

  fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value>
  where
    V: Visitor<'de>,
  {
    match self.value {
      Value::I64(value) => visitor.visit_i64(value),
      Value::I128(value) => {
        let value = i64::try_from(value)
          .map_err(|_| Error::new("integer is out of range for i64"))?;

        visitor.visit_i64(value)
      }
      value => Err(invalid_type(&value, "a signed integer")),
    }
  }

  fn deserialize_i128<V>(self, visitor: V) -> Result<V::Value>
  where
    V: Visitor<'de>,
  {
    match self.value {
      Value::I64(value) => visitor.visit_i128(i128::from(value)),
      Value::I128(value) => visitor.visit_i128(value),
      value => Err(invalid_type(&value, "a signed integer")),
    }
  }

  fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value>
  where
    V: Visitor<'de>,
  {
    deserialize_unsigned!(self, visitor, u8, visit_u8)
  }

  fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value>
  where
    V: Visitor<'de>,
  {
    deserialize_unsigned!(self, visitor, u16, visit_u16)
  }

  fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value>
  where
    V: Visitor<'de>,
  {
    deserialize_unsigned!(self, visitor, u32, visit_u32)
  }

  fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value>
  where
    V: Visitor<'de>,
  {
    match self.value {
      Value::I64(value) => {
        let value = u64::try_from(value)
          .map_err(|_| Error::new("integer is out of range for u64"))?;

        visitor.visit_u64(value)
      }
      Value::U64(value) => visitor.visit_u64(value),
      Value::U128(value) => {
        let value = u64::try_from(value)
          .map_err(|_| Error::new("integer is out of range for u64"))?;

        visitor.visit_u64(value)
      }
      value => Err(invalid_type(&value, "an unsigned integer")),
    }
  }

  fn deserialize_u128<V>(self, visitor: V) -> Result<V::Value>
  where
    V: Visitor<'de>,
  {
    match self.value {
      Value::I64(value) => {
        let value = u128::try_from(value)
          .map_err(|_| Error::new("integer is out of range for u128"))?;

        visitor.visit_u128(value)
      }
      Value::U64(value) => visitor.visit_u128(u128::from(value)),
      Value::U128(value) => visitor.visit_u128(value),
      value => Err(invalid_type(&value, "an unsigned integer")),
    }
  }

  fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value>
  where
    V: Visitor<'de>,
  {
    match self.value {
      Value::F64(value) =>
      {
        #[allow(clippy::cast_possible_truncation)]
        visitor.visit_f32(value as f32)
      }
      value => Err(invalid_type(&value, "a floating-point number")),
    }
  }

  fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value>
  where
    V: Visitor<'de>,
  {
    match self.value {
      Value::F64(value) => visitor.visit_f64(value),
      value => Err(invalid_type(&value, "a floating-point number")),
    }
  }

  fn deserialize_char<V>(self, visitor: V) -> Result<V::Value>
  where
    V: Visitor<'de>,
  {
    match self.value {
      Value::String(value) => {
        let mut chars = value.chars();

        let character = chars.next().ok_or_else(|| {
          Error::new("expected a single character, found an empty string")
        })?;

        if chars.next().is_some() {
          return Err(Error::new(
            "expected a single character, found multiple characters",
          ));
        }

        visitor.visit_char(character)
      }
      value => Err(invalid_type(&value, "a character")),
    }
  }

  fn deserialize_str<V>(self, visitor: V) -> Result<V::Value>
  where
    V: Visitor<'de>,
  {
    self.deserialize_string(visitor)
  }

  fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
  where
    V: Visitor<'de>,
  {
    match self.value {
      Value::String(value) => visitor.visit_string(value),
      value => Err(invalid_type(&value, "a string")),
    }
  }

  fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value>
  where
    V: Visitor<'de>,
  {
    self.deserialize_byte_buf(visitor)
  }

  fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value>
  where
    V: Visitor<'de>,
  {
    match self.value {
      Value::Bytes(value) => visitor.visit_byte_buf(value),
      value => Err(invalid_type(&value, "a byte buffer")),
    }
  }

  fn deserialize_option<V>(self, visitor: V) -> Result<V::Value>
  where
    V: Visitor<'de>,
  {
    match self.value {
      Value::Null => visitor.visit_none(),
      value => visitor.visit_some(Self::new(value)),
    }
  }

  fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value>
  where
    V: Visitor<'de>,
  {
    match self.value {
      Value::Null => visitor.visit_unit(),
      value => Err(invalid_type(&value, "unit")),
    }
  }

  fn deserialize_unit_struct<V>(
    self,
    _name: &'static str,
    visitor: V,
  ) -> Result<V::Value>
  where
    V: Visitor<'de>,
  {
    self.deserialize_unit(visitor)
  }

  fn deserialize_newtype_struct<V>(
    self,
    _name: &'static str,
    visitor: V,
  ) -> Result<V::Value>
  where
    V: Visitor<'de>,
  {
    visitor.visit_newtype_struct(self)
  }

  fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value>
  where
    V: Visitor<'de>,
  {
    super::compound::sequence::deserialize_sequence_value(self.value, visitor)
  }

  fn deserialize_tuple<V>(self, length: usize, visitor: V) -> Result<V::Value>
  where
    V: Visitor<'de>,
  {
    super::compound::tuple::deserialize_tuple_value(self.value, length, visitor)
  }

  fn deserialize_tuple_struct<V>(
    self,
    name: &'static str,
    length: usize,
    visitor: V,
  ) -> Result<V::Value>
  where
    V: Visitor<'de>,
  {
    super::compound::tuple_struct::deserialize_tuple_struct_value(
      name, self.value, length, visitor,
    )
  }

  fn deserialize_map<V>(self, visitor: V) -> Result<V::Value>
  where
    V: Visitor<'de>,
  {
    super::compound::map::deserialize_map_value(self.value, visitor)
  }

  fn deserialize_struct<V>(
    self,
    name: &'static str,
    fields: &'static [&'static str],
    visitor: V,
  ) -> Result<V::Value>
  where
    V: Visitor<'de>,
  {
    super::compound::structure::deserialize_structure_value(
      name, fields, self.value, visitor,
    )
  }

  fn deserialize_enum<V>(
    self,
    _name: &'static str,
    _variants: &'static [&'static str],
    visitor: V,
  ) -> Result<V::Value>
  where
    V: Visitor<'de>,
  {
    match self.value {
      Value::String(variant) => {
        visitor.visit_enum(super::access::CbrEnumAccess::new(variant, None))
      }
      Value::Object(entries) => {
        let (variant, value) = super::access::single_variant(entries)?;

        visitor
          .visit_enum(super::access::CbrEnumAccess::new(variant, Some(value)))
      }
      value => Err(Error::new(format!(
        "invalid type: {}, expected an enum",
        ValueDescription(&value),
      ))),
    }
  }

  fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value>
  where
    V: Visitor<'de>,
  {
    self.deserialize_string(visitor)
  }

  fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value>
  where
    V: Visitor<'de>,
  {
    visitor.visit_unit()
  }

  fn is_human_readable(&self) -> bool { true }
}

/// Converts a CBR value into an invalid-type error.
fn invalid_type(value: &Value, expected: &str) -> Error {
  Error::new(format!(
    "invalid type: {}, expected {expected}",
    ValueDescription(value)
  ))
}

/// Human-readable description of a CBR value.
pub(super) struct ValueDescription<'a>(pub(super) &'a Value);

impl fmt::Display for ValueDescription<'_> {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self.0 {
      Value::Null => formatter.write_str("null"),
      Value::Bool(_) => formatter.write_str("a boolean"),
      Value::I64(_) | Value::I128(_) => formatter.write_str("a signed integer"),
      Value::U64(_) | Value::U128(_) => {
        formatter.write_str("an unsigned integer")
      }
      Value::F64(_) => formatter.write_str("a floating-point number"),
      Value::String(_) => formatter.write_str("a string"),
      Value::Bytes(_) => formatter.write_str("a byte buffer"),
      Value::Array(_) => formatter.write_str("a sequence"),
      Value::Object(_) => formatter.write_str("a map"),
      Value::Set(_) => formatter.write_str("a set"),
    }
  }
}
