//! Sequence deserialization support for CBR values.
//!
//! Provides sequence access for CBR arrays and sets through
//! Serde's sequence deserialization interface.

use std::vec::IntoIter;

use ciber_cbr::Value;
use serde::de::{DeserializeSeed, SeqAccess, Visitor};

use crate::error::{Error, Result};

/// Provides sequential access to the elements of a CBR sequence.
///
/// This structure implements [`SeqAccess`] and allows Serde visitors
/// to deserialize individual elements from an owned iterator.
///
/// The underlying iterator preserves the order of arrays. For sets,
/// elements are provided in the iteration order defined by the
/// underlying `BTreeSet`.
#[derive(Debug)]
pub(super) struct CbrSeqAccess {
  elements: IntoIter<Value>,
}

impl CbrSeqAccess {
  /// Creates a new sequence access instance.
  ///
  /// # Arguments
  ///
  /// - `elements`: The elements to make available for deserialization.
  ///
  /// # Returns
  ///
  /// Returns a sequence access instance containing the supplied
  /// elements.
  #[must_use]
  pub(super) fn new(elements: Vec<Value>) -> Self {
    Self { elements: elements.into_iter() }
  }
}

impl<'de> SeqAccess<'de> for CbrSeqAccess {
  type Error = Error;

  /// Deserializes the next element using the supplied seed.
  ///
  /// Each call consumes the next available element from the
  /// underlying iterator.
  ///
  /// # Arguments
  ///
  /// - `seed`: The deserialization seed used to deserialize the next element.
  ///
  /// # Returns
  ///
  /// Returns `Some` containing the deserialized element when
  /// an element is available, or `None` when the sequence is
  /// exhausted.
  ///
  /// # Errors
  ///
  /// Returns an error if the next element cannot be
  /// deserialized into the requested type.
  fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
  where
    T: DeserializeSeed<'de>,
  {
    self
      .elements
      .next()
      .map(|value| super::deserialize_value(value, seed))
      .transpose()
  }

  /// Returns the number of elements remaining in the sequence.
  ///
  /// # Returns
  ///
  /// Returns the exact number of elements that can still be
  /// consumed from the underlying iterator.
  fn size_hint(&self) -> Option<usize> { Some(self.elements.len()) }
}

/// Deserializes a CBR sequence using the supplied visitor.
///
/// Creates a [`CbrSeqAccess`] instance and passes it to the visitor
/// through Serde's sequence interface.
///
/// # Arguments
///
/// - `elements`: The elements belonging to the sequence.
/// - `visitor`: The visitor responsible for constructing the requested Rust
///   value.
///
/// # Returns
///
/// Returns the value produced by the visitor.
///
/// # Errors
///
/// Returns an error if the visitor or any element deserialization
/// fails.
pub fn deserialize_sequence<'de, V>(
  elements: Vec<Value>,
  visitor: V,
) -> Result<V::Value>
where
  V: Visitor<'de>,
{
  visitor.visit_seq(CbrSeqAccess::new(elements))
}

/// Deserializes a CBR array or set using the supplied visitor.
///
/// Arrays preserve their original element order. Sets are converted
/// into sequences using their natural iteration order.
///
/// # Arguments
///
/// - `value`: The CBR value to deserialize.
/// - `visitor`: The visitor responsible for constructing the requested Rust
///   value.
///
/// # Returns
///
/// Returns the value produced by the visitor.
///
/// # Errors
///
/// Returns an error if the supplied value is neither an array nor
/// a set, or if an element cannot be deserialized.
pub fn deserialize_sequence_value<'de, V>(
  value: Value,
  visitor: V,
) -> Result<V::Value>
where
  V: Visitor<'de>,
{
  match value {
    Value::Array(elements) => deserialize_sequence(elements, visitor),
    Value::Set(elements) => {
      deserialize_sequence(elements.into_iter().collect(), visitor)
    }
    value => Err(Error::new(format!(
      "invalid type: {}, expected a sequence",
      super::deserializer::ValueDescription(&value),
    ))),
  }
}
