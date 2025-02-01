use std::{env, fs::File, io::{Read, Write}, path::PathBuf};
mod parser;
mod tokenizer;
mod tokens;

use clap::*;

use parser::*;
use tokenizer::*;

fn main() {
    let cwd = env::current_dir().unwrap();

    // Some clap arg parsing and fun stuff
    let shared = Arg::new("FILE").value_parser(clap::value_parser!(PathBuf));
    let matches = Command::new("brainrot")
        .subcommand_required(true)
        .subcommand(
            Command::new("run")
            .about("Executes a brainrot .rot file")
            .arg(shared.clone())
            .arg_required_else_help(true)
        )
        .subcommand(
            Command::new("tokenize")
            .about("Tokenizes the input only. Prints out a list of tokens")
            .arg(shared)
            .arg_required_else_help(true)
        )
        .subcommand(
            Command::new("proompter")
            .about("Become a proomter.")
        )
        .get_matches();

    let subcommand = matches.subcommand();
    match &subcommand {
        Some(("tokenize", sub_matches)) | Some(("run", sub_matches)) => {
            let relative_path = sub_matches.get_one::<PathBuf>("FILE").expect("required");
            let mut file_path = cwd;
            file_path.push(relative_path);

            // Opens the source file with the targetted path
            let mut file = File::open(file_path).expect("could not open file");
            let mut raw = String::new();
            file.read_to_string(&mut raw).unwrap();

            let tokenized = tokenize_raw(raw).unwrap();
            if let Some(("tokenize", _)) = subcommand {
                for token in tokenized {
                    println!("{:?}", token);
                }
            } else {
                let mut world = World::default();
                parse_and_execute(tokenized, &mut world).unwrap();
            }
        },
        Some(("proompter", sub_matches)) => {
            let mut world = World::default();
            loop {
                print!(">  ");
                std::io::stdout().flush().unwrap();
                let input = std::io::stdin().lines().next().unwrap().unwrap();
                tokenize_raw(input).map(|tokens| parse_and_execute(tokens, &mut world));
            }
        },
        _ => todo!(),
    }
}