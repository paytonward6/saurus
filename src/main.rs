use std::{fs, path::PathBuf, process};

use clap::{arg, command, Parser};

use saurus::transpiler;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    input: String,
    #[arg(short, long)]
    output: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    let input = PathBuf::from(cli.input);
    if !input.exists() {
        eprintln!("{:?} does not exist or file permissions deny use!", input);
        process::exit(1);
    }

    let output = if let Some(output) = cli.output {
        PathBuf::from(output)
    } else {
        let mut file = input.clone();
        file.set_extension("tex");
        file
    };

    let file_str = fs::read_to_string(input).expect("Unable to read from file!");
    transpiler::run(&file_str, &output);
}
