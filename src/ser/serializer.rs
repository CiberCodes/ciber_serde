//! Serialization support for CBR values.
//!
//! Provides the serializer implementation used to convert
//! Rust values into the CBR data model.

use std::collections::BTreeMap;

use ciber_cbr::Value;
use serde::ser::Serializer;

use super::compound::{
  map::CbrSerializeMap, sequence::CbrSerializeSeq,
  structure::CbrSerializeStruct, structure_variant::CbrSerializeStructVariant,
  tuple::CbrSerializeTuple, tuple_struct::CbrSerializeTupleStruct,
  tuple_variant::CbrSerializeTupleVariant,
};
use crate::error::{Error, Result};

/// Serializes Rust values into CBR values.
#[derive(Debug, Clone, Default)]
pub struct CbrSerializer;

impl CbrSerializer {
  /// Creates a new CBR serializer.
  #[must_use]
  pub const fn new() -> Self { Self }
}

impl Serializer for CbrSerializer {
  type Ok = Value;
  type Error = Error;
  type SerializeSeq = CbrSerializeSeq;
  type SerializeTuple = CbrSerializeTuple;
  type SerializeTupleStruct = CbrSerializeTupleStruct;
  type SerializeTupleVariant = CbrSerializeTupleVariant;
  type SerializeMap = CbrSerializeMap;
  type SerializeStruct = CbrSerializeStruct;
  type SerializeStructVariant = CbrSerializeStructVariant;

  fn serialize_bool(self, value: bool) -> Result<Value> {
    Ok(Value::Bool(value))
  }

  fn serialize_i8(self, value: i8) -> Result<Value> {
    Ok(Value::I64(i64::from(value)))
  }

  fn serialize_i16(self, value: i16) -> Result<Value> {
    Ok(Value::I64(i64::from(value)))
  }

  fn serialize_i32(self, value: i32) -> Result<Value> {
    Ok(Value::I64(i64::from(value)))
  }

  fn serialize_i64(self, value: i64) -> Result<Value> { Ok(Value::I64(value)) }

  fn serialize_i128(self, value: i128) -> Result<Value> {
    Ok(Value::I128(value))
  }

  fn serialize_u8(self, value: u8) -> Result<Value> {
    Ok(Value::U64(u64::from(value)))
  }

  fn serialize_u16(self, value: u16) -> Result<Value> {
    Ok(Value::U64(u64::from(value)))
  }

  fn serialize_u32(self, value: u32) -> Result<Value> {
    Ok(Value::U64(u64::from(value)))
  }

  fn serialize_u64(self, value: u64) -> Result<Value> { Ok(Value::U64(value)) }

  fn serialize_u128(self, value: u128) -> Result<Value> {
    Ok(Value::U128(value))
  }

  fn serialize_f32(self, value: f32) -> Result<Value> {
    Ok(Value::F64(f64::from(value)))
  }

  fn serialize_f64(self, value: f64) -> Result<Value> { Ok(Value::F64(value)) }

  fn serialize_char(self, value: char) -> Result<Value> {
    Ok(Value::String(value.to_string()))
  }

  fn serialize_str(self, value: &str) -> Result<Value> {
    Ok(Value::String(value.to_owned()))
  }

  fn serialize_bytes(self, value: &[u8]) -> Result<Value> {
    Ok(Value::Bytes(value.to_vec()))
  }

  fn serialize_none(self) -> Result<Value> { Ok(Value::Null) }

  fn serialize_some<T>(self, value: &T) -> Result<Value>
  where
    T: serde::Serialize + ?Sized,
  {
    value.serialize(self)
  }

  fn serialize_unit(self) -> Result<Value> { Ok(Value::Null) }

  fn serialize_unit_struct(self, _name: &'static str) -> Result<Value> {
    Ok(Value::Null)
  }

  fn serialize_unit_variant(
    self,
    _name: &'static str,
    _variant_index: u32,
    variant: &'static str,
  ) -> Result<Value> {
    Ok(Value::String(variant.to_owned()))
  }

  fn serialize_newtype_struct<T>(
    self,
    _name: &'static str,
    value: &T,
  ) -> Result<Value>
  where
    T: serde::Serialize + ?Sized,
  {
    value.serialize(self)
  }

  fn serialize_newtype_variant<T>(
    self,
    _name: &'static str,
    _variant_index: u32,
    variant: &'static str,
    value: &T,
  ) -> Result<Value>
  where
    T: serde::Serialize + ?Sized,
  {
    let serialized = value.serialize(Self)?;

    let mut object = BTreeMap::new();

    object.insert(variant.to_owned(), serialized);

    Ok(Value::Object(object))
  }

  fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq> {
    let mut sequence = CbrSerializeSeq::new();

    if let Some(len) = len {
      sequence.reserve(len);
    }

    Ok(sequence)
  }

  fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
    Ok(CbrSerializeTuple::new(len))
  }

  fn serialize_tuple_struct(
    self,
    _name: &'static str,
    len: usize,
  ) -> Result<Self::SerializeTupleStruct> {
    Ok(CbrSerializeTupleStruct::new(len))
  }

  fn serialize_tuple_variant(
    self,
    _name: &'static str,
    _variant_index: u32,
    variant: &'static str,
    len: usize,
  ) -> Result<Self::SerializeTupleVariant> {
    Ok(CbrSerializeTupleVariant::new(variant, len))
  }

  fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
    Ok(CbrSerializeMap::new())
  }

  fn serialize_struct(
    self,
    name: &'static str,
    len: usize,
  ) -> Result<Self::SerializeStruct> {
    Ok(CbrSerializeStruct::new(name, len))
  }

  fn serialize_struct_variant(
    self,
    _name: &'static str,
    _variant_index: u32,
    variant: &'static str,
    len: usize,
  ) -> Result<Self::SerializeStructVariant> {
    Ok(CbrSerializeStructVariant::new(variant, len))
  }

  fn collect_str<T>(self, value: &T) -> Result<Value>
  where
    T: std::fmt::Display + ?Sized,
  {
    Ok(Value::String(value.to_string()))
  }

  fn is_human_readable(&self) -> bool { true }
}
