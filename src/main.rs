mod color;
mod eval;
mod lexer;
mod parser;
mod run;
mod utils;

use std::{
    collections::VecDeque,
    io::{BufReader, Read},
};

use eval::Envroiment;
use run::run;

fn main() {
    let mut reader = BufReader::new(std::io::stdin());
    let mut stdin_string = String::new();
    reader.read_to_string(&mut stdin_string).unwrap();
    let file_chars: VecDeque<char> = stdin_string.chars().collect();
    let mut env = Envroiment::new();

    run(&mut env, file_chars);

    env.print_vars();
}
