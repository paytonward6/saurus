use std::fs;
use std::path::PathBuf;

use saurus::transpiler::Transpiler;

fn main() {
    let file_str = fs::read_to_string("./tests/notes.md").expect("Unable to read from file!");
    let transpiler = Transpiler::new();
    transpiler.run(&file_str, &PathBuf::from("./tests/TeX/main.tex"));
}
