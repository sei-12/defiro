use std::collections::VecDeque;

use crate::{
    eval::{eval, Envroiment}, fault::Fault, lexer::lexer, parser::parse_tokens_to_statement, utils::peek_take_while
};

pub fn run(env: &mut Envroiment, mut code_chars: VecDeque<char>) {
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
                env.faults.push(Box::new(err));
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
                env.faults.push(Box::new(err));
                continue;
            }
        };

        match eval(line_stmt, env) {
            Err(runtime_fault) => {
                env.faults.push(Box::new(runtime_fault))
            },
            _ => ()
        };
    }
}

#[cfg(test)]
mod test {

    use crate::{color::Color, eval::Envroiment, run::run};

    #[test]
    fn test_run() {
        let code = "\
        let hello = #ffffff;
        let color2 = rgb( 20, 30, 40 );
        ";
        let mut env = Envroiment::new();
        run(&mut env, code.chars().collect());
        assert_eq!(
            env.get(&"hello".to_string()),
            Some(Color {
                r: 255,
                g: 255,
                b: 255
            })
        );
        assert_eq!(
            env.get(&"color2".to_string()),
            Some(Color {
                r: 20,
                g: 30,
                b: 40
            })
        );

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
        
        color7 = rgb(10,10,10);
        ";
        let mut env = Envroiment::new();
        run(&mut env, code.chars().collect());
        assert_eq!(
            env.get(&"hello".to_string()),
            Some(Color {
                r: 255,
                g: 255,
                b: 255
            })
        );
        assert_eq!(
            env.get(&"color2".to_string()),
            Some(Color {
                r: 16,
                g: 16,
                b: 16
            })
        );
        assert_eq!(
            env.get(&"color3".to_string()),
            Some(Color {
                r: 26,
                g: 36,
                b: 46
            })
        );
        assert_eq!(
            env.get(&"color4".to_string()),
            Some(Color {
                r: 26,
                g: 36,
                b: 46
            })
        );
        assert_eq!(
            env.get(&"color5".to_string()),
            Some(Color {
                r: 10,
                g: 10,
                b: 10
            })
        );
        assert_eq!(
            env.get(&"color6".to_string()),
            Some(Color {
                r: 11,
                g: 10,
                b: 10
            })
        );
        assert_eq!(
            env.get(&"color7".to_string()),
            Some(Color {
                r: 10,
                g: 10,
                b: 10
            })
        );


        // あまりきれいなテスト方法ではない
        // 許して
        let code = "\
        include ./test/test3.txt
        ";
        let mut env = Envroiment::new();
        run(&mut env, code.chars().collect());
        assert_eq!(
            env.get(&"hello".to_string()),
            Some(Color {
                r: 10,
                g: 20,
                b: 30
            })
        );
        
        let code = "\
        include ./test/test3.txt;
        hello2 = plus(hello,10,1,2);
        ";
        let mut env = Envroiment::new();
        run(&mut env, code.chars().collect());
        assert_eq!(
            env.get(&"hello".to_string()),
            Some(Color { r: 10, g: 20, b: 30 })
        );
        assert_eq!(
            env.get(&"hello2".to_string()),
            Some(Color { r: 20, g: 21, b: 32 })
        );
        
        let code = "\
        hello2 = hello
        ";
        let mut env = Envroiment::new();
        run(&mut env, code.chars().collect());
        assert_eq!(env.faults[0].msg(),"RuntimeError: hello is Not Found".to_string());
        
        let code = "\
        include ./no_such
        ";
        let mut env = Envroiment::new();
        run(&mut env, code.chars().collect());
        assert_eq!(env.faults[0].msg(),"RuntimeError: No such file. path:./no_such".to_string());
        
        let code = "\
        include ./test/test4.txt;
        ";
        let mut env = Envroiment::new();
        run(&mut env, code.chars().collect());
        assert_eq!(env.faults[0].msg(),"RuntimeError: foo is Not Found".to_string());

        let code = "\
        hello = #fffff;
        ";
        let mut env = Envroiment::new();
        run(&mut env, code.chars().collect());
        assert_eq!(env.faults[0].msg(),"LexError: value error".to_string());

        let code = "\
        hello = #ffffff hello;
        ";
        let mut env = Envroiment::new();
        run(&mut env, code.chars().collect());
        assert_eq!(env.faults[0].msg(),"ParseError: Syntax".to_string());

        let code = "\
        hello  #ffffff;
        ";
        let mut env = Envroiment::new();
        run(&mut env, code.chars().collect());
        assert_eq!(env.faults[0].msg(),"ParseError: Syntax".to_string());

        let code = "\
        hello hello2 = #ffffff;
        ";
        let mut env = Envroiment::new();
        run(&mut env, code.chars().collect());
        assert_eq!(env.faults[0].msg(),"ParseError: Syntax".to_string());
    }
}
