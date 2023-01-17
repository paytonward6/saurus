pub fn run(file_str: &str) {
    println!("{:?}", tokenize(file_str));
}

#[derive(Debug)]
enum TokenKind {
    HEADING(usize),
    LIST(char),
}

#[derive(Debug)]
struct Token {
    kind: TokenKind,
    line_num: usize,
}

fn tokenize(file_str: &str) -> Vec<Token> {
    type Kind = TokenKind;
    let mut tokens: Vec<Token> = Vec::new(); 
    for (i, line) in file_str.lines().enumerate() {
        if re::heading(line) {
            // TODO: ONLY PUTTING ONE FOR LEVEL 1
            tokens.push(Token {kind: Kind::HEADING(1), line_num: i});
        }
        else if re::list(line) {
            // TODO: ONLY PUTTING '-' FOR NOW
            tokens.push(Token {kind: Kind::LIST('-'), line_num: i});
        }
    }
    tokens
}

mod re {
    use regex::Regex;
    pub fn heading(line: &str) -> bool {
        let re: Regex = Regex::new(r"^\s*#").unwrap();
        re.is_match(line)
    }

    pub fn list(line: &str) -> bool {
        let re: Regex = Regex::new(r"^\s*[\-\+]").unwrap();
        re.is_match(line)
    }
}


mod utils {
    pub fn line_count(file_str: &str) -> usize {
        file_str.lines().into_iter().count()
    }
}
