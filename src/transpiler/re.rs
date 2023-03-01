use regex::Captures;
use regex::Regex;

/// ```
/// use saurus::transpiler::re;
///
/// assert!(re::is_heading("# Heading 1"));
/// assert!(re::is_heading("## Heading 2"));
/// ```
pub fn is_heading(line: &str) -> bool {
    let re: Regex = Regex::new(r"^\s*#+").unwrap();
    re.is_match(line)
}

/// ```
/// use saurus::transpiler::re;
/// // NOT NORMAL
/// assert!(!re::is_normal("- Unordered List"));
/// assert!(!re::is_normal("+ Unordered List"));
/// assert!(!re::is_normal("# Heading"));
/// assert!(!re::is_normal("`inline code`"));
///
/// // NORMAL
/// assert!(re::is_normal("Regular text"));
/// assert!(re::is_normal("!Regular with exclamation"));
/// ```
pub fn is_normal(line: &str) -> bool {
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
/// assert!(re::is_unordered_list(r"- Contents here!"));
/// assert!(re::is_unordered_list(r"+ Contents here!"));
/// assert!(!re::is_unordered_list(r"**Contents here!**"));
///```
pub fn is_unordered_list(line: &str) -> bool {
    let re: Regex = Regex::new(r"^\s*[\-\+\*]{1}\s+").unwrap();
    re.is_match(line)
}

///```
/// use saurus::transpiler::re;
/// let contents = String::from("Contents here!");
/// assert_eq!(re::replace_unordered_list(&mut "- Contents here!"), contents);
/// assert_eq!(re::replace_unordered_list(&mut "+ Contents here!"), contents);
///```
pub fn replace_unordered_list(line: &str) -> String {
    let re: Regex = Regex::new(r"^\s*[\-\+\*]{1}\s*").unwrap();
    re.replace(line, "").to_string()
}

///```
/// use saurus::transpiler::re;
/// assert!(re::is_ordered_list("7. Contents here!"));
///```
pub fn is_ordered_list(line: &str) -> bool {
    let re: Regex = Regex::new(r"^\s*\d*[\.\)]").unwrap();
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
/// assert!(re::is_code_block(&"```python".to_string()));
///```
pub fn is_code_block(line: &str) -> bool {
    let re = Regex::new(r"^\s*```").unwrap();
    re.is_match(line)
}

///```
/// use saurus::transpiler::re;
/// assert_eq!(re::replace_code_block(Some("```python")).unwrap(), "python".to_string());
///```
pub fn replace_code_block(line: Option<&str>) -> Option<String> {
    let re = Regex::new(r"\s*```(.+)").unwrap();
    if let Some(line) = line {
        let cap = re.captures(&line);
        if let Some(cap) = cap {
            let contents: &str = cap.get(1).unwrap().as_str();
            return Some(contents.to_owned());
        }
    }
    None
}

/// ```
/// use saurus::transpiler::re;
/// assert!(!re::is_block_quote(&" > Initial spaces not allowed"));
/// assert!(re::is_block_quote(&"> This is allowed"));
/// ````
pub fn is_block_quote(line: &str) -> bool {
    let re = Regex::new(r"^>\s*").unwrap();
    re.is_match(line)
}

/// ```
/// use saurus::transpiler::re;
/// assert_eq!(re::replace_block_quote(&"> this is my text"), "this is my text".to_string());
/// ````
pub fn replace_block_quote(line: &str) -> String {
    let re = Regex::new(r"^>\s*(.*)").unwrap();
    let cap = re.captures(line).unwrap();

    let contents = cap.get(1).unwrap().as_str().trim();
    contents.to_string()
}

///```
/// use saurus::transpiler::re;
/// assert_eq!(re::bold(&mut "**bold me**".to_string()), r"\textbf{bold me}".to_string());
///```
pub fn bold(line: &mut String) -> String {
    //Option<String>
    let re = Regex::new(r"\*\*([^\*]*)\*\*").unwrap();
    re.replace_all(line, |caps: &Captures| format!("\\textbf{{{}}}", &caps[1]))
        .to_string()
}

///```
/// use saurus::transpiler::re;
/// assert_eq!(re::italicize(&mut "*italicize me*".to_string()), r"\textit{italicize me}".to_string());
/// assert_ne!(re::italicize(&mut "**italicize me**".to_string()), r"\textit{italicize me}*".to_string());
///```
pub fn italicize(line: &mut String) -> String {
    //Option<String>
    let re = Regex::new(r"\*([^\*]*)\*").unwrap();
    re.replace_all(line, |caps: &Captures| format!("\\textit{{{}}}", &caps[1]))
        .to_string()
}

///```
/// use saurus::transpiler::re;
/// assert_eq!(re::bold_italicize(&mut "***italicize me***".to_string()), r"\textbf{\textit{italicize me}}".to_string());
///```
pub fn bold_italicize(line: &mut String) -> String {
    //Option<String>
    let re = Regex::new(r"\*\*\*([^\*]*)\*\*\*").unwrap();
    re.replace_all(line, |caps: &Captures| {
        format!("\\textbf{{\\textit{{{}}}}}", &caps[1])
    })
    .to_string()
}

///```
/// use saurus::transpiler::re;
/// assert_eq!(re::links(&mut "[saurus](https://github.com/paytonward6/saurus)".to_string()), r"\href{https://github.com/paytonward6/saurus}{saurus}".to_string());
/// assert_eq!(re::links(&mut "[indentfirst](https://ctan.org/pkg/indentfirst) text afterwards".to_string()), r"\href{https://ctan.org/pkg/indentfirst}{indentfirst} text afterwards".to_string());
///```
pub fn links(line: &mut String) -> String {
    // \[.*\]([^\)]*) potentially fixes
    let re = Regex::new(r"\[([a-zA-Z:][^\]]*)\]\((https://[^\)\(]*)\)").unwrap();
    re.replace_all(line, |caps: &Captures| {
        format!("\\href{{{}}}{{{}}}", &caps[2], &caps[1])
    })
    .to_string()
}

///```
/// use saurus::transpiler::re;
/// assert_eq!(re::inline_code(&mut "`let x = 2;`".to_string()), r"\verb|let x = 2;|".to_string());
///```
pub fn inline_code(line: &mut String) -> String {
    //Option<String>
    //let re = Regex::new(r"`([^`]*)`").unwrap();
    let re = Regex::new(r"`([^`]+)`").unwrap();
    re.replace_all(line, |caps: &Captures| format!("\\verb|{}|", &caps[1]))
        .to_string()
}

///```
/// use saurus::transpiler::re;
/// assert_eq!(re::symbols(&mut "=>".to_string()), r"$\rightarrow$".to_string());
/// assert_eq!(re::symbols(&mut "&rarr;".to_string()), r"$\rightarrow$".to_string());
/// assert_eq!(re::symbols(&mut "$123".to_string()), r"\$123".to_string());
/// assert_eq!(re::symbols(&mut r"$\frac{1}{2}$".to_string()), r"$\frac{1}{2}$".to_string());
/// assert_eq!(re::symbols(&mut r"You & Me".to_string()), r"You \& Me".to_string());
/// assert_eq!(re::symbols(&mut r"$Me \& You$".to_string()), r"$Me \& You$".to_string());
///```
pub fn symbols(line: &mut String) -> String {
    //Option<String>
    let arrows = Regex::new(r"=>|&rarr;").unwrap();
    let dollar_signs = Regex::new(r"\$(\d+)").unwrap();
    let ampersands = Regex::new(r"[^\\]&").unwrap();
    *line = dollar_signs
        .replace_all(line, |caps: &Captures| format!("\\${}", &caps[1]))
        .to_string();
    *line = ampersands.replace_all(line, r" \&").to_string();
    arrows.replace_all(line, "$\\rightarrow$").to_string()
}

/// uses the "ulem" package
/// ```
/// use saurus::transpiler::re;
/// assert_eq!(re::strike_out(&mut "~~strike this out~~".to_string()), r"\sout{strike this out}".to_string());
/// ```
pub fn strike_out(line: &mut String) -> String {
    //Option<String>
    let re = Regex::new(r"\~\~([^\~]*)\~\~").unwrap();
    re.replace_all(line, |caps: &Captures| format!("\\sout{{{}}}", &caps[1]))
        .to_string()
}

/// uses the "ulem" package
/// ```
/// use saurus::transpiler::re;
/// assert_eq!(re::indent_level(&"- item"), 0);
/// assert_eq!(re::indent_level(&"    - item"), 1);
/// assert_eq!(re::indent_level(&"        - item"), 2);
/// ```
pub fn indent_level(line: &str) -> usize {
    if line.is_empty() {
        0
    } else {
        // Only works for 4 space indent
        let re = Regex::new(r"(\s*)\S").unwrap();
        let cap = re.captures(line).unwrap();
        let res = cap.get(1).map_or("", |m| m.as_str());
        res.len() / 4
    }
}
