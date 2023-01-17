use std::fmt;
pub fn run(file_str: &str) {
    latexer::write(&tokenize(file_str)).unwrap_or_else(|error| {println!("{}ahhhh", error);});
}

#[derive(Debug)]
pub enum TokenKind {
    HEADING(usize),
    LIST(char),
}

#[derive(Debug)]
pub struct Token {
    pub contents: String,
    pub kind: TokenKind,
    pub line_num: usize,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.contents)
    }
}

fn tokenize(file_str: &str) -> Vec<Token> {
    type Kind = TokenKind;
    let mut tokens: Vec<Token> = Vec::new(); 
    for (i, line) in file_str.lines().enumerate() {
        let line = line.to_string();
        if re::heading(&line) {
            // TODO: ONLY PUTTING ONE FOR LEVEL 1
            tokens.push(Token {contents: line, kind: Kind::HEADING(1), line_num: i});
        }
        else if re::list(&line) {
            // TODO: ONLY PUTTING '-' FOR NOW
            tokens.push(Token {contents: line, kind: Kind::LIST('-'), line_num: i});
        }
    }
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
}


mod utils {
    pub fn line_count(file_str: &str) -> usize {
        file_str.lines().into_iter().count()
    }
}

mod latexer{
    use crate::tokenizer::parse::{Token, TokenKind};
    use std::io::{Error, Write};
    use std::fs;

    static INTRO: &str = "\\documentclass{article}\n\\begin{document}";
    static OUTRO: &str = "\\end{document}";

    pub fn write(contents: &Vec<Token>) -> Result<(), Error> {
        let mut file = fs::File::create("test.tex")?;
        write!(file, "{}\n", INTRO)?;
        for line in contents.iter() {
            write!(file, "{}\n", out(line))?;
        }
        write!(file, "{}\n", OUTRO)?;
        Ok(())
    }

    fn out(token: &Token) -> String {
        match token.kind {
            TokenKind::HEADING(_) => format!("\\section{{}}"),
            TokenKind::LIST(_) => format!("\\begin{{itemize}}\n \t\\item{} \n\\end{{itemize}}", token.contents),
        }
    }
}
