#[path="tokens.rs"] mod tokens;
pub use tokens::{ Token, TokenType };

#[path="errors.rs"] mod errors;
use errors::Errorable;

pub struct Scanner {
    pub source: String,
    pub start: usize,
    pub current: usize,
    line: usize,
    max: usize
}

impl Scanner {
    pub fn new(source: &str) -> Self {
        Self {
            source: source.to_string(),
            start: 3,                       // ignore leading '\%{'
            current: 3,
            line: 0,
            max: source.chars().count() - 1 // ignore ending '}'
        }
    }

    fn advance(&mut self) -> Option<char> {
        self.current += 1;
        self.source.chars().nth(self.current - 1)
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.max
    }

    fn make_token(&self, type_: TokenType) -> Token {
        Token {
            type_: type_,
            lexeme: &self.source[self.start..self.current],
            line: self.line
        }
    }

    fn matches(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        let current: char = self.source
                                .chars()
                                .nth(self.current)
                                .unwrap();
        if current != expected {
            return false;
        }
        self.current += 1;

        true
    }

    fn peek(&self) -> char {
        self.source
            .chars()
            .nth(self.current)
            .unwrap()
    }

    fn peek_previous(&self) -> char {
        if self.current == 0 {
            return '\0';
        }

        self.source
            .chars()
            .nth(self.current - 1)
            .unwrap()
    }

    pub fn scan_token(&mut self) -> Token {
        self.skip_whitespace();
        self.start = self.current;

        if self.is_at_end() {
            return Token::new(
                TokenType::EndOfFile, 
                "", 
                self.line
            )
        }

        let next: char = self.advance().unwrap();

        match next {
            '{' => self.make_token(TokenType::CurlyBraceOpen),
            '}' => self.make_token(TokenType::CurlyBraceClose),
            '[' => self.make_token(TokenType::SquareBraceOpen),
            ']' => self.make_token(TokenType::SquareBraceClose),
            '(' => self.make_token(TokenType::RoundBraceOpen),
            ')' => self.make_token(TokenType::RoundBraceClose),

            '+' => self.make_token(TokenType::Plus),
            '-' => {
                if self.matches('>') {
                    self.make_token(TokenType::DashArrow)
                } else {
                    self.make_token(TokenType::Minus)
                }
            },
            '/' => self.make_token(TokenType::SlashForward),
            '*' => self.make_token(TokenType::Asterisk),
            '=' => {
                if self.matches('=') {
                    self.make_token(TokenType::EqualEqual)
                } else {
                    self.make_token(TokenType::Equal)
                }
            },
            '!' => {
                if self.matches('=') {
                    self.make_token(TokenType::BangEqual)
                } else {
                    self.make_token(TokenType::Bang)
                }
            },
            '>' => {
                if self.matches('=') {
                    self.make_token(TokenType::GreaterEqual)
                } else {
                    self.make_token(TokenType::GreaterThan)
                }
            },
            '<' => {
                if self.matches('=') {
                    self.make_token(TokenType::LessEqual)
                } else {
                    self.make_token(TokenType::LessThan)
                }
            },
            '&' => {
                if self.matches('&') {
                    self.make_token(TokenType::AmpersandDouble)
                } else {
                    self.make_token(TokenType::Ampersand)
                }
            },
            '|' => {
                if self.matches('|') {
                    self.make_token(TokenType::PipeDouble)
                } else {
                    self.make_token(TokenType::Pipe)
                }
            },

            '\\' => self.make_token(TokenType::SlashBack),
            ';' => self.make_token(TokenType::Semicolon),
            ',' => self.make_token(TokenType::Comma),
            '.' => self.make_token(TokenType::Dot),
            '?' => self.make_token(TokenType::QuestionMark),
            ':' => {
                if self.matches(':') {
                    self.make_token(TokenType::DoubleColon)
                } else {
                    self.make_token(TokenType::Colon)
                }
            }

            _ => self.to_error()
                     .throw(
                        format!(
                            "Unexpected character '{}' at line {}, characters {} to {}.",
                            next,
                            self.line + 1,
                            self.start,
                            self.current
                        ),
                        self.line
                     )
        }
    }

    fn skip_whitespace(&mut self) -> () {
        loop {
            _ = match self.peek() {
                ' ' | '\r' | '\t' => { self.advance(); },
                '\n' => {
                    self.line += 1;
                    self.advance();
                },
                _ => {
                    break;
                }
            }
        }
        return;
    }
}

impl Errorable for Scanner {
    fn to_error(&self) -> errors::ErrorStruct {
        errors::ErrorStruct {
            source: self.source.clone(),
            max: self.max
        }
    }
}