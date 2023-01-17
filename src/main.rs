use std::fs;
use std::error::Error;

use saurus::tokenizer::parse;

fn main() {
    let file_str = fs::read_to_string("./src/notes.txt").expect("Unable to read from file!");
    parse::run(&file_str);
}
