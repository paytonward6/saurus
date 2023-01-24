use regex::Regex;
use regex::Captures;

use crate::transpiler::code_blocks::{Languages};

/// ```
/// use saurus::transpiler::re;
///
/// assert!(re::heading("# Heading 1"));
/// assert!(re::heading("## Heading 2"));
/// ```
pub fn heading(line: &str) -> bool {
    let re: Regex = Regex::new(r"^\s*#+").unwrap();
    re.is_match(line)
}

/// ```
/// use saurus::transpiler::re;
/// // NOT NORMAL
/// assert!(!re::normal("- Unordered List"));
/// assert!(!re::normal("+ Unordered List"));
/// assert!(!re::normal("# Heading"));
/// assert!(!re::normal("`inline code`"));
///
/// // NORMAL
/// assert!(re::normal("Regular text"));
/// assert!(re::normal("!Regular with exclamation"));
/// ```
pub fn normal(line: &str) -> bool {
    let re: Regex = Regex::new(r"^\s*[^#\-\+`]").unwrap();
    re.is_match(line)
}

///```
/// use saurus::transpiler::re;
/// let (number1, contents1) = re::parse_heading(r"# Heading 1");
/// assert_eq!(number1, 1);
/// assert_eq!(contents1, "Heading 1");
///
/// let (number4, contents4) = re::parse_heading(r"#### Heading 4");
/// assert_eq!(number4, 4);
/// assert_eq!(contents4, "Heading 4");
///```
pub fn parse_heading(line: &str) -> (usize, String) {
    let re: Regex = Regex::new(r"^#+\s*").unwrap();
    let line = line.trim();
    let first = line.find(' ');
    (
        line.split_at(first.unwrap()).0.chars().into_iter().count(),
        re.replace_all(line, "").to_string(),
    )
}

///```
/// use saurus::transpiler::re;
/// assert!(re::unordered_list(r"- Contents here!"));
/// assert!(re::unordered_list(r"+ Contents here!"));
///```
pub fn unordered_list(line: &str) -> bool {
    let re: Regex = Regex::new(r"^\s*[\-\+]\s*").unwrap();
    re.is_match(line)
}

///```
/// use saurus::transpiler::re;
/// let contents = String::from("Contents here!");
/// assert_eq!(re::replace_unordered_list(&mut "- Contents here!"), contents);
/// assert_eq!(re::replace_unordered_list(&mut "+ Contents here!"), contents);
///```
pub fn replace_unordered_list(line: &str) -> String {
    let re: Regex = Regex::new(r"^\s*[\-\+]\s*").unwrap();
    re.replace(line, "").to_string()
}

///```
/// use saurus::transpiler::re;
/// assert!(re::ordered_list("7. Contents here!"));
///```
pub fn ordered_list(line: &str) -> bool {
    let re: Regex = Regex::new(r"^\s*\d*\.").unwrap();
    re.is_match(line)
}

///```
/// use saurus::transpiler::re;
/// let (number, contents) = re::replace_ordered_list("7. Contents here!");
/// assert_eq!(number, 7);
/// assert_eq!(contents, "Contents here!");
///```
pub fn replace_ordered_list(line: &str) -> (usize, String) {
    let re = Regex::new(r"(\d*)\.\s*(.*)").unwrap();
    let cap = re.captures(line).unwrap();

    let number = cap.get(1).unwrap().as_str();
    let contents = cap.get(2).unwrap().as_str();
    (number.trim().parse().unwrap(), contents.to_string())
}

///```
/// use saurus::transpiler::re;
/// assert!(re::code_block(&"```python".to_string()));
///```
pub fn code_block(line: &str) -> bool {
    let re = Regex::new(r"^\s*```").unwrap();
    re.is_match(line)
}

///```
/// use saurus::transpiler::re;
/// assert_eq!(re::replace_code_block(&"```python".to_string()).unwrap(), r"python".to_string());
///```
pub fn replace_code_block(line: &str) -> Option<Languages> {
    let re = Regex::new(r"\s*```(.+)").unwrap();
    let cap = re.captures(line);
    if let Some(cap) = cap {
        let contents: &str = cap.get(1).unwrap().as_str();
        let result: Languages = contents.parse().unwrap_or_else(|_| {
            println!("Code block: Language \"{}\" not found. Using default of \"Language::C\"", contents);
            Languages::C
        });
        return Some(result)
    }
    None
}

///```
/// use saurus::transpiler::re;
/// assert!(re::blank(&mut "".to_string()));
///```
pub fn blank(line: &str) -> bool {
    line.is_empty()
}
///```
/// use saurus::transpiler::re;
/// assert_eq!(re::bold(&mut "**bold me**".to_string()), r"\textbf{bold me}".to_string());
///```
pub fn bold(line: &mut String) -> String {//Option<String> 
    let re = Regex::new(r"\*\*([^\*]*)\*\*").unwrap();
    re.replace_all(line, |caps: &Captures| {
        format!("\\textbf{{{}}}", &caps[1])
    }).to_string()
}
///```
/// use saurus::transpiler::re;
/// assert_eq!(re::italicize(&mut "*italicize me*".to_string()), r"\textit{italicize me}".to_string());
///```
pub fn italicize(line: &mut String) -> String {//Option<String> 
    let re = Regex::new(r"\*([^\*]*)\*").unwrap();
    re.replace_all(line, |caps: &Captures| {
        format!("\\textit{{{}}}", &caps[1])
    }).to_string()
}

///```
/// use saurus::transpiler::re;
/// assert_eq!(re::bold_italicize(&mut "***italicize me***".to_string()), r"\textbf{\textit{italicize me}}".to_string());
///```
pub fn bold_italicize(line: &mut String) -> String {//Option<String> 
    let re = Regex::new(r"\*\*\*([^\*]*)\*\*\*").unwrap();
    re.replace_all(line, |caps: &Captures| {
        format!("\\textbf{{\\textit{{{}}}}}", &caps[1])
    }).to_string()
}

///```
/// use saurus::transpiler::re;
/// assert_eq!(re::links(&mut "[saurus](https://github.com/paytonward6/saurus)".to_string()), r"\href{https://github.com/paytonward6/saurus}{saurus}".to_string());
///```
pub fn links(line: &mut String) -> String {
    let re = Regex::new(r"\[([a-zA-Z:]*)\]\((https://.*)\)").unwrap();
    re.replace_all(line, |caps: &Captures| {
        format!("\\href{{{}}}{{{}}}", &caps[2], &caps[1])
    }).to_string()
}

///```
/// use saurus::transpiler::re;
/// assert_eq!(re::inline_code(&mut "`let x = 2;`".to_string()), r"\verb|let x = 2;|".to_string());
///```
pub fn inline_code(line: &mut String) -> String {//Option<String> 
    //let re = Regex::new(r"`([^`]*)`").unwrap();
    let re = Regex::new(r"`([^`]+)`").unwrap();
    re.replace_all(line, |caps: &Captures| {
        println!("{}", &caps[1]);
        format!("\\verb|{}|", &caps[1])
    }).to_string()
}

///```
/// use saurus::transpiler::re;
/// assert_eq!(re::symbols(&mut "=>".to_string()), r"$\rightarrow$".to_string());
///```
pub fn symbols(line: &mut String) -> String {//Option<String> 
    let re = Regex::new(r"=>").unwrap();
    re.replace_all(line, "$\\rightarrow$").to_string()
}

/// uses the "ulem" package
/// ```
/// use saurus::transpiler::re;
/// assert_eq!(re::strike_out(&mut "~~strike this out~~".to_string()), r"\sout{strike this out}".to_string());
/// ```
pub fn strike_out(line: &mut String) -> String {//Option<String> 
    let re = Regex::new(r"\~\~([^\~]*)\~\~").unwrap();
    re.replace_all(line, |caps: &Captures| {
        format!("\\sout{{{}}}", &caps[1])
    }).to_string()
}

