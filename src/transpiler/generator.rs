use crate::transpiler::{lexer, parser, re};

pub fn generate_line(mut contents: parser::Contents) -> Option<String> {
    type Token = lexer::Token;
    match contents.kind {
        Token::FileStart => Some(format!("\\begin{{document}}\n {}", qol_customizations())),
        Token::FileEnd => Some(format!("\\end{{document}}")),
        Token::Blank => None,
        _ => {
            // Can unwrap since any other Token's line will not be None
            // by Parser's design
            let line = transpile_line(&mut contents.line).unwrap();
            match contents.kind {
                Token::Heading(level) => match level {
                    1 => Some(format!("\\section{{{}}}\n", line)),
                    2 => Some(format!("\\subsection{{{}}}\n", line)),
                    3 => Some(format!("\\subsubsection{{{}}}\n", line)),
                    _ => Some(format!("\\subsubsection{{{}}}\n", line)),
                },
                Token::UnorderedList | Token::OrderedList(_) => Some(listify(contents)),
                Token::Text => Some(line),
                Token::CodeBlock => Some(code_block(contents)),
                Token::BlockQuote => Some(block_quote(&mut contents)),
                _ => None
            }
        }
    }
}

fn transpile_line(line: &mut Option<String>) -> Option<String> {
    if let Some(line) = line {
        *line = re::bold(line);
        *line = re::italicize(line);
        *line = re::inline_code(line);
        *line = re::strike_out(line);
        *line = re::symbols(line);
        *line = re::links(line);
        Some(line.to_string())
    } else {
        None
    }
}

fn block_quote(contents: &mut parser::Contents) -> String {
    // Can unwrap since any group item will not be None per Parser's
    // design
    let line =  re::replace_block_quote(&contents.line.as_ref().unwrap());
    type Chronology = parser::Chronology;
    match contents.chron {
        Chronology::Start => {
            format!("\\begin{{quote}}\n    {}\\\\", line)
        }
        Chronology::Middle => format!("    {}\\\\", line),
        Chronology::End => format!("    {}\n\\end{{quote}}\n", line),
        Chronology::None => format!("\\begin{{quote}}\n    {}\n\\end{{quote}}\n", line),
    }
}

fn listify(contents: parser::Contents) -> String {
    type Chronology = parser::Chronology;
    type Token = lexer::Token;
    // Can unwrap since any group item will not be None per Parser's
    // design
    let line = contents.line.unwrap();
    if let Token::UnorderedList = contents.kind {
        match contents.chron {
            Chronology::Start => {
                format!("\\begin{{itemize}}\n    \\item {}", line)
            }
            Chronology::Middle => format!("    \\item {}", line),
            Chronology::End => format!("    \\item {}\n\\end{{itemize}}\n", line),
            Chronology::None => format!(
                "\\begin{{enumerate}}\n    \\item {}\n\\end{{enumerate}}\n",
                line
            ),
        }
    } else if let Token::OrderedList(num) = contents.kind {
        match contents.chron {
            Chronology::Start => {
                format!(
                    "\\begin{{enumerate}}\n    \\setcounter{{enumi}}{{{}}}\n    \\item {}",
                num - 1, line
                )
            },
            Chronology::Middle => format!("    \\item {}", line),
            Chronology::End => format!("    \\item {}\n\\end{{enumerate}}\n", line),
            Chronology::None => format!("\\begin{{enumerate}}\n    \\setcounter{{enumi}}{{{}}}\n    \\item {}\n\\end{{enumerate}}\n", num - 1, line),
        }
    } else {
        "".to_string()
    }
}

fn code_block(content: parser::Contents) -> String {
    type Chronology = parser::Chronology;
    match content.chron {
        Chronology::Start => {
            format!(
                "\\begin{{lstlisting}}[language={}, style=myStyle]",
                content.line.unwrap()
            )
        }
        Chronology::End => "\\end{lstlisting}\n".to_owned(),
        _ => "".to_string(),
    }
}

const PACKAGES: [&str; 6] = [
    "geometry",
    "ulem",
    "listings",
    "hyperref",
    "xcolor",
    "indentfirst",
];

pub fn packages(contains_code_block: bool) -> String {
    let mut packages = String::new();
    for package in PACKAGES.into_iter() {
        packages.push_str(&format!("\\usepackage{{{}}}\n", package));
    }
    if contains_code_block {
        packages.push_str(code_block_customizations());
    }
    packages.push_str(hyperlink_customizations());
    packages
}

pub fn documentclass() -> String {
    format!("\\documentclass{{article}}\n")
}

pub fn code_block_customizations() -> &'static str {
    const CUSTOMS: &str = r"
    \definecolor{codegreen}{rgb}{0, 0.6, 0}
    \definecolor{backcolour}{rgb}{0.95,0.95,0.92}
    \lstdefinestyle{myStyle}{
        keywordstyle=\color{blue},
        identifierstyle=\color{violet},
        commentstyle=\color{codegreen},
        backgroundcolor=\color{backcolour},
        %basicstyle=\normal,
        showspaces=false,
        showstringspaces=false,
        keepspaces=true,
        extendedchars=true,
        %numbers=left,
    }";
    CUSTOMS
}

pub fn hyperlink_customizations() -> &'static str {
    const HYPERLINK: &str = r"
    \hypersetup{
        colorlinks=true,
        linkcolor=blue,
        filecolor=magenta,
        urlcolor=blue,
    }";
    HYPERLINK
}

pub fn qol_customizations() -> &'static str {
    const QOL: &str = r"
\setcounter{secnumdepth}{0}
";
    QOL
}
