#![allow(non_upper_case_globals)]
#![allow(unreachable_patterns)]
#![allow(unused_assignments)]
#![allow(unused_parens)]

%%{
    machine scan;

    newline             = '\n';
    unicode_char        = any - newline;
    unicode_letter      = [A-Za-z]; # TODO
    unicode_digit       = [0-9]; # TODO

    whitespace          = ' ' | '\t' | '\r';

    # Letters and digits
    letter              = unicode_letter | '_';
    decimal_digit       = [0-9];
    octal_digit         = [0-7];
    hex_digit           = [0-9A-Fa-f];
    hex_digit_err       = hex_digit
                        | (any - hex_digit) %{ return Err(LexError::IllegalHexDigit(p - 1, p)); };

    # Identifiers
    identifier          = letter (letter | unicode_digit)*;

    # Integer literals
    decimal_lit         = [1-9] decimal_digit*;
    octal_lit           = '0' octal_digit*;
    hex_lit             = '0' [xX] hex_digit+;

    # Floating point literals
    decimals            = decimal_digit+;
    exponent            = [eE] [\-+]? decimals;
    float_lit           = (decimals '.' decimals? exponent?)
                        | decimals exponent
                        | ('.' decimals exponent?)
                        ;

    # Imaginary literals
    imaginary_lit       = (decimals | float_lit) 'i';

    # Rune literals
    action escaped_char {
        self.value = match &self.data[tm..p] {
            "a"  => 0x07,
            "b"  => 0x08,
            "f"  => 0x0c,
            "n"  => 0x0a,
            "r"  => 0x0d,
            "t"  => 0x09,
            "v"  => 0x0b,
            "\\" => 0x5c,
            "'"  => 0x27,
            "\"" => 0x22,
            _    => unreachable!(),
        };
    }

    action hex_value {
        self.value = u32::from_str_radix(&self.data[tm..p], 16).expect("rune_hex_value should always succeed");
    }

    action octal_value {
        self.value = u32::from_str_radix(&self.data[tm..p], 8)
            .map_err(|_| LexError::IllegalOctalValue(tm, self.te))?;
    }

    rune_uni_char       = unicode_char %{ self.value = self.getkey(self.ts); };
    escaped_char        = "\\"  %{ tm = p } [abfnrtv\\'"]       %escaped_char;
    big_u_value         = "\\U" %{ tm = p } hex_digit_err{8}    %hex_value;
    little_u_value      = "\\u" %{ tm = p } hex_digit_err{4}    %hex_value;
    hex_byte_value      = "\\x" %{ tm = p } hex_digit_err{2}    %hex_value;
    octal_byte_value    = "\\"  %{ tm = p } octal_digit{3}      %octal_value;

    # Scanners
    rune := |*
        newline             => { return Err(LexError::IllegalNewline(self.ts, self.te)); };
        "'"                 => { return Err(LexError::UnexpectedChar(self.ts, self.te)); };
        rune_uni_char |
        little_u_value |
        big_u_value |
        escaped_char |
        octal_byte_value |
        hex_byte_value      => { let c = self.u32_as_char()?; self.token(TokenInfo::RuneContents(c)); fnext rune_end; };
        zlen                => { return Err(LexError::UnterminatedRune); };
    *|;

    rune_end := |*
        "'"                 => { self.token(TokenInfo::RuneEnd); fnext main; };
        newline             => { return Err(LexError::IllegalNewline(self.ts, self.te)); };
        any                 => { return Err(LexError::UnexpectedChar(self.ts, self.te)); };
        zlen                => { return Err(LexError::UnterminatedRune); };
    *|;

    string_uni_esc      = (escaped_char | big_u_value | little_u_value) - "\'"; # single quote escaped illegal in string lits
    string_byte_esc     = hex_byte_value | octal_byte_value;

    interp_string := |*
        "\""                => { self.token(TokenInfo::StringEnd); fnext main; };
        newline             => { return Err(LexError::IllegalNewline(self.ts, self.te)); };
        (any - [\\"\n])*    => { let bytes = self.bytes(); self.token(TokenInfo::StringContents(bytes)); };
        string_uni_esc      => { let c = self.u32_as_char()?.to_string(); self.token(TokenInfo::StringContents(c.as_bytes().to_vec())); };
        # escaped bytes are treated as raw bytes in strings, as opposed to code points like in runes:
        string_byte_esc     => { let bytes = vec![self.value as u8]; self.token(TokenInfo::StringContents(bytes)); };
        "\\"                => { return Err(LexError::BadEscape(self.ts, self.te)); };
    *|;

    raw_string := |*
        (any - "`")*    => { let bytes = self.bytes(); self.token(TokenInfo::StringContents(bytes)); };
        "`"             => { self.token(TokenInfo::StringEnd); fnext main; };
    *|;

    comment_line := |*
        newline         => { fnext main; };
        any             => {};
    *|;

    comment_multiline := |*
        "*/"            => { fnext main; };
        any             => {};
        zlen            => { return Err(LexError::UnterminatedComment); };
    *|;

    main := |*
        # whitespace:
        newline         => { let lexeme = Lexeme::Newline(self.ts, self.te); self.lexeme(lexeme) };
        whitespace      => { self.lexeme(Lexeme::Whitespace) };

        # comments:
        "//"            => { fnext comment_line; };
        "/*"            => { fnext comment_multiline; };

        # keywords:
        "break"         => { self.token(TokenInfo::Break) };
        "default"       => { self.token(TokenInfo::Default) };
        "func"          => { self.token(TokenInfo::Func) };
        "interface"     => { self.token(TokenInfo::Interface) };
        "select"        => { self.token(TokenInfo::Select) };
        "case"          => { self.token(TokenInfo::Case) };
        "defer"         => { self.token(TokenInfo::Defer) };
        "go"            => { self.token(TokenInfo::Go) };
        "map"           => { self.token(TokenInfo::Map) };
        "struct"        => { self.token(TokenInfo::Struct) };
        "chan"          => { self.token(TokenInfo::Chan) };
        "else"          => { self.token(TokenInfo::Else) };
        "goto"          => { self.token(TokenInfo::Goto) };
        "package"       => { self.token(TokenInfo::Package) };
        "switch"        => { self.token(TokenInfo::Switch) };
        "const"         => { self.token(TokenInfo::Const) };
        "fallthrough"   => { self.token(TokenInfo::Fallthrough) };
        "if"            => { self.token(TokenInfo::If) };
        "range"         => { self.token(TokenInfo::Range) };
        "type"          => { self.token(TokenInfo::Type) };
        "continue"      => { self.token(TokenInfo::Continue) };
        "for"           => { self.token(TokenInfo::For) };
        "import"        => { self.token(TokenInfo::Import) };
        "return"        => { self.token(TokenInfo::Return) };
        "var"           => { self.token(TokenInfo::Var) };

        # punctuation:
        "+"             => { self.token(TokenInfo::Plus) };
        "&"             => { self.token(TokenInfo::Amp) };
        "+="            => { self.token(TokenInfo::PlusEq) };
        "&="            => { self.token(TokenInfo::AmpEq) };
        "&&"            => { self.token(TokenInfo::AmpAmp) };
        "=="            => { self.token(TokenInfo::EqEq) };
        "!="            => { self.token(TokenInfo::NotEq) };
        "("             => { self.token(TokenInfo::LParen) };
        ")"             => { self.token(TokenInfo::RParen) };
        "-"             => { self.token(TokenInfo::Minus) };
        "|"             => { self.token(TokenInfo::Pipe) };
        "-="            => { self.token(TokenInfo::MinusEq) };
        "|="            => { self.token(TokenInfo::PipeEq) };
        "||"            => { self.token(TokenInfo::PipePipe) };
        "<"             => { self.token(TokenInfo::Lt) };
        "<="            => { self.token(TokenInfo::LtEq) };
        "["             => { self.token(TokenInfo::LBracket) };
        "]"             => { self.token(TokenInfo::RBracket) };
        "*"             => { self.token(TokenInfo::Star) };
        "^"             => { self.token(TokenInfo::Caret) };
        "*="            => { self.token(TokenInfo::StarEq) };
        "^="            => { self.token(TokenInfo::CaretEq) };
        "<-"            => { self.token(TokenInfo::LtMinus) };
        ">"             => { self.token(TokenInfo::Gt) };
        ">="            => { self.token(TokenInfo::GtEq) };
        "{"             => { self.token(TokenInfo::LBrace) };
        "}"             => { self.token(TokenInfo::RBrace) };
        "/"             => { self.token(TokenInfo::Slash) };
        "<<"            => { self.token(TokenInfo::LtLt) };
        "/="            => { self.token(TokenInfo::SlashEq) };
        "<<="           => { self.token(TokenInfo::LtLtEq) };
        "++"            => { self.token(TokenInfo::PlusPlus) };
        "="             => { self.token(TokenInfo::Eq) };
        ":="            => { self.token(TokenInfo::ColonEq) };
        ","             => { self.token(TokenInfo::Comma) };
        ";"             => { self.token(TokenInfo::Semicolon) };
        "%"             => { self.token(TokenInfo::Percent) };
        ">>"            => { self.token(TokenInfo::GtGt) };
        "%="            => { self.token(TokenInfo::PercentEq) };
        ">>="           => { self.token(TokenInfo::GtGtEq) };
        "--"            => { self.token(TokenInfo::MinusMinus) };
        "!"             => { self.token(TokenInfo::Not) };
        "..."           => { self.token(TokenInfo::Ellipsis) };
        "."             => { self.token(TokenInfo::Dot) };
        ":"             => { self.token(TokenInfo::Colon) };
        "&^"            => { self.token(TokenInfo::AmpCaret) };
        "&^="           => { self.token(TokenInfo::AmpCaretEq) };

        # identifiers:
        identifier      => { self.token_val(TokenInfo::Identifier) };

        # numeric literals:
        decimal_lit     => { self.token_val(TokenInfo::DecInt) };
        octal_lit       => { self.token_val(TokenInfo::OctInt) };
        hex_lit         => { self.token_val(TokenInfo::HexInt) };
        float_lit       => { self.token_val(TokenInfo::Float) };
        imaginary_lit   => { self.token_val(TokenInfo::Imaginary) };

        # rune literals:
        "'"             => { self.token(TokenInfo::RuneBeg); fnext rune; };

        # string literals:
        "\""            => { self.token(TokenInfo::StringBeg); fnext interp_string; };
        "`"             => { self.token(TokenInfo::StringBeg); fnext raw_string; };

        # termination:
        any             => { return Err(LexError::UnexpectedChar(self.ts, self.te)) };
        zlen            => { self.token(TokenInfo::Eof) };
    *|;
}%%

%% access self.;
%% alphtype u32;
%% getkey self.getkey(p);

%% write data;

use loc::Loc;
use super::{Token, TokenInfo, LexError};

#[derive(Debug)]
pub enum Lexeme {
    Token(Token),
    Whitespace,
    Newline(usize, usize),
}

struct Scanner<'a> {
    lexemes: Vec<Lexeme>,
    value: u32,

    data: &'a str,
    cs: usize,
    ts: usize,
    te: usize,
    act: usize,
}

impl<'a> Scanner<'a> {
    fn exec(mut self) -> Result<Vec<Lexeme>, LexError> {
        %% write init;

        let mut tm = ::std::usize::MAX;
        let mut p = 0;
        let pe = self.data.len();
        let eof = pe;

        while self.cs != scan_error && p < pe {
            %% write exec;
        }

        Ok(self.lexemes)
    }

    fn getkey(&self, idx: usize) -> u32 {
        self.data[idx..].chars().next().unwrap() as u32
    }

    fn bytes(&self) -> Vec<u8> {
        self.data[self.ts..self.te].as_bytes().to_vec()
    }

    fn string(&self) -> String {
        self.data[self.ts..self.te].to_string()
    }

    fn lexeme(&mut self, lexeme: Lexeme) {
        self.lexemes.push(lexeme)
    }

    fn token(&mut self, tok: TokenInfo) {
        let start = Loc::new(self.ts..self.ts);
        let end = Loc::new(self.te..self.te);
        let lexeme = Lexeme::Token((start, tok, end));
        self.lexeme(lexeme);
    }

    fn token_val(&mut self, mktok: impl Fn(String) -> TokenInfo) {
        let tok = mktok(self.string());
        self.token(tok)
    }

    fn u32_as_char(&self) -> Result<char, LexError> {
        let codepoint = self.value;

        match ::std::char::from_u32(codepoint) {
            None => Err(LexError::IllegalUnicodeValue(self.ts, self.te)),
            Some(c) => Ok(c),
        }
    }
}

pub fn scan(input: &str) -> Result<Vec<Lexeme>, LexError> {
    Scanner {
        lexemes: Vec::new(),
        value: 0,
        data: input,
        cs: 0,
        ts: 0,
        te: 0,
        act: 0,
    }.exec()
}
