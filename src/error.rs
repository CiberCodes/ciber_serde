//! Error handling for cbr format serialization

use std::fmt;

/// The result type used by `ciber_serde`.
pub type Result<T> = std::result::Result<T, Error>;

/// An error produced during CBR serialization or deserialization.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Error {
  message: String,
}

impl Error {
  /// Creates a new serialization or deserialization error.
  ///
  /// # Arguments
  ///
  /// * `message`: A description of the error.
  #[must_use]
  pub fn new(message: impl Into<String>) -> Self {
    Self { message: message.into() }
  }

  /// Returns the error message.
  #[must_use]
  pub fn message(&self) -> &str { &self.message }
}

impl fmt::Display for Error {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str(&self.message)
  }
}

impl std::error::Error for Error {}

impl From<ciber_cbr::error::Error> for Error {
  fn from(value: ciber_cbr::Error) -> Self { Self::new(value.message()) }
}

impl serde::ser::Error for Error {
  fn custom<T>(message: T) -> Self
  where
    T: fmt::Display,
  {
    Self::new(message.to_string())
  }
}

impl serde::de::Error for Error {
  fn custom<T>(message: T) -> Self
  where
    T: fmt::Display,
  {
    Self::new(message.to_string())
  }
}
