use std::mem;

use itertools::multipeek;

use crate::transpiler::code_blocks;
use crate::transpiler::lexer;
use crate::transpiler::lexer::Token;
use crate::transpiler::re;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
            let (_num, (token, string)) = item;
            //println!("{:?}, ({:?}, {:?})", num, token, string);
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
                //}
            } else {
                self.results
                    .push(Contents::new("".to_string(), *token, Chronology::None));
            }
        }
    }

    fn group_to_contents(
        &self,
        string: &str,
        token: Token,
        next: Token,
        previous: &Previous,
    ) -> Option<(Previous, Contents)> {
        let token_discrim = mem::discriminant(&token);
        let next_discrim = mem::discriminant(&next);
        let prev_discrim = mem::discriminant(&previous.kind);

        if let Token::CodeBlock = token {
            if let Some(language) = re::replace_code_block(string) {
                let mut language = language;
                if code_blocks::LISTINGS_LANGUAGES
                    .iter()
                    .filter(|listings_languages| language == **listings_languages)
                    .count()
                    == 0
                {
                    eprintln!(
                        "Language \"{}\" not found. Using default of \"python\".",
                        language
                    );
                    language = "python".to_string();
                };
                let contents = Contents::new(language, token, Chronology::Start);
                return Some((Previous::from(&contents), contents));
            } else {
                let contents = Contents::new(string.to_string(), token, Chronology::End);
                return Some((Previous::from(&contents), contents));
            }
        } else if token_discrim != prev_discrim && token_discrim != next_discrim {
            let contents = Contents::new(string.to_string(), token, Chronology::None);
            return Some((Previous::from(&contents), contents));
        } else if token_discrim != next_discrim {
            let contents = Contents::new(string.to_string(), token, Chronology::End);
            return Some((Previous::from(&contents), contents));
        } else if token_discrim != prev_discrim
            || (token_discrim == prev_discrim && previous.chron == Chronology::End)
        {
            let contents = Contents::new(string.to_string(), token, Chronology::Start);
            return Some((Previous::from(&contents), contents));
        } else if token_discrim == prev_discrim && token_discrim == next_discrim {
            let contents = Contents::new(string.to_string(), token, Chronology::Middle);
            return Some((Previous::from(&contents), contents));
        }
        None
    }
}
