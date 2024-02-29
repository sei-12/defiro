mod app_path;
mod color;
mod envroiment;
mod eval;
mod fault;
mod lexer;
mod parser;
mod run;
mod utils;

use app_path::AbsFilePath;
use clap::Parser;
use envroiment::Envroiment;
use run::run;
use std::{
    collections::VecDeque,
    fs::{self, read_to_string},
    path::PathBuf,
};

#[derive(Parser, Debug)]
struct Args {
    file_path: String,
}

fn main() {
    let args = Args::parse();

    let mut env = Envroiment::new();

    let file_path = PathBuf::from(&args.file_path);
    let file_string = read_to_string(&file_path).unwrap();

    let file_chars: VecDeque<char> = file_string.chars().collect();
    let tmp = fs::canonicalize(file_path).unwrap();
    let abs_path = tmp.to_str().unwrap();

    let abs_file_path = AbsFilePath::from_string(abs_path).unwrap();

    run(&mut env, file_chars, abs_file_path);

    println!("{}", env.vars_json());

    for err in env.faults {
        eprintln!("{}", err.msg());
    }
}
