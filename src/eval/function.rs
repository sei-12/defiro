use crate::{color::Color, envroiment::Envroiment, parser:: Expression};

use super::{eval_expression, EvalFault, Value};


pub (super) fn eval_plus_function(mut args: Vec<Expression>, env: &mut Envroiment) -> Result<Value, EvalFault> {
    if args.len() != 4 {
        return Err(EvalFault::NumArgments { req: 4, got: args.len() })
    };

    let Expression::Int(b) = args.pop().expect("bug") else {
        return Err(EvalFault::ArgType);
    };

    let Expression::Int(g) = args.pop().expect("bug") else {
        return Err(EvalFault::ArgType);
    };

    let Expression::Int(r) = args.pop().expect("bug") else {
        return Err(EvalFault::ArgType);
    };

    let value = eval_expression(args.pop().expect("bug"), env)?;

    let Value::Color(color) = value else {
        return Err(EvalFault::ArgType);
    };

    Ok(Value::Color(color.plus(r, g, b)))
}

pub (super) fn eval_rgb_function(args: Vec<Expression>) -> Result<Value, EvalFault> {
    if args.len() != 3 {
        return Err(EvalFault::NumArgments { req: 3, got: args.len() })
    };
    let Expression::Int(r) = args[0] else {
        return Err(EvalFault::ArgType);
    };
    let Expression::Int(g) = args[1] else {
        return Err(EvalFault::ArgType);
    };
    let Expression::Int(b) = args[2] else {
        return Err(EvalFault::ArgType);
    };

    Ok(Value::Color(Color::new(r, g, b)))
}

pub (super) fn eval_minus_function(mut args: Vec<Expression>,env: &mut Envroiment) -> Result<Value, EvalFault> {
    if args.len() != 4 {
        return Err(EvalFault::NumArgments { req: 4, got: args.len() })
    };

    let Expression::Int(b) = args.pop().expect("bug") else {
        return Err(EvalFault::ArgType);
    };

    let Expression::Int(g) = args.pop().expect("bug") else {
        return Err(EvalFault::ArgType);
    };

    let Expression::Int(r) = args.pop().expect("bug") else {
        return Err(EvalFault::ArgType);
    };

    let value = eval_expression(args.pop().expect("bug"), env)?;

    let Value::Color(color) = value else {
        return Err(EvalFault::ArgType);
    };

    Ok(Value::Color(color.minus(r, g, b)))
}

#[cfg(test)]
mod test {
    use std::vec;

    use crate::{color::Color, envroiment::Envroiment, eval::function::{eval_minus_function, eval_plus_function}, parser::Expression};

    use super::{EvalFault, Value};


    #[test]
    fn test_eval_minus_func(){
        assert_eval_minus_func_ok(
            vec![
                Expression::Color(Color::new(110, 110, 110)),
                Expression::Int(10),
                Expression::Int(10),
                Expression::Int(10),
            ], 
            vec![], 
            Value::Color(Color::new(100, 100,100))
        );
        assert_eval_minus_func_ok(
            vec![
                Expression::Color(Color::new(0, 110, 254)),
                Expression::Int(1),
                Expression::Int(111),
                Expression::Int(255),
            ], 
            vec![], 
            Value::Color(Color::new(0, 0,0))
        );
        assert_eval_minus_func_ok(
            vec![
                Expression::Identifier("hello".to_string()),
                Expression::Int(100),
                Expression::Int(10),
                Expression::Int(0),
            ],
            vec![
                ("hello",Color::new(100, 200, 100))
            ],
            Value::Color(Color::new(0,190,100))
        );
    }

    #[test]
    fn test_eval_plus_func(){
        assert_eval_plus_func_ok(
            vec![
                Expression::Color(Color::new(10, 10, 10)),
                Expression::Int(10),
                Expression::Int(10),
                Expression::Int(10),
            ],
            vec![],
            Value::Color(Color::new(20, 20, 20))
        );

        assert_eval_plus_func_ok(
            vec![
                Expression::Identifier("hello".to_string()),
                Expression::Int(100),
                Expression::Int(10),
                Expression::Int(0),
            ],
            vec![
                ("hello",Color::new(100, 200, 255))
            ],
            Value::Color(Color::new(200,210,255))
        );

        assert_eval_plus_func_ok(
            vec![
                Expression::Color(Color::new(255, 100, 10)),
                Expression::Int(100),
                Expression::Int(10),
                Expression::Int(0),
            ],
            vec![],
            Value::Color(Color::new(255, 110, 10))
        );
    }
    
    #[test]
    fn test_eval_plus_func_err(){
        assert_eval_plus_func_err(
            vec![], 
            vec![], 
            EvalFault::NumArgments { req: 4, got: 0 }
        );

        assert_eval_plus_func_err(
            vec![
                Expression::Int(1),
                Expression::Int(1),
                Expression::Int(1),
                Expression::Int(1),
                Expression::Int(1),
            ], 
            vec![], 
            EvalFault::NumArgments { req: 4, got: 5 }
        );

        assert_eval_plus_func_err(
            vec![
                Expression::Int(1)
            ], 
            vec![], 
            EvalFault::NumArgments { req: 4, got: 1 }
        );

        assert_eval_plus_func_err(
            vec![
                Expression::Int(0),
                Expression::Int(100),
                Expression::Int(10),
                Expression::Int(0),
            ],
            vec![],
            EvalFault::ArgType 
        );

        assert_eval_plus_func_err(
            vec![
                Expression::Int(0),
                Expression::Color(Color::new(10, 10, 10)),
                Expression::Int(10),
                Expression::Int(0),
            ],
            vec![],
            EvalFault::ArgType 
        );

    }
    
    #[test]
    fn test_eval_minus_func_err(){
        assert_eval_minus_func_err(
            vec![], 
            vec![], 
            EvalFault::NumArgments { req: 4, got: 0 }
        );

        assert_eval_minus_func_err(
            vec![
                Expression::Int(1),
                Expression::Int(1),
                Expression::Int(1),
                Expression::Int(1),
                Expression::Int(1),
            ], 
            vec![], 
            EvalFault::NumArgments { req: 4, got: 5 }
        );

        assert_eval_minus_func_err(
            vec![
                Expression::Int(1)
            ], 
            vec![], 
            EvalFault::NumArgments { req: 4, got: 1 }
        );

        assert_eval_minus_func_err(
            vec![
                Expression::Int(0),
                Expression::Int(100),
                Expression::Int(10),
                Expression::Int(0),
            ],
            vec![],
            EvalFault::ArgType 
        );

        assert_eval_minus_func_err(
            vec![
                Expression::Int(0),
                Expression::Color(Color::new(10, 10, 10)),
                Expression::Int(10),
                Expression::Int(0),
            ],
            vec![],
            EvalFault::ArgType 
        );

    }
    fn assert_eval_minus_func_err(
        args: Vec<Expression>,
        env_vars: Vec<(&str,Color)>,
        assert_val: EvalFault
    ){
        let mut env = Envroiment::new();
        for var in env_vars {
            env.set(var.0.to_string(), var.1)            
        }

        let result = eval_minus_function(args, &mut env).unwrap_err();
        
        assert_eq!(result,assert_val);
    }

    fn assert_eval_minus_func_ok(
        args: Vec<Expression>,
        env_vars: Vec<(&str,Color)>,
        assert_val: Value
    ){
        let mut env = Envroiment::new();
        for var in env_vars {
            env.set(var.0.to_string(), var.1)            
        }

        let result = eval_minus_function(args, &mut env).unwrap();
        
        assert_eq!(result,assert_val);
    }
    fn assert_eval_plus_func_err(
        args: Vec<Expression>,
        env_vars: Vec<(&str,Color)>,
        assert_val: EvalFault
    ){
        let mut env = Envroiment::new();
        for var in env_vars {
            env.set(var.0.to_string(), var.1)            
        }

        let result = eval_plus_function(args, &mut env).unwrap_err();
        
        assert_eq!(result,assert_val);
    }

    fn assert_eval_plus_func_ok(
        args: Vec<Expression>,
        env_vars: Vec<(&str,Color)>,
        assert_val: Value
    ){
        let mut env = Envroiment::new();
        for var in env_vars {
            env.set(var.0.to_string(), var.1)            
        }

        
        let result = eval_plus_function(args, &mut env).unwrap();
        
        assert_eq!(result,assert_val);
    }
}