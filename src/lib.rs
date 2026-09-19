//! # Kyogars
//!
//! Parser for Kyogas, A simple markdown language.
//!
//! Read more about Kyogas [here](https://github.com/pixelqualitysoftware).
//!
//! View our repository [here](https://github.com/pixelqualitysoftware/kyogars)
//!
//! Writen by [Tuxzilla T. Penguin](https://tuxzilla.com)
//! Email for support [here](mailto:tuxzilla@tuxzilla.com)

// src/lib.rs

mod errors;
mod lexer;
mod parser;
use errors::KyoError;
use parser::ParseOutcome;

pub fn load_file(path: impl AsRef<std::path::Path>) -> Result<ParseOutcome, KyoError> {
    let mut file = std::fs::File::open(path)?;
    parser::parse_file(&mut file)
}
