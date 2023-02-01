use crate::transpiler::{lexer, parser};
pub fn generate_line(contents: parser::Contents) -> String {
    type Token = lexer::Token;
    type Chronology = parser::Chronology;
    match contents.kind {
        Token::Heading(level) => match level {
            1 => format!("\\section{{{}}}", contents.line),
            2 => format!(
                "\\subsection{{{}}}",
                contents.line
            ),
            3 => format!(
                "\\subsubsection{{{}}}",
                contents.line
            ),
            _ => format!(
                "\\subsubsection{{{}}}",
                contents.line)
        },
        Token::UnorderedList => {
            match contents.chron {
                Chronology::Start => {
format!("\\begin{{itemize}}\n    \\item {}", contents.line)
                }
                Chronology::Middle => "\n".to_owned()

            }

        }
        _ => "\n".to_owned()

    }
}
