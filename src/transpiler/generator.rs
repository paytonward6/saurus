use crate::transpiler::{lexer, parser, re};

pub fn generate_line(mut contents: parser::Contents) -> Option<String> {
    type Token = lexer::Token;
    contents.line = transpile_line(&mut contents.line);
    match contents.kind {
        Token::FileStart => Some(format!("\\begin{{document}}\n {}", qol_customizations())),
        Token::FileEnd => Some(format!("\\end{{document}}")),
        Token::Heading(level) => match level {
            1 => Some(format!("\\section{{{}}}\n", contents.line)),
            2 => Some(format!("\\subsection{{{}}}\n", contents.line)),
            3 => Some(format!("\\subsubsection{{{}}}\n", contents.line)),
            _ => Some(format!("\\subsubsection{{{}}}\n", contents.line)),
        },
        Token::UnorderedList | Token::OrderedList(_) => Some(listify(contents)),
        Token::Text => Some(contents.line),
        Token::CodeBlock => Some(code_block(contents)),
        Token::BlockQuote => Some(block_quote(&mut contents)),
        _ => None,
    }
}

fn transpile_line(line: &mut String) -> String {
    *line = re::bold(line);
    *line = re::italicize(line);
    *line = re::inline_code(line);
    *line = re::strike_out(line);
    *line = re::symbols(line);
    *line = re::links(line);
    line.to_string()
}

fn block_quote(contents: &mut parser::Contents) -> String {
    contents.line = re::replace_block_quote(&contents.line);
    type Chronology = parser::Chronology;
    match contents.chron {
        Chronology::Start => {
            format!("\\begin{{quote}}\n    {}\\\\", contents.line)
        }
        Chronology::Middle => format!("    {}\\\\", contents.line),
        Chronology::End => format!("    {}\n\\end{{quote}}\n", contents.line),
        Chronology::None => format!("\\begin{{quote}}\n    {}\n\\end{{quote}}\n", contents.line),
    }
}

fn listify(contents: parser::Contents) -> String {
    type Chronology = parser::Chronology;
    type Token = lexer::Token;
    if let Token::UnorderedList = contents.kind {
        match contents.chron {
            Chronology::Start => {
                format!("\\begin{{itemize}}\n    \\item {}", contents.line)
            }
            Chronology::Middle => format!("    \\item {}", contents.line),
            Chronology::End => format!("    \\item {}\n\\end{{itemize}}\n", contents.line),
            Chronology::None => format!(
                "\\begin{{enumerate}}\n    \\item {}\n\\end{{enumerate}}\n",
                contents.line
            ),
        }
    } else if let Token::OrderedList(num) = contents.kind {
        match contents.chron {
            Chronology::Start => {
                format!(
                    "\\begin{{enumerate}}\n    \\setcounter{{enumi}}{{{}}}\n    \\item {}",
                num - 1, contents.line
                )
            },
            Chronology::Middle => format!("    \\item {}", contents.line),
            Chronology::End => format!("    \\item {}\n\\end{{enumerate}}\n", contents.line),
            Chronology::None => format!("\\begin{{enumerate}}\n    \\setcounter{{enumi}}{{{}}}\n    \\item {}\n\\end{{enumerate}}\n", num - 1, contents.line),
        }
    } else {
        "".to_string()
    }
}

fn code_block(content: parser::Contents) -> String {
    type Chronology = parser::Chronology;
    let language = "python";
    match content.chron {
        Chronology::Start => {
            format!(
                "\\begin{{lstlisting}}[language={}, style=myStyle]",
                language
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
