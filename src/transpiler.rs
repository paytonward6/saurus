use std::collections::VecDeque;
use std::fmt;
use std::fs;
use std::io::{Error, Write};
use std::path::PathBuf;

pub mod latexer;
pub mod re;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TokenKind {
    FileStart,
    FileEnd,
    Heading(usize),

    BeginUnorderedList,
    UnorderedListItem(char),
    EndUnorderedList,

    BeginOrderedList(usize),
    OrderedListItem(usize),
    EndOrderedList,

    BeginCodeBlock(String),
    BodyCodeBlock,
    EndCodeBlock,

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

#[derive(Debug)]
pub struct Transpiler {
    pub tokens: Vec<Token>,
    pub stack: VecDeque<TokenKind>,
}

impl Transpiler {
    pub fn new() -> Self {
        let tokens: Vec<Token> = Vec::new();
        let stack: VecDeque<TokenKind> = VecDeque::new();
        Transpiler { tokens, stack }
    }

    fn add_structure(&mut self, contents: Option<String>, kind: TokenKind, line_num: usize) {
        self.tokens
            .push(Token::new(contents, kind.clone(), line_num));
        self.stack.push_back(kind);
    }

    pub fn run(mut self, file_str: &str, path: &PathBuf) {
        self.tokenize(file_str);
        self.write(path).unwrap_or_else(|error| {
            println!("{}", error);
        });
    }

    fn close_list(&mut self, line_num: usize) {
        match self.stack.back() {
            Some(TokenKind::BeginOrderedList(_)) => {
                self.add_structure(None, TokenKind::EndOrderedList, line_num);
            }
            Some(TokenKind::BeginUnorderedList) => {
                self.add_structure(None, TokenKind::EndUnorderedList, line_num);
            }
            _ => (),
        }
    }

    fn tokenize(&mut self, file_str: &str) {
        type Kind = TokenKind;

        self.add_structure(None, Kind::FileStart, usize::MIN);

        for (line_number, line) in file_str.lines().enumerate() {
            let mut line = line.to_string();
            let line = Transpiler::transpile_line(&mut line);
            if re::heading(&line) {
                self.close_list(line_number);

                let (num_hash, line) = re::parse_heading(&line);
                self.add_structure(Some(line), Kind::Heading(num_hash), line_number);
            } else if re::unordered_list(&line) {
                self.add_unordered_list(line, line_number);
            } else if re::blank(&line) {
                self.close_list(line_number);
            } else if re::ordered_list(&line) {
                self.add_ordered_list(line, line_number);
            } else if re::code_block(&line) {
                self.add_code_block(line, line_number);
            } else if re::normal(&line) {
                self.normal(line, line_number);
            }

        }

        match self.stack.back() {
            Some(Kind::FileStart) => self
                .tokens
                .push(Token::new(None, Kind::FileEnd, usize::MIN)),
            Some(Kind::BeginUnorderedList) => {
                self.tokens
                    .push(Token::new(None, Kind::EndUnorderedList, usize::MAX - 1))
            }
            _ => (),
        }

        self.add_structure(None, Kind::FileEnd, usize::MAX);
    }

    pub fn add_unordered_list(&mut self, line: String, line_number: usize) {
        let line = re::replace_unordered_list(&line);

        #[deny(clippy::single_match)]
        if let Some(last) = self.stack.back() {
            match last {
                TokenKind::BeginUnorderedList => {
                    // TODO: ONLY PUTTING '-' FOR NOW
                    self.tokens.push(Token::new(
                        Some(line),
                        TokenKind::UnorderedListItem('-'),
                        line_number,
                    ));
                }
                _ => {
                    self.add_structure(None, TokenKind::BeginUnorderedList, line_number);
                    self.tokens.push(Token::new(
                        Some(line),
                        TokenKind::UnorderedListItem('-'),
                        line_number,
                    ));
                }
            }
        }
    }

    fn add_ordered_list(&mut self, line: String, line_number: usize) {
        let (item_number, line) = re::replace_ordered_list(&line);

        #[deny(clippy::single_match)]
        if let Some(last) = self.stack.back() {
            match last {
                TokenKind::BeginOrderedList(_) => {
                    self.tokens.push(Token::new(
                        Some(line),
                        TokenKind::OrderedListItem(item_number),
                        line_number,
                    ));
                }
                _ => {
                    // Begin list
                    self.add_structure(None, TokenKind::BeginOrderedList(item_number), line_number);
                    self.tokens.push(Token::new(
                        Some(line),
                        TokenKind::OrderedListItem(item_number),
                        line_number,
                    ));
                }
            }
        }
    }

    fn add_code_block(&mut self, line: String, line_number: usize) {
        let language = re::replace_code_block(&line);
        if let Some(last) = self.stack.back() {
            match last {
                TokenKind::BodyCodeBlock | TokenKind::BeginCodeBlock(_) => {
                    self.add_structure(None, TokenKind::EndCodeBlock, line_number);
                }
                _ => {
                    self.add_structure(None, TokenKind::BeginCodeBlock(language.unwrap()), line_number);
                }
            }
        }
    }

    fn normal(&mut self, line: String, line_number: usize) {
        if let Some(last) = self.stack.back() {
            match last {
                TokenKind::BeginCodeBlock(_) | TokenKind::BodyCodeBlock => {
                    self.add_structure(Some(line), TokenKind::BodyCodeBlock, line_number);
                }
                _ => ()
            }
        }
    }

    pub fn write(&self, path: &PathBuf) -> Result<(), Error> {
        let mut file = fs::File::create(path)?;
        write!(file, "{}\n", latexer::documentclass())?;
        write!(file, "{}\n", latexer::packages())?;
        for line in self.tokens.iter() {
            let line = latexer::body(line);
            if let Some(line) = line {
                write!(file, "{}\n", line)?;
            }
        }
        Ok(())
    }

    pub fn transpile_line(line: &mut String) -> String {
        *line = re::bold(line);
        *line = re::italicize(line);
        *line = re::inline_code(line);
        *line = re::strike_out(line);
        line.to_string()
    }
}

//mod Lines {
//    fn parse(&mut sel)
//}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.contents.as_ref().unwrap())
    }
}
