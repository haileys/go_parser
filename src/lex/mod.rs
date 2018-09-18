use loc::Loc;

mod scan;

use self::scan::Lexeme;

#[derive(Debug)]
pub enum LexError {
    UnexpectedChar(Loc),
    BadEscape(Loc),
    IllegalNewline(Loc),
    IllegalHexDigit(Loc),
    IllegalOctalValue(Loc),
    IllegalUnicodeValue(Loc),
}

#[derive(Debug)]
pub struct Token(Loc, TokenInfo);

#[derive(Debug)]
pub enum TokenInfo {
    Eof,
    // non-keyword barewords:
    Identifier(String),
    // keywords:
    Break,
    Default,
    Func,
    Interface,
    Select,
    Case,
    Defer,
    Go,
    Map,
    Struct,
    Chan,
    Else,
    Goto,
    Package,
    Switch,
    Const,
    Fallthrough,
    If,
    Range,
    Type,
    Continue,
    For,
    Import,
    Return,
    Var,
    // punctuation:
    Plus,
    Amp,
    PlusEq,
    AmpEq,
    AmpAmp,
    EqEq,
    NotEq,
    LParen,
    RParen,
    Minus,
    Pipe,
    MinusEq,
    PipeEq,
    PipePipe,
    Lt,
    LtEq,
    LBracket,
    RBracket,
    Star,
    Caret,
    StarEq,
    CaretEq,
    LtMinus,
    Gt,
    GtEq,
    LBrace,
    RBrace,
    Slash,
    LtLt,
    SlashEq,
    LtLtEq,
    PlusPlus,
    Eq,
    ColonEq,
    Comma,
    Semicolon,
    Percent,
    GtGt,
    PercentEq,
    GtGtEq,
    MinusMinus,
    Not,
    Ellipsis,
    Dot,
    Colon,
    AmpCaret,
    AmpCaretEq,
    // integer literals:
    DecInt(String),
    OctInt(String),
    HexInt(String),
    // float literals:
    Float(String),
    // imaginary literals:
    Imaginary(String),
    // rune literals:
    RuneBeg,
    RuneContents(char),
    RuneEnd,
    // string literals:
    StringBeg,
    StringContents(Vec<u8>),
    StringEnd,
}

pub fn lex(source: &str) -> Result<Vec<Token>, LexError> {
    let lexemes = scan::scan(source)?;

    // insert semicolons:
    let mut tokens = Vec::new();

    for lexeme in lexemes {
        match lexeme {
            Lexeme::Token(tok) => { tokens.push(tok); }
            Lexeme::Whitespace => {}
            Lexeme::Newline(loc) => {
                let insert = match tokens.last() {
                    Some(Token(_, TokenInfo::Identifier(_))) => true,
                    Some(Token(_, TokenInfo::DecInt(_))) => true,
                    Some(Token(_, TokenInfo::OctInt(_))) => true,
                    Some(Token(_, TokenInfo::HexInt(_))) => true,
                    Some(Token(_, TokenInfo::Float(_))) => true,
                    Some(Token(_, TokenInfo::Imaginary(_))) => true,
                    Some(Token(_, TokenInfo::RuneEnd)) => true,
                    Some(Token(_, TokenInfo::StringEnd)) => true,
                    _ => false,
                };

                if insert {
                    tokens.push(Token(loc, TokenInfo::Semicolon));
                }
            }
        }
    }

    Ok(tokens)
}
