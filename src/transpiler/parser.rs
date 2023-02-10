use std::mem;

use crate::transpiler::code_blocks;
use crate::transpiler::lexer;
use crate::transpiler::lexer::Token;
use crate::transpiler::re;

#[derive(Debug)]
pub struct Parser {
    pub records: Vec<usize>,
    pub results: Vec<Contents>,
    pub previous: Record,
    pub contains_code_block: bool,
}

#[derive(Debug, Clone)]
pub struct Contents {
    pub line: Option<String>,
    pub kind: lexer::Token,
    pub indent_level: usize,
    pub chron: Chronology,
}

#[derive(Debug)]
pub struct Record {
    pub kind: lexer::Token,
    pub chron: Chronology,
    pub indent_level: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Chronology {
    Start,
    Middle,
    End,
    None,
}

impl From<&Contents> for Record {
    fn from(contents: &Contents) -> Self {
        Record {
            kind: contents.kind,
            chron: contents.chron,
            indent_level: contents.indent_level,
        }
    }
}

impl Contents {
    fn new(info: lexer::Info, chron: Chronology) -> Self {
        Contents { line: info.line, kind: info.token, indent_level: info.indent_level, chron }
    }

    fn new_with_line(line: Option<String>, info: lexer::Info, chron: Chronology) -> Self {
        Contents { line, kind: info.token, indent_level: info.indent_level, chron }
    }
}

impl Parser {
    pub fn new() -> Parser {
        let records: Vec<usize> = Vec::new();
        let previous = Record {
            kind: Token::FileStart,
            chron: Chronology::None,
            indent_level: 0,
        };
        let results: Vec<Contents> = Vec::new();
        let contains_code_block = false;
        Parser {
            records,
            results,
            previous,
            contains_code_block,
        }
    }

    pub fn run(&mut self, lexer: lexer::Lexer) {
        if lexer.results.iter().any(|item| item.token == Token::CodeBlock) {
            self.contains_code_block = true;
        }

        let mut iter = lexer.results.into_iter().enumerate().peekable();
        while let Some(item) = iter.next() {
            let (number, current) = item;
            if let Some((_, next)) = iter.peek() {
                if lexer::Lexer::is_group(&current.token) {
                    if let Some(contents) = self.group_to_contents(current, next) {
                        if contents.chron == Chronology::Start {
                            self.records.push(number);
                        }
                        else if contents.chron == Chronology::End {
                            self.records.pop();
                        }
                        self.previous = Record::from(&contents);
                        self.results.push(contents);
                    }
                } else {
                    // Close open lists up to that point
                    for record in self.records.iter() {
                        let contents = self.results.get(*record).unwrap();
                        self.results.push(Contents {
                            line: None,
                            kind: contents.kind,
                            chron: Chronology::End,
                            indent_level: contents.indent_level,
                        });
                    }
                    self.records.clear();
                    let contents = Contents::new(
                        current,
                        Chronology::None,
                    );
                    self.previous = Record::from(&contents);
                    self.results.push(contents);
                }
            }
        }
        self.results.push(Contents {line: None, kind: Token::FileEnd, indent_level: 0, chron: Chronology::None});
        println!("{:#?}", self.results);
    }

    fn group_to_contents(&self, current: lexer::Info, next: &lexer::Info) -> Option<Contents> {
        let token_discrim = mem::discriminant(&current.token);
        let next_discrim = mem::discriminant(&next.token);
        let prev_discrim = mem::discriminant(&self.previous.kind);

        if let Token::CodeBlock = current.token {
            if let Some(language) = re::replace_code_block(current.line.as_ref().map(|x| &**x)) {
                let mut language = language;
                if code_blocks::is_invalid_language(&language)
                {
                    eprintln!(
                        "Language \"{}\" not found. Using default of \"python\".",
                        language
                    );
                    language = "python".to_string();
                };
                return Some(Contents::new_with_line(Some(language), current, Chronology::Start))
            } else {
                return Some(Contents::new(current, Chronology::End))
            }
        } else if current.indent_level > self.previous.indent_level && current.indent_level > next.indent_level {
            return Some(Contents::new(current, Chronology::None));
        } else if current.indent_level > self.previous.indent_level {
            return Some(Contents::new(current, Chronology::Start))
        } else if current.indent_level > next.indent_level {
            return Some(Contents::new(current, Chronology::End))
        }  else if token_discrim != prev_discrim && token_discrim != next_discrim {
            return Some(Contents::new(current, Chronology::None));
        } else if token_discrim != next_discrim {
            return Some(Contents::new(current, Chronology::End));
        } else if token_discrim != prev_discrim
            //|| (token_discrim == prev_discrim && self.previous.chron == Chronology::End)
        {
            return Some(Contents::new(current, Chronology::Start));
        } else if token_discrim == prev_discrim && token_discrim == next_discrim {
            return Some(Contents::new(current, Chronology::Middle));
        }
        None
    }
}
