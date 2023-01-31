use itertools::Itertools;

use crate::transpiler::lexer;

#[derive(Debug)]
pub struct Parser {
    pub lexer: lexer::Lexer,
    pub stack: Vec<lexer::Token>,
    pub results: Vec<Contents>,
}

#[derive(Debug)]
pub struct Contents {
    pub contents: String,
    pub kind: lexer::Token,
    pub chron: Chronology,
}

#[derive(Debug)]
pub enum Chronology {
    Start,
    Middle,
    End,
    None,
}

impl From<lexer::Lexer> for Parser {
    fn from(lexer: lexer::Lexer) -> Self {
        let stack: Vec<lexer::Token> = Vec::new();
        let results: Vec<Contents> = Vec::new();
        Parser {
            lexer,
            stack,
            results,
        }
    }
}

impl Parser {
    pub fn run(&mut self) {
        for ((prev_token, prev_string), (curr_token, curr_string), (next_token, next_string)) in self.lexer.tokens.iter().tuple_windows() {
            if lexer::Lexer::is_group(curr_token) && prev_token != curr_token {
                self.stack.push(*curr_token);
                self.results.push(Contents {
                    contents: curr_string.as_ref().unwrap().to_string(), kind: *curr_token, chron: Chronology::Start
                })
            }
            else {
                match curr_token {
                    
                }
            }
            //println!("({:?}, {:?}) | ({:?}, {:?}) | ({:?}, {:?})", prev_token, prev_string, curr_token, curr_string, next_token, next_string);
        }
        println!("{:?}", self.stack);
    }
}
