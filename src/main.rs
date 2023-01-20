use std::fs;
use std::path::PathBuf;

use saurus::tokenizer::parse::Transpiler;
use saurus::tokenizer::re;

fn main() {
    let file_str = fs::read_to_string("./tests/notes.md").expect("Unable to read from file!");
    let transpiler = Transpiler::new();
    transpiler.run(&file_str, &PathBuf::from("./tests/TeX/main.tex"));

    let string = re::bold(&mut "** hi guys ** this is text **hi** this is other text".to_string());
    println!("{:?}", string);


}
