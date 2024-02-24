mod color;
mod eval;
mod lexer;
mod parser;
mod utils;

use eval::{eval, Env, LocalEnvroiment, RootEnvroiment};
use lexer::lexer;
use parser::parse_tokens_to_statement;
use std::{
    collections::VecDeque, io::{BufReader, Read}, sync::{Arc, Mutex}
};
use utils::peek_take_while;

fn run(parent_env: Arc<Mutex<dyn Env>>, mut code_chars: VecDeque<char>) -> LocalEnvroiment {
    let mut env = LocalEnvroiment::new(parent_env);

    loop {
        if code_chars.front().is_none() {
            break;
        }

        let line = peek_take_while(&mut code_chars, |&ch| ch == ';');
        code_chars.pop_front();

        let line_string: String = line.into_iter().collect();
        let mut chars = line_string.chars().collect();
        let tokens = match lexer(&mut chars) {
            Ok(tokens) => tokens,
            Err(err) => {
                println!("{}", err.msg());
                continue;
            }
        };

        if tokens.len() == 0 {
            continue;
        }
        let line_stmt = match parse_tokens_to_statement(tokens) {
            Ok(stmt) => stmt,
            Err(err) => {
                println!("{}", err.msg());
                continue;
            }
        };
        let result = eval(line_stmt, &mut env);

        if let Err(runtime_fault) = result {
            runtime_fault.print_msg();
        };
    }

    env
}

fn main() {
    let mut reader = BufReader::new(std::io::stdin());
    let mut stdin_string = String::new();
    reader.read_to_string(&mut stdin_string).unwrap();
    let file_chars: VecDeque<char> = stdin_string.chars().collect();
    let env = Arc::new(Mutex::new(RootEnvroiment::new()));

    let child_env = run(env.clone(), file_chars);
    
    child_env.print_vars();
}


#[cfg(test)]
mod test {
    use std::sync::{Arc, Mutex};

    use crate::{color::Color, eval::{Env, RootEnvroiment}, run};

    #[test]
    fn test_run(){
        let code = "\
        let hello = #ffffff;
        let color2 = rgb( 20, 30, 40 );
        ";
        let result = run(
            Arc::new(Mutex::new(RootEnvroiment::new())),
            code.chars().collect()
        );
        assert_eq!(result.get(&"hello".to_string()),   Some(Color{ r: 255, g: 255, b: 255 }));
        assert_eq!(result.get(&"color2".to_string()),  Some(Color{ r:  20, g:  30, b:  40 }));
        

        
        let code = "\
        let hello = #ffffff;
        let color2 = rgb( 20, 30, 40 );
        let color2 = #101010;
        let color3 = plus( color2, 10, 20, 30 );
        let color4 = minus( 
            plus( color3 ,1,2,3) , 1,2,3
        );
        let color5 = minus( 
            plus( 
                minus(
                    plus(
                        rgb(1,2,3),
                        10,10,10
                    ),
                    1,2,3
                ),
                1,2,3
            ), 
            1,2,3
        );
        let color6 = minus( 
            plus( 
                minus(
                    plus(
                        #010203,
                        11,10,10
                    ),
                    1,2,3
                ),
                1,2,3
            ), 
            1,2,3
        );
        ";        
        let result = run(
            Arc::new(Mutex::new(RootEnvroiment::new())),
            code.chars().collect()
        );
        assert_eq!(result.get(&"hello".to_string()),   Some(Color{ r: 255, g: 255, b: 255 }));
        assert_eq!(result.get(&"color2".to_string()),  Some(Color{ r:  16, g:  16, b:  16 }));
        assert_eq!(result.get(&"color3".to_string()),  Some(Color{ r:  26, g:  36, b:  46 }));
        assert_eq!(result.get(&"color4".to_string()),  Some(Color{ r:  26, g:  36, b:  46 }));
        assert_eq!(result.get(&"color5".to_string()),  Some(Color{ r:  10, g:  10, b:  10 }));
        assert_eq!(result.get(&"color6".to_string()),  Some(Color{ r:  11, g:  10, b:  10 }));

    }
}