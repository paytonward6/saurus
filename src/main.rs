use std::error::Error;
use std::fs;
use std::path::PathBuf;

use saurus::tokenizer::parse;

fn main() {
    let file_str = fs::read_to_string("./tests/notes.md").expect("Unable to read from file!");
    parse::run(&file_str, &PathBuf::from("./tests/TeX/main.tex"));
}
