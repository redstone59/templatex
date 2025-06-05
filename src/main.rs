use std::fs;

mod preprocessor;
use preprocessor::Preprocessor;

mod scanner;
use scanner::Scanner;

fn main() {
    let to_process = fs::read_to_string("test.tex")
        .expect("Expected file to exist.");

    let mut prep= Preprocessor::new(&to_process);

    for result in prep.preprocess() {
        let mut scanner = Scanner::new(&result);
        loop {
            let token  = scanner.scan_token();
            println!("{}", token);
            if token.type_ == scanner::TokenType::EndOfFile {
                break;
            }
        }
    }

    println!("done!");
}