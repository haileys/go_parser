use ast::SourceFile;
use lex::{Token, TokenInfo};
use loc::Loc;

use self::grammar::SourceFileParser;

use lalrpop_util;

lalrpop_mod!(grammar, "/parse/grammar.rs");

#[derive(Debug)]
pub enum ParseError {
    Lalrpop(lalrpop_util::ParseError<Loc, TokenInfo, &'static str>),
}

pub fn parse(tokens: Vec<Token>) -> Result<SourceFile, ParseError> {
    let parser = SourceFileParser::new();
    parser.parse(tokens).map_err(ParseError::Lalrpop)
}
