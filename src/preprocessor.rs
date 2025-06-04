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
            line: 1
        }
    }

    pub fn current(&self) -> Option<char> {
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

    pub fn peek(&self) -> Option<char> {
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

    pub fn next(&mut self) -> Option<char> {
        self.index += 1;
        if self.current() == Some('\n') {
            self.line += 1;
        }
        
        self.current()
    }

    pub fn is_at_end(&self) -> bool {
        self.index >= self.max
    }

    fn match_block(&mut self) -> String {
        let mut bracket_level: i32 = 1;
        let start_index: usize = self.index;

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
            
            if self.is_at_end() {
                panic!(
                    "Mismatched brackets for block starting at line {}, character {} to end of file.",
                    self.line,
                    start_index
                );
            }
        }
        self.next(); // Consume '}'

        // -2 to include '\%'
        self.source[start_index - 2..self.index].to_string()
    }

    pub fn preprocess(&mut self) -> Vec<String> {
        let mut resultant: Vec<String> = vec![];

        while !self.is_at_end() {
            if self.next() == Some('\\') && self.peek() == Some('%') {
                self.next();     // Consume '%'
                if self.peek() == Some('{') {
                    self.next(); // Consume '{'
                    resultant.push(self.match_block());
                }
            }
        }

        resultant
    }
}

impl Iterator for Preprocessor {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        self.next()
    }
}