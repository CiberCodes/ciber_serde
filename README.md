<div>

# Ciber Serde

[![CI](https://github.com/CiberCodes/ciber_serde/actions/workflows/ci.yml/badge.svg)](https://github.com/CiberCodes/ciber_serde/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/ciber_serde.svg)](https://crates.io/crates/ciber_serde)
[![Documentation](https://docs.rs/ciber_serde/badge.svg)](https://docs.rs/ciber_serde)
[![License](https://img.shields.io/badge/license-MIT%20%2F%20Apache--2.0-blue.svg)](#license)

**Serde serialization and deserialization for the CBR data format**

---

## Overview

`ciber_serde` connects Rust types that implement [Serde](https://serde.rs/) to the
CBR format provided by [`ciber_cbr`](https://github.com/CiberCodes/ciber_cbr).
It serializes Rust values into CBR documents and deserializes CBR documents back
into Rust types.

This crate provides `CbrSerializer`, `CbrDeserializer`, and convenience
functions for reading, writing, and converting CBR text. Parsing and document
formatting are handled by `ciber_cbr`.

## Features

- Serialize Serde-compatible values to compact or pretty CBR text.
- Deserialize CBR documents from readers and strings.
- Support structs, maps, sequences, tuples, tuple structs, and enum variants.
- Preserve CBR value types, including nulls, booleans, integers, floats,
  strings, bytes, arrays, objects, and sets.
- Check integer conversions against the destination type's range.

## Installation

Add the crate to your `Cargo.toml`:

```toml
[dependencies]
ciber_serde = "0.1"
serde = { version = "1", features = ["derive"] }
```

The `0.1` requirement selects compatible `0.1.x` releases, including `0.1.0`.

## Quick start

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Application {
    name: String,
    version: usize,
    enabled: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = Application {
        name: "CbrApplication".to_owned(),
        version: 1,
        enabled: true,
    };

    let document = ciber_serde::to_string_pretty(&app)?;
    println!("{document}");

    let decoded: Application = ciber_serde::from_str(&document)?;
    assert_eq!(app, decoded);

    Ok(())
}
```

For file and stream I/O, use `write` and `read`:

```rust,no_run
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Settings {
    enabled: bool,
}

fn save_and_load() -> Result<(), Box<dyn std::error::Error>> {
    let settings = Settings { enabled: true };
    let file = std::fs::File::create("settings.cbr")?;
    ciber_serde::write(file, &settings)?;

    let file = std::fs::File::open("settings.cbr")?;
    let loaded: Settings = ciber_serde::read(file)?;
    assert!(loaded.enabled);

    Ok(())
}
```

## API overview

| Function                                               | Purpose                                 |
|--------------------------------------------------------|-----------------------------------------|
| `to_string(value)`                                     | Serialize to compact CBR text.          |
| `to_string_pretty(value)`                              | Serialize to pretty-printed CBR text.   |
| `from_str(text)` / `from_string(text)`                 | Parse CBR text and deserialize it.      |
| `write(writer, value)` / `write_pretty(writer, value)` | Serialize to a writer.                  |
| `read(reader)`                                         | Read a CBR document and deserialize it. |
| `CbrSerializer` / `CbrDeserializer`                    | Use the Serde integration directly.     |

## Examples and tests

Runnable examples are in [`examples/`](examples/), including a file read/write
round trip. Integration tests cover serialization and deserialization for
maps, sequences, structs, tuples, and enum variants.

Run the test suite with:

```bash
cargo test
```

Run the examples with:

```bash
cargo run --example file
```

## Related crate

[`ciber_cbr`](https://github.com/CiberCodes/ciber_cbr) implements the CBR value
model, parser, formatter, and document I/O used by this Serde integration.

## License

Licensed under either of the following, at your option:

- [MIT License](LICENSE-MIT)
- [Apache License 2.0](LICENSE-APACHE)

---

<sub>Engineered by CiberCodes</sub>
</div>
