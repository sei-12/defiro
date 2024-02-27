mod color;
mod fault;
mod eval;
mod lexer;
mod parser;
mod run;
mod utils;
mod app_path;

use clap::Parser;
use std::{
    collections::VecDeque, fs::read_to_string, io::{BufReader, Read}, path::PathBuf
};
use eval::Envroiment;
use run::run;

#[derive(Parser, Debug)]
struct Args {
    file_path: Option<String>    
}

fn main() {
    let args = Args::parse();

    let mut env = Envroiment::new();
    
    let file_path = match args.file_path {
        Some(file_path) => Some(PathBuf::from(&file_path)),
        None => None
    };

    let file_string = match file_path {
        Some(ref path) => { 
            read_to_string(path).unwrap()
        },
        None => {
            let mut reader = BufReader::new(std::io::stdin());
            let mut stdin_string = String::new();
            reader.read_to_string(&mut stdin_string).unwrap();
            stdin_string
        }
    };

    let file_chars: VecDeque<char> = file_string.chars().collect();

    run(&mut env, file_chars, file_path);

    env.print_vars();
    for err in env.faults {
        eprintln!("{}",err.msg());
    }
}
