mod color;
mod eval;
mod parser;
mod utils;
mod lexer;

use std::{
    collections::VecDeque, io::{BufReader, Read}
};
use eval::{eval, Envroiment};
use lexer::lexer;
use parser::parse_tokens_to_statement;
use utils::peek_take_while;


fn main() {
    let mut reader = BufReader::new(std::io::stdin());
    let mut stdin_string = String::new();
    reader.read_to_string(&mut stdin_string).unwrap();
    let mut file_chars: VecDeque<char> = stdin_string.chars().collect();
    let mut env = Envroiment::new();

    loop {
        if file_chars.front().is_none() {
            break;
        }
        
        let line = peek_take_while(&mut file_chars, |&ch| ch == ';');
        file_chars.pop_front();

        let line_string: String = line.into_iter().collect();
        let mut tmp = line_string.chars().collect();
        let tokens = lexer(&mut tmp).unwrap();
        let line_stmt = parse_tokens_to_statement(tokens).unwrap();
        eval(line_stmt, &mut env)
    }

    env.print();
}
