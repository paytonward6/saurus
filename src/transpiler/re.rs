use regex::Regex;
use regex::Captures;

pub fn heading(line: &str) -> bool {
    let re: Regex = Regex::new(r"^\s*#").unwrap();
    re.is_match(line)
}

pub fn normal(line: &str) -> bool {
    let re: Regex = Regex::new(r"^\s*[^#\-\+`]").unwrap();
    re.is_match(line)
}

pub fn parse_heading(line: &str) -> (usize, String) {
    let re: Regex = Regex::new(r"#\s*").unwrap();
    let line = line.trim();
    let first = line.find(' ');
    (
        line.split_at(first.unwrap()).0.chars().into_iter().count(),
        re.replace_all(line, "").to_string(),
    )
}

pub fn unordered_list(line: &str) -> bool {
    let re: Regex = Regex::new(r"^\s*[\-\+]\s*").unwrap();
    re.is_match(line)
}

pub fn replace_unordered_list(line: &str) -> String {
    let re: Regex = Regex::new(r"^\s*[\-\+]\s*").unwrap();
    re.replace(line, "").to_string()
}

pub fn ordered_list(line: &str) -> bool {
    let re: Regex = Regex::new(r"^\s*\d*\.").unwrap();
    re.is_match(line)
}

pub fn replace_ordered_list(line: &str) -> (usize, String) {
    let re = Regex::new(r"(\d*)\.(\s*.*)").unwrap();
    let cap = re.captures(line).unwrap();

    let number = cap.get(1).unwrap().as_str();
    let contents = cap.get(2).unwrap().as_str();
    (number.trim().parse().unwrap(), contents.to_string())
}

pub fn code_block(line: &str) -> bool {
    let re = Regex::new(r"\s*```").unwrap();
    re.is_match(line)
}

pub fn replace_code_block(line: &str) -> Option<String> {
    let re = Regex::new(r"\s*```(.*)").unwrap();
    let cap = re.captures(line);
    if let Some(cap) = cap {
        let contents = cap.get(1).unwrap().as_str();
        return Some(contents.to_string())
    }
    None
}

pub fn blank(line: &str) -> bool {
    line.is_empty()
}

pub fn bold(line: &mut String) -> String {//Option<String> 
    let re = Regex::new(r"\*\*([^\*]*)\*\*").unwrap();
    re.replace_all(line, |caps: &Captures| {
        format!("\\textbf{{{}}}", &caps[1])
    }).to_string()
}

pub fn italicize(line: &mut String) -> String {//Option<String> 
    let re = Regex::new(r"\*([^\*]*)\*").unwrap();
    re.replace_all(line, |caps: &Captures| {
        format!("\\textit{{{}}}", &caps[1])
    }).to_string()
}

// NOT TO BE USED YET
pub fn inline_code(line: &mut String) -> String {//Option<String> 
    let re = Regex::new(r"`([^`])`").unwrap();
    re.replace_all(line, |caps: &Captures| {
        format!("\\verb|{}|", &caps[1])
    }).to_string()
}


/// uses the "ulem" package
pub fn strike_out(line: &mut String) -> String {//Option<String> 
    let re = Regex::new(r"\~\~([^\~]*)\~\~").unwrap();
    re.replace_all(line, |caps: &Captures| {
        format!("\\sout{{{}}}", &caps[1])
    }).to_string()
}
