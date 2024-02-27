mod color;
mod fault;
mod eval;
mod lexer;
mod parser;
mod run;
mod utils;
mod app_path;

use app_path::AbsFilePath;
use clap::Parser;
use std::{
    collections::VecDeque, fs::{self, read_to_string}, path::PathBuf
};
use eval::Envroiment;
use run::run;

#[derive(Parser, Debug)]
struct Args {
    file_path: String   
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

    env.print_vars();
    for err in env.faults {
        eprintln!("{}",err.msg());
    }
}
