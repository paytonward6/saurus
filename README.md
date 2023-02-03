# saurus

Markdown &rarr; LaTeX transpiler

## LaTeX packages required
- [geometry](https://ctan.org/pkg/geometry): margins
- [ulem](https://ctan.org/pkg/ulem): ~~strikeout~~ functionality
- [listings](https://ctan.org/pkg/listings): source code blocks (list of all available languages can be found in [src/transpiler/code_blocks.rs](https://github.com/paytonward6/saurus/blob/main/src/transpiler/code_blocks.rs))
- [hyperref](https://ctan.org/pkg/hyperref): hyperlinks
- [xcolor](https://ctan.org/pkg/xcolor): robust colors
- [indentfirst](https://ctan.org/pkg/indentfirst): indents first paragraph after section heading ([required package](https://ctan.org/pkg/required) in all LaTeX distributions)
