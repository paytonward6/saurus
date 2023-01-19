use std::fs;
use std::io::{Error, Write};
use std::path::PathBuf;

use crate::tokenizer::parse::{Token, TokenKind};

pub fn write(contents: &Vec<Token>, path: &PathBuf) -> Result<(), Error> {
    let mut file = fs::File::create(path)?;
    for line in contents.iter() {
        let line = generate(line);
        if let Some(line) = line {
            write!(file, "{}\n", line)?;
        }
    }
    Ok(())
}

fn generate(token: &Token) -> Option<String> {
    match token.kind {
        TokenKind::FileStart => Some(format!("\\documentclass{{article}}\n\\begin{{document}}")),
        TokenKind::FileEnd => Some(format!("\\end{{document}}")),
        TokenKind::Heading(level) => match level {
            1 => Some(format!("\\section{{{}}}", token.contents.as_ref().unwrap())),
            2 => Some(format!(
                "\\subsection{{{}}}",
                token.contents.as_ref().unwrap()
            )),
            3 => Some(format!(
                "\\subsubsection{{{}}}",
                token.contents.as_ref().unwrap()
            )),
            _ => None,
        },
        TokenKind::BeginUnorderedList => Some(format!("\\begin{{itemize}}")),
        TokenKind::UnorderedListItem(_) => {
            Some(format!("    \\item {}", token.contents.as_ref().unwrap()))
        }
        TokenKind::EndUnorderedList => Some(format!("\\end{{itemize}}\n")),
        TokenKind::BeginOrderedList => Some(format!("\\begin{{enumerate}}")),
        TokenKind::OrderedListItem(_) => {
            Some(format!("    \\item {}", token.contents.as_ref().unwrap()))
        }
        TokenKind::EndOrderedList => Some(format!("\\end{{enumerate}}\n")),
        _ => None,
    }
}
