use itertools::multipeek;
use itertools::peek_nth;
use itertools::structs::PeekNth;

use crate::transpiler::lexer;
use crate::transpiler::lexer::Token;

#[derive(Debug)]
pub struct Parser {
    pub lexer: lexer::Lexer,
    pub stack: Vec<lexer::Token>,
    pub results: Vec<Contents>,
    pub previous: Previous,
}

#[derive(Debug)]
pub struct Contents {
    pub line: String,
    pub kind: lexer::Token,
    pub chron: Chronology,
}

#[derive(Debug)]
pub struct Previous {
    pub kind: lexer::Token,
    pub chron: Chronology,
}

#[derive(Debug, Clone, Copy)]
pub enum Chronology {
    Start,
    Middle,
    End,
    None,
}

impl From<lexer::Lexer> for Parser {
    fn from(lexer: lexer::Lexer) -> Self {
        let stack: Vec<lexer::Token> = Vec::new();
        let previous = Previous {
            kind: Token::FileStart,
            chron: Chronology::None,
        };
        let results: Vec<Contents> = Vec::new();
        Parser {
            lexer,
            stack,
            results,
            previous,
        }
    }
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
    fn new(line: String, kind: lexer::Token, chron: Chronology) -> Self {
        Contents { line, kind, chron }
    }
}

impl Parser {
    pub fn run(&mut self) {
        let mut iter = multipeek(self.lexer.tokens.iter().enumerate());
        while let Some(item) = iter.next() {
            let (num, (token, string)) = item;
            println!("{:?}, ({:?}, {:?})", num, token, string);
            if let Some(next) = iter.peek() {
                if lexer::Lexer::is_group(&token) {
                    let next_token = next.1 .0;
                    if let Some((previous, contents)) = self.group_to_contents(
                        &string.as_ref().unwrap_or(&"".to_owned()),
                        *token,
                        next_token,
                        &self.previous,
                    ) {
                        self.previous = previous;
                        self.results.push(contents);
                    }
                } else {
                    if let Token::Blank = token {
                        continue
                    }
                    else {
                        self.results.push(Contents::new(
                            string.as_ref().unwrap_or(&"".to_owned()).to_string(),
                            *token,
                            Chronology::None,
                        ));
                        self.previous = Previous {
                            kind: *token,
                            chron: Chronology::None,
                        };
                    }
                }
            }
        }
        println!("{:?}", self.previous);
    }

    fn group_to_contents(
        &self,
        string: &str,
        token: Token,
        next: Token,
        previous: &Previous,
    ) -> Option<(Previous, Contents)> {
        if token != previous.kind && token != next {
            let contents = Contents::new(string.to_string(), token, Chronology::None);
            return Some((Previous::from(&contents), contents));
        } else if token != next {
            // if token == next token
            let contents = Contents::new(string.to_string(), token, Chronology::End);
            return Some((Previous::from(&contents), contents));
        } else if token != previous.kind {
            // if token == next token
            let contents = Contents::new(string.to_string(), token, Chronology::Start);
            return Some((Previous::from(&contents), contents));
        } else if token == previous.kind && token == next {
            // if token == next token
            let contents = Contents::new(string.to_string(), token, Chronology::Middle);
            return Some((Previous::from(&contents), contents));
        }
        None
    }
}
