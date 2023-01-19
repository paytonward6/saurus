use std::collections::VecDeque;
use std::fmt;
use std::path::PathBuf;

use crate::tokenizer::latexer;

pub fn run(file_str: &str, path: &PathBuf) {
    let tokens = tokenize(file_str);
    latexer::write(&tokens, path).unwrap_or_else(|error| {
        println!("{}", error);
    });
}

#[derive(Debug, PartialEq, Eq)]
pub enum TokenKind {
    FileStart,
    FileEnd,
    Heading(usize),

    BeginUnorderedList,
    UnorderedListItem(char),
    EndUnorderedList,

    BeginOrderedList,
    OrderedListItem(usize),
    EndOrderedList,

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
            contents,
            kind,
            line_num,
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
            match stack.back() {
                Some(Kind::BeginOrderedList) => {
                    tokens.push(Token::new(None, Kind::EndOrderedList, i));
                    stack.push_back(Kind::EndOrderedList);
                }
                Some(Kind::BeginUnorderedList) => {
                    tokens.push(Token::new(None, Kind::EndUnorderedList, i));
                    stack.push_back(Kind::EndUnorderedList);
                }
                _ => (),
            }

            let (num_hash, line) = re::parse_heading(&line);
            tokens.push(Token::new(Some(line), Kind::Heading(num_hash), i));
            stack.push_back(Kind::Heading(num_hash));
        } else if re::unordered_list(&line) {
            let line = re::replace_unordered_list(&line);

            #[deny(clippy::single_match)]
            if let Some(last) = stack.back() {
                if last != &Kind::BeginUnorderedList {
                    tokens.push(Token::new(None, Kind::BeginUnorderedList, i));
                    stack.push_back(Kind::BeginUnorderedList);
                }
            }

            // TODO: ONLY PUTTING '-' FOR NOW
            tokens.push(Token::new(Some(line), Kind::UnorderedListItem('-'), i));
        } else if re::ordered_list(&line) {
            let (number, line) = re::replace_ordered_list(&line);

            #[deny(clippy::single_match)]
            if let Some(last) = stack.back() {
                if last != &Kind::BeginOrderedList {
                    tokens.push(Token::new(None, Kind::BeginOrderedList, i));
                    stack.push_back(Kind::BeginOrderedList);
                }
            }

            // TODO: ONLY PUTTING '-' FOR NOW
            tokens.push(Token::new(Some(line), Kind::OrderedListItem(number), i));
        } else if re::blank(&line) {
            match stack.back() {
                Some(Kind::BeginUnorderedList) => {
                    tokens.push(Token::new(None, Kind::EndUnorderedList, i));
                    stack.push_back(Kind::EndUnorderedList)
                }
                _ => (),
            }
        }
    }

    match stack.back() {
        Some(Kind::FileStart) => tokens.push(Token::new(None, Kind::FileEnd, usize::MIN)),
        Some(Kind::BeginUnorderedList) => {
            tokens.push(Token::new(None, Kind::EndUnorderedList, usize::MAX - 1))
        }
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

    pub fn parse_heading(line: &str) -> (usize, String) {
        let re: Regex = Regex::new(r"#\s*").unwrap();
        let line = line.trim();
        let first = line.find(' ');
        (
            line.split_at(first.unwrap()).0.chars().into_iter().count(),
            re.replace_all(line, "").to_string(),
        )
    }

    pub fn unordered_list(line: &str) -> bool {
        let re: Regex = Regex::new(r"^\s*[\-\+]\s*").unwrap();
        re.is_match(line)
    }

    pub fn replace_unordered_list(line: &str) -> String {
        let re: Regex = Regex::new(r"^\s*[\-\+]\s*").unwrap();
        re.replace(line, "").to_string()
    }

    pub fn ordered_list(line: &str) -> bool {
        let re: Regex = Regex::new(r"^\s*\d\.").unwrap();
        re.is_match(line)
    }

    pub fn replace_ordered_list(line: &str) -> (usize, String) {
        let re = Regex::new(r"(\d)\.(\s*.*)").unwrap();
        let cap = re.captures(line).unwrap();

        let number = cap.get(1).unwrap().as_str();
        let contents = cap.get(2).unwrap().as_str();
        (number.trim().parse().unwrap(), contents.to_string())
    }

    // TODO: Capture for ordered list \(\d.\)\(\s*.*\)

    pub fn blank(line: &str) -> bool {
        line.is_empty()
    }
}
