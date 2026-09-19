use crate::errors::KyoError;

use std::collections::HashMap;

use std::io::Read;

#[derive(Debug, Clone, PartialEq)]
pub enum KyoType {
    Str(String),
    Int(i64),
    UInt(u64),
    Byte(u8),
    Flt(f64),
    Bool(bool),
    Arr(Vec<KyoType>),
    Obj(HashMap<String, KyoType>),
    Null,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ParseOutcome {
    pub data: KyoType,
}

// this OBVIOUSLY. isn't complete.
// the parsing part of the parser
// uh
// doesnt do anything
pub fn parse_file(input: &mut std::fs::File) -> Result<ParseOutcome, KyoError> {
    let mut content = String::new();
    input.read_to_string(&mut content)?;
    let mut lexer = crate::lexer::lex_file(&content);

    while let Some(result) = lexer.next() {
        let span = lexer.span();
        let slice = lexer.slice();

        match result {
            Ok(token) => println!("{token:?} => {slice:?} at {span:?}"),
            Err(error) => println!("lexing error: {error:?}"),
        }
    }

    Ok(ParseOutcome {
        data: KyoType::Null,
    })
}
