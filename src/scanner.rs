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

    fn identifier(&mut self) -> Token {
        while self.peek().is_alphanumeric() || self.peek() == '_' {
            self.advance();
        }

        self.make_token(self.identifier_type())
    }

    fn identifier_type(&self) -> TokenType {
        let current_identifier: &str = &self.source[self.start..self.current];
        match current_identifier {
            "struct" => TokenType::Struct,
            "implement" => TokenType::Impl,
            "for" => TokenType::For,
            "while" => TokenType::While,
            "break" => TokenType::Break,
            "continue" => TokenType::Continue,
            "if" => TokenType::If,
            "else" => TokenType::Else,
            "def" => TokenType::Define,
            "let" => TokenType::Let,
            "true" => TokenType::True,
            "false" => TokenType::False,
            "none" => TokenType::None,
            "return" => TokenType::Return,
            "echo" => TokenType::Echo,
            _ => TokenType::Identifier
        }
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

    fn number(&mut self) -> Token {
        let mut radix: u32 = 10;
        if self.peek_previous() == '0' {
            match self.peek() {
                'x' | 'X' => {
                    radix = 16;
                    self.advance();
                },
                'o' | 'O' => {
                    radix = 8;
                    self.advance();
                },
                'b' | 'B' => {
                    radix = 2;
                    self.advance();
                },
                _ => radix = 10
            }
        }

        while self.peek().is_digit(16) {
            self.to_error()
                .throw_if(
                    || !self.peek().is_digit(radix),
                    format!(
                        "Unexpected digit '{}' in number literal with radix {}.",
                        self.peek(),
                        radix
                    ),
                    self.line);
            self.advance();
        }

        // Decimal and exponent notation only applies to base 10 number literals.
        if radix == 10 {
            if self.peek() == '.' && self.peek_next().is_digit(10) {
                self.advance();
                while self.peek().is_digit(10) {
                    self.advance();
                }
            }
            if self.peek() == 'e' && self.peek_next().is_digit(10) {
                self.advance();
                while self.peek().is_digit(10) {
                    self.advance();
                }
            }
        }

        self.make_token(TokenType::Number)
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

    fn peek_next(&self) -> char {
        if self.current + 1 == self.max {
            return '\0';
        }
        
        self.source
            .chars()
            .nth(self.current + 1)
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

        if next.is_alphabetic() || next == '_' {
            return self.identifier()
        }

        if next.is_digit(10) {
            return self.number()
        }

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
            },

            '"' => self.string(false),

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

    fn string(&mut self, raw: bool) -> Token {
        self.advance(); // Consume '"'
        let start_index: usize = self.current;
        let start_line: usize = self.line;

        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        self.to_error()
            .throw_if(
                || self.is_at_end(),
                format!(
                    "Unterminated string from line {}, character {}, to end of file.",
                    start_line + 1,
                    start_index
                ),
                start_line
            );
        
        self.advance(); // Consume ending '"'.
        self.make_token(TokenType::String)
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