#[path ="errors.rs"] mod errors;
use errors::Errorable;

pub struct Preprocessor {
    pub source: String,
    pub index: usize,
    max: usize,
    line: usize
}

impl Preprocessor {
    pub fn new(source: &str) -> Self {
        Self {
            source: source.to_string(),
            index: 1,
            max: source.chars().count(),
            line: 0
        }
    }
    
    pub fn preprocess(&mut self) -> Vec<String> {
        let mut resultant: Vec<String> = vec![];

        while !self.is_at_end() {
            if self.next() == Some('\\') && self.matches('%') {
                self.next();     // Consume '%'
                if self.matches('{') {
                    self.next(); // Consume '{'
                    resultant.push(self.match_block());
                }
            }
        }

        resultant
    }

    fn current(&self) -> Option<char> {
        if self.is_at_end() {
            return None;
        }

        Some(
            self.source
                .chars()
                .nth(self.index)
                .unwrap()
        )
    }

    fn peek(&self) -> Option<char> {
        if self.index + 1 >= self.max {
            return None;
        }

        Some(
            self.source
                .chars()
                .nth(self.index + 1)
                .unwrap()
        )
    }

    fn matches(&self, to_match: char) -> bool {
        if let Some(next) = self.peek() {
            return next == to_match;
        }

        return false;
    }

    fn next(&mut self) -> Option<char> {
        self.index += 1;
        if self.current() == Some('\n') {
            self.line += 1;
        }
        
        self.current()
    }

    fn is_at_end(&self) -> bool {
        self.index >= self.max
    }

    fn match_block(&mut self) -> String {
        // Starts at 1 since a '{' was consumed before this was called.
        let mut bracket_level: i32 = 1;
        let start_index: usize = self.index;
        let start_line: usize = self.line;

        while bracket_level != 0 {
            if let Some(current) = self.next() {
                match current {
                    '{' => bracket_level += 1,
                    '}' => bracket_level -= 1,
                    _ => ()
                }
                println!("index {}: {}, bracket level {}", self.index, current, bracket_level);
                continue;
            }
            
            self.to_error()
                .throw_if(
                    || self.is_at_end(), // theres gotta be a better way to do this
                    format!(
                        "Mismatched brackets for block starting at line {}, character {} to end of file.",
                        start_line + 1, // zero-indexing :D
                        start_index
                    ),
                    start_line
                );
        }
        self.next(); // Consume '}'

        // -2 to include '\%'
        self.source[start_index - 2..self.index].to_string()
    }
}

impl Iterator for Preprocessor {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        self.next()
    }
}

impl Errorable for Preprocessor {
    fn to_error(&self) -> errors::ErrorStruct {
        errors::ErrorStruct {
            source: self.source.clone(),
            max: self.max
        }
    }
}