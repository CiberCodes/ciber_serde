//! Minimal CBR Serde integration example.

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
struct Example {
  enabled: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let original = Example { enabled: true };
  let encoded = ciber_serde::to_string(&original)?;
  let decoded: Example = ciber_serde::from_str(&encoded)?;

  assert_eq!(original, decoded);

  Ok(())
}
