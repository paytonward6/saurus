use std::mem;

use crate::transpiler::code_blocks;
use crate::transpiler::lexer;
use crate::transpiler::lexer::Token;
use crate::transpiler::re;

#[derive(Debug)]
pub struct Parser {
    pub stack: Vec<lexer::Token>,
    pub results: Vec<Contents>,
    pub previous: Previous,
    pub contains_code_block: bool,
}

#[derive(Debug)]
pub struct Contents {
    pub line: Option<String>,
    pub kind: lexer::Token,
    pub chron: Chronology,
}

#[derive(Debug)]
pub struct Previous {
    pub kind: lexer::Token,
    pub chron: Chronology,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Chronology {
    Start,
    Middle,
    End,
    None,
}

impl From<&Contents> for Previous {
    fn from(contents: &Contents) -> Self {
        Previous {
            kind: contents.kind,
            chron: contents.chron,
        }
    }
}

impl Contents {
    fn new(line: Option<String>, kind: lexer::Token, chron: Chronology) -> Self {
        Contents { line, kind, chron }
    }
}

impl Parser {
    pub fn new() -> Parser {
        let stack: Vec<lexer::Token> = Vec::new();
        let previous = Previous {
            kind: Token::FileStart,
            chron: Chronology::None,
        };
        let results: Vec<Contents> = Vec::new();
        let contains_code_block = false;
        Parser {
            stack,
            results,
            previous,
            contains_code_block,
        }
    }

    pub fn run(&mut self, lexer: lexer::Lexer) {
        if lexer.results.iter().any(|item| item.token == Token::CodeBlock) {
            self.contains_code_block = true;
        }

        let mut iter = lexer.results.into_iter().peekable();
        while let Some(item) = iter.next() {
            let current = item;
            if current.token == Token::Blank {
                continue
            }
            else if let Some(next) = iter.peek() {
                if lexer::Lexer::is_group(&current.token) {
                    if let Some(contents) = self.group_to_contents(current, next) {
                        self.previous = Previous::from(&contents);
                        self.results.push(contents);
                    }
                } else {
                    let contents = Contents::new(
                        current.line,
                        current.token,
                        Chronology::None,
                    );
                    self.previous = Previous::from(&contents);
                    self.results.push(contents);
                }
            } else {
                self.results.push(Contents::new(
                    None,
                    current.token,
                    Chronology::None,
                ));
            }
            println!("{:?}", self.previous);
        }
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
                return Some(Contents::new(Some(language), current.token, Chronology::Start))
            } else {
                return Some(Contents::new(current.line, current.token, Chronology::End))
            }
        } else if token_discrim != prev_discrim && token_discrim != next_discrim {
            return Some(Contents::new(current.line, current.token, Chronology::None));
        } else if token_discrim != next_discrim {
            return Some(Contents::new(current.line, current.token, Chronology::End));
        } else if token_discrim != prev_discrim
            || (token_discrim == prev_discrim && self.previous.chron == Chronology::End)
        {
            return Some(Contents::new(current.line, current.token, Chronology::Start));
        } else if token_discrim == prev_discrim && token_discrim == next_discrim {
            return Some(Contents::new(current.line, current.token, Chronology::Middle));
        }
        None
    }
}
