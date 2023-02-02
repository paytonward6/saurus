use std::fs;
use std::path::PathBuf;

use saurus::transpiler;
use saurus::transpiler::{generator, lexer, parser};

use clap::{arg, command, value_parser};

fn main() {
    let matches = command!()
        .arg(
            arg!(
                -i --input <FILE> "Sets a custom input file"
            )
            .required(true)
            .value_parser(value_parser!(PathBuf)),
        )
        .arg(
            arg!(
                -o --output <FILE> "Sets a custom output file"
            )
            .required(true)
            .value_parser(value_parser!(PathBuf)),
        )
        .get_matches();

    let input = matches.get_one::<PathBuf>("input").unwrap();
    let output = matches.get_one::<PathBuf>("output").unwrap();

    //let transpiler = Transpiler::new();
    let file_str = fs::read_to_string(input).expect("Unable to read from file!");
    //transpiler.run(&file_str, &PathBuf::from(output));

    //    let mut lex = lexer::Lexer {
    //    tokens: vec![
    //        (
    //            lexer::Token::FileStart,
    //            None,
    //        ),
    //        (
    //            lexer::Token::Heading,
    //            Some(
    //                "# Heading 1".to_string(),
    //            ),
    //        ),
    //        (
    //            lexer::Token::Blank,
    //            None,
    //        ),
    //        (
    //            lexer::Token::UnorderedList,
    //            Some(
    //                "- item 1".to_string(),
    //            ),
    //        ),
    //        (
    //            lexer::Token::UnorderedList,
    //            Some(
    //                "- item 2".to_string(),
    //            ),
    //        ),
    //        (
    //            lexer::Token::FileEnd,
    //            None,
    //        ),
    //    ],
    //    number_of_lines: 4,
    //};

    transpiler::run(&file_str, output);
}
