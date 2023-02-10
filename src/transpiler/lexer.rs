#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Token {
    FileStart,
    FileEnd,

    Heading(usize),

    UnorderedList,

    OrderedList(usize),

    CodeBlock,

    BlockQuote,

    Text,

    Blank,
}

#[derive(Debug)]
pub struct Lexer {
    pub results: Vec<Info>,
    pub number_of_lines: usize,
    pub contains_code_block: bool,
}

#[derive(Debug, Clone)]
pub struct Info {
    pub token: Token,
    pub line: Option<String>,
    pub indent_level: usize,
}

impl Info {
    fn new(token: Token, line: Option<String>, indent_level: usize) -> Self {
        Self {
            token,
            line,
            indent_level,
        }
    }
}

use crate::transpiler::re;
impl Lexer {
    pub fn new() -> Self {
        let tokens: Vec<Info> = Vec::new();
        let number_of_lines = 0;
        let contains_code_block = false;
        Self {
            results: tokens,
            number_of_lines,
            contains_code_block,
        }
    }
    pub fn tokenize(&mut self, file_str: &str) {
        self.results.push(Info::new(Token::FileStart, None, 0));
        for (_line_number, line) in file_str.lines().enumerate() {
            let indent_level = re::indent_level(line);
            let line = line.to_string();

            if re::is_heading(&line) {
                let (level, line) = re::parse_heading(&line);
                self.results
                    .push(Info::new(Token::Heading(level), Some(line), indent_level));
            } else if re::is_unordered_list(&line) {
                let line = re::replace_unordered_list(&line);
                self.results
                    .push(Info::new(Token::UnorderedList, Some(line), indent_level));
            } else if line.is_empty() {
                continue;
                //self.results
                //    .push(Info::new(Token::Blank, None, indent_level));
            } else if re::is_ordered_list(&line) {
                let (number, line) = re::replace_ordered_list(&line);
                self.results.push(Info::new(
                    Token::OrderedList(number),
                    Some(line),
                    indent_level,
                ));
            } else if re::is_code_block(&line) {
                self.contains_code_block = true;
                self.results
                    .push(Info::new(Token::CodeBlock, Some(line), indent_level));
            } else if re::is_block_quote(&line) {
                self.results
                    .push(Info::new(Token::BlockQuote, Some(line), indent_level));
            } else if re::is_normal(&line) {
                self.results
                    .push(Info::new(Token::Text, Some(line), indent_level));
            }
            self.number_of_lines += 1;
        }
        self.results.push(Info::new(Token::FileEnd, None, 0));
    }

    pub fn is_group(kind: &Token) -> bool {
        matches!(
            kind,
            Token::UnorderedList | Token::OrderedList(_) | Token::BlockQuote | Token::CodeBlock
        )
    }
}
