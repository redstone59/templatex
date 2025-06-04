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

    fn get_surrounding_lines(&self, index: usize) -> String {
        let lines: Vec<String> = self.source
                                     .lines()
                                     .map(|x| x.to_string())
                                     .collect();
        
        let mut start_line = index - 2;
        if start_line < 0 { start_line = 0; }

        let mut end_line = index + 2;
        if end_line >= self.max { end_line = self.max - 1 }

        let mut to_display: Vec<String> = vec![];

        if start_line != 0 { to_display.push("   | ...".to_string()); }
        for i in start_line..=end_line {
            let to_push: String = format!("{:>3}: {}", i, lines[i]);
            if i == index {
                to_display.push(format!("\x1b[1;33m{}\x1b[0m", to_push));
            } else {
                to_display.push(to_push);
            }
        }
        if end_line != self.max { to_display.push("   | ...".to_string()); }
        
        to_display.join("\n")
    }

    fn err_string(&self, message: String, line_index: usize) -> String {
        format!(
            // '\x1b[1;31m' sets text to bold red.
            // '\x1b[0m' sets text to terminal default.
            "=====\n{}\n=====\n\x1b[1;31mError in preprocessing\x1b[0m: {}\n=====",
            self.get_surrounding_lines(line_index),
            message
        )
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
            
            if self.is_at_end() {
                panic!(
                    "{}",
                    self.err_string(
                        format!(
                            "Mismatched brackets for block starting at line {}, character {} to end of file.",
                            start_line,
                            start_index
                        ),
                        start_line
                    )
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