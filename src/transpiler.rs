use std::fs;
use std::io::{Error, Write};
use std::path::PathBuf;

pub mod code_blocks;
pub mod generator;
pub mod lexer;
pub mod parser;
pub mod re;

pub fn run(file_str: &str, path: &PathBuf) {
    let mut lex = lexer::Lexer::new();
    lex.tokenize(&file_str);

    let mut parse = parser::Parser::from(lex);
    parse.run();

    write(path, parse).unwrap_or_else(|error| {
        println!("{}", error);
    });
}

fn write(path: &PathBuf, parser: parser::Parser) -> Result<(), Error> {
    let mut file = fs::File::create(path)?;
    write!(file, "{}\n", generator::documentclass())?;
    write!(
        file,
        "{}\n",
        generator::packages(parser.lexer.contains_code_block)
    )?;
    for line in parser.results.into_iter() {
        if let Some(line) = generator::generate_line(line) {
            write!(file, "{}\n", line)?;
        }
    }
    Ok(())
}
