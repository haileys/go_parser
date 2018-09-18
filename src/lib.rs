// #[macro_use] extern crate lazy_static;

extern crate regex;
extern crate unicode_categories;

mod lex;
mod loc;

pub use lex::{scan, LexError, Token, TokenInfo, Lexeme};
