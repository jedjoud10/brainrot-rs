use std::{env, fs::File, io::Read, path::PathBuf};
mod parser;
mod tokenizer;
mod tokens;

use clap::*;

use parser::*;
use tokenizer::*;

fn main() {
    let cwd = env::current_dir().unwrap();

    // Some clap arg parsing and fun stuff
    let matches = Command::new("brainrot")
        .arg(Arg::new("FILE").value_parser(clap::value_parser!(PathBuf)))
        .arg_required_else_help(true)
        .subcommand_required(true)
        .subcommand(
            Command::new("run")
            .about("Executes a brainrot .rot file")
        )
        .subcommand(
            Command::new("tokenize")
            .about("Tokenizes the input only. Prints out a list of tokens")
        ).get_matches();

    let relative_path = matches.get_one::<PathBuf>("FILE").expect("required");
    let mut file_path = cwd;
    file_path.push(relative_path);

    // Opens the source file with the targetted path
    let mut file = File::open(file_path).expect("could not open file");
    let mut raw = String::new();
    file.read_to_string(&mut raw).unwrap();

    match matches.subcommand() {
        Some(("tokenize", sub_matches)) => {
            for token in tokenize_raw(raw) {
                println!("{:?}", token);
            }

        },
        Some(("run", sub_matches)) => {
            parse_and_execute(tokenize_raw(raw))
        },
        _ => todo!(),
    }
}