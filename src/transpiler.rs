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

    let mut parse = parser::Parser::new();
    parse.run(lex);

    write(path, parse).unwrap_or_else(|error| {
        println!("{}", error);
    });
}

fn write(path: &PathBuf, parser: parser::Parser) -> Result<(), Error> {
    let mut file = fs::File::create(path)?;
    writeln!(file, "{}", generator::documentclass())?;
    writeln!(
        file,
        "{}",
        generator::packages(parser.contains_code_block)
    )?;
    for line in parser.results.into_iter() {
        if let Some(line) = generator::generate_line(line) {
            writeln!(file, "{}", line)?;
        }
    }
    Ok(())
}
