pub mod error;
use error::Error;
use logos::{Logos, Lexer, Skip,};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Dir {
    Left,
    Right,
}

#[derive(Logos, Debug, PartialEq)]
#[logos(extras=(usize, usize), error = Error, skip r"[ \t\f]+")]
pub enum Token {
    #[regex(r"\n")]
    Newline,
    #[regex("-?[0-9]+", |lex| lex.slice().parse())]
    Int(i64),
    #[regex("-?[0-9]+\\.[0-9]+", |lex| lex.slice().parse())]
    Float(f64),
    // #[regex(r"\w+", word_callback)]
    // Word((usize, usize)),
    /// Tokens can be literal strings, of any length.
    #[token("fast")]
    Fast,
    #[token("=")]
    Eq,
    #[token(":")]
    Colon,
    #[token(".")]
    Period,
    #[token("<", |_| Dir::Left)]
    #[token(">", |_| Dir::Right)]
    Cmp(Dir),
    #[token("(", |_| Dir::Left)]
    #[token(")", |_| Dir::Right)]
    Paren(Dir),
    #[token("{", |_| Dir::Left)]
    #[token("}", |_| Dir::Right)]
    Brace(Dir),
    #[token("[", |_| Dir::Left)]
    #[token("]", |_| Dir::Right)]
    Bracket(Dir),
    #[token(",")]
    Comma,
    #[regex(r#""([^"\\]|\\["\\bnfrt]|u[a-fA-F0-9]{4})*""#, |lex| lex.slice().to_owned())]
    String(String),
    /// Or regular expressions.
    #[regex(r"[a-zA-Z]+")]
    Word,
    #[token("true", |_| true)]
    #[token("false", |_| false)]
    Bool(bool),
    #[token("nil", |_| ())]
    Nil,
}
