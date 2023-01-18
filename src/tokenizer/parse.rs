use std::collections::VecDeque;
use std::fmt;
use std::path::PathBuf;
pub fn run(file_str: &str, path: &PathBuf) {
    let tokens = tokenize(file_str);
    latexer::write(&tokens, path).unwrap_or_else(|error| {
        println!("{}ahhhh", error);
    });
}

#[derive(Debug, PartialEq, Eq)]
pub enum TokenKind {
    FileStart,
    FileEnd,
    Heading(usize),
    BeginList,
    List(char),
    EndList,
    Blank,
}

#[derive(Debug)]
pub struct Token {
    pub contents: Option<String>,
    pub kind: TokenKind,
    pub line_num: usize,
}

impl Token {
    fn new(contents: Option<String>, kind: TokenKind, line_num: usize) -> Self {
        Token {
            contents: contents,
            kind: kind,
            line_num: line_num,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.contents.as_ref().unwrap())
    }
}

fn tokenize(file_str: &str) -> Vec<Token> {
    type Kind = TokenKind;
    let mut tokens: Vec<Token> = Vec::new();
    let mut stack: VecDeque<TokenKind> = VecDeque::new();

    stack.push_back(Kind::FileStart);
    tokens.push(Token::new(None, Kind::FileStart, usize::MIN));

    for (i, line) in file_str.lines().enumerate() {
        let line = line.to_string();
        if re::heading(&line) {
            // TODO: ONLY PUTTING ONE FOR LEVEL 1
            tokens.push(Token::new(Some(line), Kind::Heading(1), i));
        } else if re::list(&line) {
            if let Some(last) = stack.back() {
                if last != &Kind::BeginList {
                    tokens.push(Token::new(None, Kind::BeginList, i));
                    stack.push_back(Kind::BeginList);
                }
            }
            // TODO: ONLY PUTTING '-' FOR NOW
            tokens.push(Token::new(Some(line), Kind::List('-'), i));
        } else if re::blank(&line) {
            match stack.back() {
                Some(Kind::BeginList) => {
                    tokens.push(Token::new(None, Kind::EndList, i));
                    stack.push_back(Kind::EndList)
                }
                _ => (),
            }
        }
    }

    match stack.back() {
        Some(Kind::FileStart) => tokens.push(Token::new(None, Kind::FileEnd, usize::MIN)),
        Some(Kind::BeginList) => tokens.push(Token::new(None, Kind::EndList, usize::MAX - 1)),
        _ => (),
    }

    stack.push_back(Kind::FileEnd);
    tokens.push(Token::new(None, Kind::FileEnd, usize::MAX));
    tokens
}

mod re {
    use regex::Regex;
    pub fn heading(line: &str) -> bool {
        let re: Regex = Regex::new(r"^\s*#").unwrap();
        re.is_match(line)
    }

    pub fn list(line: &str) -> bool {
        let re: Regex = Regex::new(r"^\s*[\-\+]").unwrap();
        re.is_match(line)
    }

    pub fn blank(line: &str) -> bool {
        line.is_empty()
    }
}

mod utils {
    pub fn line_count(file_str: &str) -> usize {
        file_str.lines().into_iter().count()
    }
}

mod latexer {
    use crate::tokenizer::parse::{Token, TokenKind};
    use std::fs;
    use std::io::{Error, Write};
    use std::path::PathBuf;

    pub fn write(contents: &Vec<Token>, path: &PathBuf) -> Result<(), Error> {
        let mut file = fs::File::create(path)?;
        for line in contents.iter() {
            let line = out(line);
            if let Some(line) = line {
                write!(file, "{}\n", line)?;
            }
        }
        Ok(())
    }

    fn out(token: &Token) -> Option<String> {
        match token.kind {
            TokenKind::FileStart => {
                Some(format!("\\documentclass{{article}}\n\\begin{{document}}"))
            }
            TokenKind::FileEnd => Some(format!("\\end{{document}}")),
            TokenKind::Heading(_) => Some(format!("\\section{{}}")),
            TokenKind::BeginList => Some(format!("\\begin{{itemize}}")),
            TokenKind::List(_) => Some(format!("\t\\item{}", token.contents.as_ref().unwrap())),
            TokenKind::EndList => Some(format!("\\end{{itemize}}\n")),
            _ => None,
        }
    }
}
