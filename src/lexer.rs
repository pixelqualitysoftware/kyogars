use logos::Logos;

pub fn lex_file(input: &str) -> logos::Lexer<Token> {
    Token::lexer(input)
}

#[derive(Debug, Logos, PartialEq)]
#[logos(skip r"[ \t]+")]
#[logos(skip r"//[^\r\n]*?")]
pub enum Token {
    #[token("\n")]
    Newline,

    #[token(":")]
    Colon,

    #[token("<")]
    TypeOpen,

    #[token(">")]
    TypeClose,

    #[token("<-")]
    ArrayStart,

    #[token("->")]
    ArrayEnd,

    #[token("str")]
    Str,

    #[token("short")]
    Int16,

    #[token("ushort")]
    UInt16,

    #[token("long")]
    Int64,

    #[token("ulong")]
    UInt64,

    #[token("flt")]
    Flt,

    #[token("byte")]
    Byte8,

    #[token("sbyte")]
    SByte8,

    #[token("bool")]
    Bool,

    #[regex(r"[A-Za-z_][A-Za-z0-9_]*")]
    Identifier,

    #[regex(r"-?(0|[1-9][0-9]*)")]
    Integer,

    #[regex(r"-?(0|[1-9][0-9]*)\.[0-9]+")]
    Float,

    #[regex(r#""([^"\\]|\\.)*""#)]
    String,

    #[token("true")]
    True,

    #[token("false")]
    False,
}
