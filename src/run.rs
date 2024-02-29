use std::collections::VecDeque;

use crate::{
    app_path::AbsFilePath, envroiment::Envroiment, eval::eval, lexer::lexer, parser::parse_tokens_to_statement, utils::peek_take_while
};

pub fn run(env: &mut Envroiment, mut code_chars: VecDeque<char>, file_path: AbsFilePath ) {
    let result = env.include_file_stack.push(file_path); 
    if let Err(err) = result {
        env.faults.push(Box::new(err));
        return;
    }

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
    
    env.include_file_stack.pop();
}

#[cfg(test)]
mod test {

    use crate::{app_path, color::Color, envroiment::Envroiment, fault, parser::ParseFault, run::run};

    #[test]
    fn test_run() {
        let code = "\
        let hello = #ffffff;
        let color2 = rgb( 20, 30, 40 );
        ";
        let mut env = Envroiment::new();
        run(&mut env, code.chars().collect(),app_path::AbsFilePath::create_decoy());
        assert_eq!(
            env.get(&"hello".to_string()),
            Some(Color::new(255, 255, 255))
        );
        assert_eq!(
            env.get(&"color2".to_string()),
            Some(Color::new(20, 30, 40))
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
        run(&mut env, code.chars().collect(),app_path::AbsFilePath::create_decoy());
        assert_eq!(
            env.get(&"hello".to_string()),
            Some(Color ::new( 255,  255,  255 ))
        );
        assert_eq!(
            env.get(&"color2".to_string()),
            Some(Color ::new( 16,  16,  16 ))
        );
        assert_eq!(
            env.get(&"color3".to_string()),
            Some(Color ::new( 26,  36,  46 ))
        );
        assert_eq!(
            env.get(&"color4".to_string()),
            Some(Color ::new( 26,  36,  46 ))
        );
        assert_eq!(
            env.get(&"color5".to_string()),
            Some(Color ::new( 10,  10,  10 ))
        );
        assert_eq!(
            env.get(&"color6".to_string()),
            Some(Color ::new( 11,  10,  10 ))
        );
        assert_eq!(
            env.get(&"color7".to_string()),
            Some(Color ::new( 10,  10,  10 ))
        );

        let code = "\
        hello = rgb(255,150,0);
        hello2 = plus(hello,1,200,100);
        ";
        let mut env = Envroiment::new();
        run(&mut env, code.chars().collect(),app_path::AbsFilePath::create_decoy());
        assert_eq!(
            env.get(&"hello".to_string()),
            Some(Color ::new( 255,  150,  0 ))
        );
        assert_eq!(
            env.get(&"hello2".to_string()),
            Some(Color ::new( 255,  255,  100 ))
        );

        let code = "\
        hello = rgb(0,0,0);
        hello2 = minus(hello,1,1,1);
        ";
        let mut env = Envroiment::new();
        run(&mut env, code.chars().collect(),app_path::AbsFilePath::create_decoy());
        assert_eq!(
            env.get(&"hello".to_string()),
            Some(Color ::new( 0,  0,  0 ))
        );
        assert_eq!(
            env.get(&"hello2".to_string()),
            Some(Color ::new( 0,  0,  0 ))
        );

        let code = "\
        hello2 = hello
        ";
        let mut env = Envroiment::new();
        run(&mut env, code.chars().collect(),app_path::AbsFilePath::create_decoy());
        assert_eq!(env.faults[0].msg(),"RuntimeError: hello is Not Found".to_string());
        
        let code = "\
        hello = #fffff;
        ";
        let mut env = Envroiment::new();
        run(&mut env, code.chars().collect(),app_path::AbsFilePath::create_decoy());
        assert_eq!(env.faults[0].msg(),"LexError: value error".to_string());

        let code = "\
        hello = #ffffff hello;
        ";
        let mut env = Envroiment::new();
        run(&mut env, code.chars().collect(),app_path::AbsFilePath::create_decoy());
        assert_eq!(env.faults[0].msg(),"ParseError: Syntax".to_string());

        let code = "\
        hello  #ffffff;
        ";
        let mut env = Envroiment::new();
        run(&mut env, code.chars().collect(),app_path::AbsFilePath::create_decoy());
        assert_eq!(env.faults[0].msg(),"ParseError: Syntax".to_string());

        let code = "\
        hello hello2 = #ffffff;
        ";
        let mut env = Envroiment::new();
        run(&mut env, code.chars().collect(),app_path::AbsFilePath::create_decoy());
        assert_eq!(env.faults[0].msg(),"ParseError: Syntax".to_string());


        test_run_(
            "// let hello = #ffffff",
            vec![],
            vec![]
        );

        test_run_(
            "\
            hello = #ffffff; // hello
            // hello world
            let aaa = rgb( 10,10,10 );
            ",
            vec![
                ("hello",Color::new(0xff, 0xff, 0xff)),
                ("aaa",Color::new(10, 10,10)),
            ],
            vec![]
        );

        test_run_(
            "\
            let hello = rgb(
                10,
                // aaa
                10,
                // hello
                10
            )",
            vec![
                ("hello",Color::new(10, 10, 10))
            ],
            vec![]
        );

        test_run_(
            "let hello = #ffffff // hello",
            vec![
                ("hello",Color::new(0xff, 0xff, 0xff))
            ],
            vec![]
        );

        test_run_(
            "/ hello /",
            vec![ ],
            vec![ Box::new(ParseFault::Syntax)]
        );

        test_run_(
            "/// // let hello",
            vec![ ],
            vec![ ]
        );

        test_run_(
            "let hello = #ffffff/hello",
            vec![ ],
            vec![
                Box::new(ParseFault::Syntax)
            ]
        );

        test_run_(
            "let hello = #ffffff//hello",
            vec![ 
                ("hello",Color::new(255, 255, 255))
            ],
            vec![ ]
        );

        test_run_(
            "// // let hello",
            vec![ ],
            vec![ ]
        );
    }
    
    fn test_run_(
        code: &str,
        vars: Vec<(&str,Color)>,
        errs: Vec<Box<dyn fault::Fault>>
    ){
        println!("{}",code);
        let mut env = Envroiment::new();
        run(&mut env, code.chars().collect(), app_path::AbsFilePath::create_decoy());
        assert_eq!(env.vars_len(),vars.len());
        assert_eq!(env.faults.len(),errs.len());

        for err in errs.iter().zip(&env.faults) {
            assert_eq!(err.0.msg(),err.1.msg())
        }

        for var in vars {
            assert_eq!(env.get(&var.0.to_string()).unwrap(),var.1);
        }
    }
}
