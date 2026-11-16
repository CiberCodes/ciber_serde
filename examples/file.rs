//! file Rust module file
//! Edit Documentation here ...

use std::error::Error;
use std::fs::File;

use serde::{Deserialize, Serialize};

/// Estrutura de criação de aplicação para o exemplo de integração
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ApplicationExample {
  name: String,
  version: usize,
  is_enabled: bool,
}

impl ApplicationExample {
  /// Função auxiliar para a criação de um novo aplicativo.
  #[must_use]
  pub fn new(name: &str, version: usize) -> Self {
    Self { name: name.to_string(), version, is_enabled: true }
  }
}

fn main() -> Result<(), Box<dyn Error>> {
  let file = File::create("examples/resources/Config.cbr")?;
  let app = ApplicationExample::new("CbrApplication", 1);
  ciber_serde::write(file, &app)?;

  let decoded: ApplicationExample =
    ciber_serde::read(File::open("examples/resources/Config.cbr")?)?;

  assert_eq!(app, decoded);

  Ok(())
}
