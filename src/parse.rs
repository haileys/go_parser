use std::iter::Peekable;

use ast::*;
use lex::{Token, TokenInfo};

#[derive(Debug)]
pub enum ParseError {
    UnexpectedToken(Token, &'static str),
}

pub struct Parser<I: Iterator<Item = Token>> {
    tokens: Peekable<I>
}

fn unexpected<T>(tok: Token, descr: &'static str) -> Result<T, ParseError> {
    Err(ParseError::UnexpectedToken(tok, descr))
}

macro_rules! match_next {
    ($(p:patt => { $code:expr })*) => {
        match self.next() {
            $($p => $code)*
            tok => return unexpected(tok, "??")
        }
    };
}

impl<I: Iterator<Item = Token>> Parser<I> {
    pub fn new(tokens: impl IntoIterator<IntoIter = I, Item = Token>) -> Self {
        Parser { tokens: tokens.into_iter().peekable() }
    }

    pub fn parse(mut self) -> Result<SourceFile, ParseError> {
        self.source_file()
    }

    fn peek(&mut self) -> Option<&TokenInfo> {
        self.tokens.peek().map(|Token(loc, info)| info)
    }

    fn next(&mut self) -> Token {
        self.tokens.next().unwrap()
    }

    fn source_file(&mut self) -> Result<SourceFile, ParseError> {
        let package_clause = if let Some(TokenInfo::Package) = self.peek() {
            let Token(pkg_loc, _) = self.next();
            match self.next() {
                Token(nam_loc, TokenInfo::Identifier(nam)) => {
                    let loc = pkg_loc.join(&nam_loc);
                    Some(PackageClause { loc, name: Id { loc: nam_loc, ident: nam } })
                }
                tok => return unexpected(tok, "expected identifier")
            }
        } else {
            None
        };

        unimplemented!()
    }
}
