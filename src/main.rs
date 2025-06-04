use std::fs;

mod preprocessor;
use preprocessor::Preprocessor;

fn main() {
    let to_process = fs::read_to_string("test.tex")
        .expect("Expected file to exist.");

    let mut prep= Preprocessor::new(&to_process);

    for result in prep.preprocess() {
        println!("match:\n{}", result);
    }

    println!("done!");
}