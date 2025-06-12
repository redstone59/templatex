use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum TokenType {
    // Brackets
    CurlyBraceOpen, CurlyBraceClose,
    SquareBraceOpen, SquareBraceClose,
    RoundBraceOpen, RoundBraceClose,

    // Operators
    Plus, Minus, SlashForward, Asterisk,
    Equal, EqualEqual,
    Bang, BangEqual,
    GreaterThan, GreaterEqual,
    LessThan, LessEqual,
    Ampersand, AmpersandDouble, 
    Pipe, PipeDouble,

    // Keywords
    Struct, Impl,
    For, While, Break, Continue,
    If, Else,
    Define, DashArrow,
    Let,
    True, False, None,
    Return, Echo,                         // yeah im just stealing this from php so what

    // Characters
    SlashBack, Semicolon, Comma,
    DoubleColon, Dot,                     // paamayim nekudotayim
    QuestionMark, Colon,

    // Literals
    Number, 
    String, StringLatex,
    Identifier,

    EndOfFile
}

pub struct Token<'a> {
    pub type_: TokenType,
    pub lexeme: &'a str,
    pub line: usize
}

impl<'a> Token<'a> {
    pub fn new(ty: TokenType, lex: &'a str, line: usize) -> Self {
        Self {
            type_: ty,
            lexeme: lex,
            line: line
        }
    }
}

impl<'a> Display for Token<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Token(TokenType::{:?}, \'{}\')", self.type_, self.lexeme)
    }
}