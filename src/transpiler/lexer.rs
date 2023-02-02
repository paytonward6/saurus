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
    pub tokens: Vec<(Token, Option<String>)>,
    pub number_of_lines: usize,
    pub contains_code_block: bool,
}

use crate::transpiler::re;
impl Lexer {
    pub fn new() -> Self {
        let tokens: Vec<(Token, Option<String>)> = Vec::new();
        let number_of_lines = 0;
        let contains_code_block = false;
        Self {
            tokens,
            number_of_lines,
            contains_code_block,
        }
    }
    pub fn tokenize(&mut self, file_str: &str) {
        self.tokens.push((Token::FileStart, None));
        for (_line_number, line) in file_str.lines().enumerate() {
            let line = line.to_string();
            //let line = Transpiler::transpile_line(&mut line);
            if re::is_heading(&line) {
                let (level, line) = re::parse_heading(&line);
                self.tokens.push((Token::Heading(level), Some(line)));
            } else if re::is_unordered_list(&line) {
                self.tokens.push((Token::UnorderedList, Some(line)));
            } else if line.is_empty() {
                self.tokens.push((Token::Blank, None));
            } else if re::is_ordered_list(&line) {
                let (number, line) = re::replace_ordered_list(&line);
                self.tokens.push((Token::OrderedList(number), Some(line)));
            } else if re::is_code_block(&line) {
                self.contains_code_block = true;
                self.tokens.push((Token::CodeBlock, Some(line)));
            } else if re::is_block_quote(&line) {
                self.tokens.push((Token::BlockQuote, Some(line)));
            } else if re::is_normal(&line) {
                self.tokens.push((Token::Text, Some(line)));
            }
            self.number_of_lines += 1;
        }
        self.tokens.push((Token::FileEnd, None));
    }

    pub fn is_group(kind: &Token) -> bool {
        match kind {
            Token::UnorderedList | Token::OrderedList(_) | Token::BlockQuote | Token::CodeBlock => {
                true
            }
            _ => false,
        }
    }
}
