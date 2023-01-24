use crate::transpiler::{Token, TokenKind};

const PACKAGES: [&str; 6] = ["geometry", "ulem", "listings", "hyperref", "xcolor", "indentfirst"];

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

pub fn body(token: &Token) -> Option<String> {
    match &token.kind {
        TokenKind::FileStart => Some(format!("\\begin{{document}}\n {}", qol_customizations())),
        TokenKind::FileEnd => Some(format!("\\end{{document}}")),
        TokenKind::Text => Some(format!("{}\n", token.contents.as_ref().unwrap())),
        TokenKind::Heading(level) => match level {
            1 => Some(format!("\\section{{{}}}", token.contents.as_ref().unwrap())),
            2 => Some(format!(
                "\\subsection{{{}}}",
                token.contents.as_ref().unwrap()
            )),
            3 => Some(format!(
                "\\subsubsection{{{}}}",
                token.contents.as_ref().unwrap()
            )),
            _ => None,
        },

        TokenKind::BeginUnorderedList => Some(format!("\\begin{{itemize}}")),
        TokenKind::UnorderedListItem(_) => {
            Some(format!("    \\item {}", token.contents.as_ref().unwrap()))
        }
        TokenKind::EndUnorderedList => Some(format!("\\end{{itemize}}\n")),

        TokenKind::BeginOrderedList(num) => Some(format!(
            "\\begin{{enumerate}}\n    \\setcounter{{enumi}}{{{}}}",
            num - 1
        )),
        TokenKind::OrderedListItem(_) => {
            Some(format!("    \\item {}", token.contents.as_ref().unwrap()))
        }
        TokenKind::EndOrderedList => Some(format!("\\end{{enumerate}}\n")),

        TokenKind::BeginCodeBlock(language) => Some(format!("\\begin{{lstlisting}}[language={}, style=myStyle]", <&crate::transpiler::code_blocks::Languages as Into<&str>>::into(language))),
        TokenKind::BodyCodeBlock => Some(token.contents.as_ref().unwrap().to_string()),
        TokenKind::EndCodeBlock => Some(format!("\\end{{lstlisting}}\n")),

        TokenKind::BeginBlockQuote => Some(format!("\\begin{{quote}}")),
        // Formatting for BlockQuote done in Transpiler
        TokenKind::BodyBlockQuote => Some(format!("{}", token.contents.as_ref().unwrap())),
        TokenKind::EndBlockQuote => Some(format!("\\end{{quote}}\n")),
 
        _ => None,
    }
}

pub fn code_block_customizations() -> &'static str {
    const CUSTOMS: &str = 
    r"
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
    const HYPERLINK: &str = 
    r"
    \hypersetup{
        colorlinks=true,
        linkcolor=blue,
        filecolor=magenta,
        urlcolor=blue,
    }";
    HYPERLINK
}

pub fn qol_customizations() -> &'static str {
    const QOL: &str = 
    r"
\setcounter{secnumdepth}{0}

    ";
    QOL
}
