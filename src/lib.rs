mod ast;
mod lex;
mod loc;
mod parse;

pub use lex::{lex, LexError, Token, TokenInfo};
pub use parse::{ParseError};

pub fn parse(tokens: impl IntoIterator<Item = Token>) -> Result<ast::SourceFile, ParseError> {
    parse::Parser::new(tokens).parse()
}
