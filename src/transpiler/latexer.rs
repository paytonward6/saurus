use crate::transpiler::{Token, TokenKind};

const PACKAGES: [&str; 5] = ["geometry", "ulem", "listings", "hyperref", "xcolor"];

pub fn packages() -> String {
    let mut packages = String::new();
    for package in PACKAGES.into_iter() {
        packages.push_str(&format!("\\usepackage{{{}}}\n", package));
    }
    packages.push_str(package_customizations());
    packages
}

pub fn documentclass() -> String {
    format!("\\documentclass{{article}}\n")
}

pub fn body(token: &Token) -> Option<String> {
    match &token.kind {
        TokenKind::FileStart => Some(format!("\\begin{{document}}")),
        TokenKind::FileEnd => Some(format!("\\end{{document}}")),
        TokenKind::Text => Some(token.contents.as_ref().unwrap().to_string()),
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
        _ => None,
    }
}

pub fn package_customizations() -> &'static str {
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
    }

    \hypersetup{
        colorlinks=true,
        linkcolor=blue,
        filecolor=magenta,
        urlcolor=blue,
    }";
    CUSTOMS
}
