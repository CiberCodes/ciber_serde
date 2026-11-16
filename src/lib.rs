//! Serde support for the CBR data format.

#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod de;
pub mod error;
pub mod ser;

use std::io::Cursor;

use ciber_cbr::CbrFile;
pub use de::deserializer::CbrDeserializer;
pub use error::{Error, Result};
pub use ser::CbrSerializer;
use serde::Serialize;

#[inline]
fn into_cbr_file<T>(value: &T) -> Result<CbrFile>
where
  T: ?Sized + Serialize,
{
  let value = value.serialize(CbrSerializer)?;
  Ok(CbrFile::from_value(value))
}

#[inline]
/// Serializes a value to compact CBR text.
///
/// # Errors
///
/// Returns an error if serialization or CBR formatting fails.
pub fn to_string<T>(value: &T) -> Result<String>
where
  T: ?Sized + Serialize,
{
  let file = into_cbr_file(&value)?;
  let content = file.to_string()?;
  Ok(content)
}

#[inline]
/// Serializes a value to pretty-printed CBR text.
///
/// # Errors
///
/// Returns an error if serialization or CBR formatting fails.
pub fn to_string_pretty<T>(value: &T) -> Result<String>
where
  T: ?Sized + Serialize,
{
  let file = into_cbr_file(&value)?;
  let content = file.to_string_pretty()?;
  Ok(content)
}

#[inline]
/// Serializes a value and writes compact CBR text to `writer`.
///
/// # Errors
///
/// Returns an error if serialization, formatting, or writing fails.
pub fn write<W, T>(writer: W, value: &T) -> Result<()>
where
  W: std::io::Write,
  T: ?Sized + Serialize,
{
  let file = into_cbr_file(&value)?;
  file.write_to(writer)?;
  Ok(())
}

#[inline]
/// Serializes a value and writes pretty-printed CBR text to `writer`.
///
/// # Errors
///
/// Returns an error if serialization, formatting, or writing fails.
pub fn write_pretty<W, T>(writer: W, value: &T) -> Result<()>
where
  W: std::io::Write,
  T: ?Sized + Serialize,
{
  let file = into_cbr_file(&value)?;
  file.write_pretty_to(writer)?;
  Ok(())
}

/// Reads a CBR document from `read` and deserializes its value.
///
/// # Errors
///
/// Returns an error if reading, parsing, or deserialization fails.
pub fn read<R, T>(read: R) -> Result<T>
where
  R: std::io::Read,
  T: serde::de::DeserializeOwned,
{
  let file = CbrFile::from_reader(read)?;
  let de = CbrDeserializer::new(file.into_inner());
  let value = de.deserialize::<T>()?;
  Ok(value)
}

/// Parses CBR text and deserializes its value.
///
/// # Errors
///
/// Returns an error if parsing or deserialization fails.
pub fn from_str<T>(s: &str) -> Result<T>
where
  T: serde::de::DeserializeOwned,
{
  from_string(s)
}

/// Parses owned CBR text and deserializes its value.
///
/// # Errors
///
/// Returns an error if parsing or deserialization fails.
pub fn from_string<T>(s: impl Into<String>) -> Result<T>
where
  T: serde::de::DeserializeOwned,
{
  let mut string = s.into();
  let value = read(Cursor::new(&mut string))?;
  Ok(value)
}
