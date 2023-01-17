use std::fs;
use std::error::Error;

fn heading(line: &str) -> bool {
    line.starts_with("#")
}

fn main() {
    let foo = fs::read_to_string("./src/notes.txt").expect("Unable to read from file!");
    for line in foo.lines() {
        if heading(line) {
            println!("{line}");
        }
    }
}
