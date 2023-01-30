#[derive(Debug)]
pub enum Token {
    FileStart,
    FileEnd,
    Heading,

    BeginUnorderedList,
    UnorderedListItem,
    EndUnorderedList,

    BeginOrderedList,
    OrderedListItem,
    EndOrderedList,

    BeginCodeBlock,
    BodyCodeBlock,
    EndCodeBlock,

    BeginBlockQuote,
    BodyBlockQuote,
    EndBlockQuote,

    Text,

    Blank,
}

#[derive(Debug)]
pub struct Lexer {
    pub contents: Vec<(Token, String)>,
    pub number_of_lines: usize,
}

use crate::transpiler::re;
impl Lexer {
    pub fn tokenize(&mut self, file_str: &str) {
        for (_line_number, line) in file_str.lines().enumerate() {
            let line = line.to_string();
            //let line = Transpiler::transpile_line(&mut line);
            if re::is_heading(&line) {
                self.contents.push((Token::Heading, line));
            } else if re::is_unordered_list(&line) {
                self.contents.push((Token::UnorderedListItem, line));
            } else if line.is_empty() {
                self.contents.push((Token::Blank, line));
            } else if re::is_ordered_list(&line) {
                self.contents.push((Token::OrderedListItem, line));
            } else if re::is_code_block(&line) {
                self.contents.push((Token::BodyCodeBlock, line));
            } else if re::is_block_quote(&line) {
                self.contents.push((Token::BodyBlockQuote, line));
            } else if re::is_normal(&line) {
                self.contents.push((Token::Text, line));
            }
        }
    }
}
