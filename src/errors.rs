pub struct ErrorStruct {
    pub source: String,
    pub max: usize
}

impl ErrorStruct {
    fn get_surrounding_lines(&self, index: usize) -> String {
        let lines: Vec<String> = self.source
                                     .lines()
                                     .map(|x| x.to_string())
                                     .collect();
        
        let start_line = if index < 2 { 0 } else { index - 2 };

        let mut end_line = index + 2;
        if end_line >= self.max { end_line = self.max - 1 }

        let mut to_display: Vec<String> = vec![];

        if start_line != 0 { to_display.push("   | ...".to_string()); }
        for i in start_line..=end_line {
            let to_push: String = format!("{:>3}: {}", i + 1, lines[i]);
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
            "=====\n{}\n=====\n\x1b[1;31merror\x1b[0m: {}\n=====",
            self.get_surrounding_lines(line_index),
            message
        )
    }

    /// Panic and print an error message.
    pub fn throw(&self, message: String, line_index: usize) -> ! {
        panic!(
            "{}",
            self.err_string(
                    message, 
                    line_index
                )
        )
    }

    /// Panic and print an error message if `predicate()` returns `true`.
    pub fn throw_if<F>(&self, predicate: F, message: String, line_index: usize) -> ()
    where F: FnOnce() -> bool 
    {
        if predicate() {
            self.throw(message, line_index);
        }
    }
}

/// Allows the struct to convert into an `ErrorStruct`, meaning that it can print pretty error messages on panicking.
pub trait Errorable {
    fn to_error(&self) -> ErrorStruct;
}