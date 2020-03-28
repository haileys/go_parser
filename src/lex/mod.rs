mod scan;

use loc::Loc;
use self::scan::Lexeme;

#[derive(Debug)]
pub enum LexError {
    UnexpectedChar(usize, usize),
    UnterminatedComment,
    UnterminatedString,
    UnterminatedRune,
    BadEscape(usize, usize),
    IllegalNewline(usize, usize),
    IllegalHexDigit(usize, usize),
    IllegalOctalValue(usize, usize),
    IllegalUnicodeValue(usize, usize),
}

pub type Token = (Loc, TokenInfo, Loc);

#[derive(Debug, PartialEq, Eq)]
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
            Lexeme::Newline(s, e) => {
                let insert = match tokens.last() {
                    Some((_, TokenInfo::Identifier(_), _)) => true,
                    Some((_, TokenInfo::DecInt(_), _)) => true,
                    Some((_, TokenInfo::OctInt(_), _)) => true,
                    Some((_, TokenInfo::HexInt(_), _)) => true,
                    Some((_, TokenInfo::Float(_), _)) => true,
                    Some((_, TokenInfo::Imaginary(_), _)) => true,
                    Some((_, TokenInfo::RuneEnd, _)) => true,
                    Some((_, TokenInfo::StringEnd, _)) => true,
                    _ => false,
                };

                if insert {
                    tokens.push((Loc::new(s..s), TokenInfo::Semicolon, Loc::new(e..e)));
                }
            }
        }
    }

    Ok(tokens)
}
