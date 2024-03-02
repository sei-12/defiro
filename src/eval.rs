// use std::fs;
use std::fs::read_to_string;

use crate::app_path::{self, AbsFilePathError};
use crate::color::ColorInt;
use crate::envroiment::Envroiment;
use crate::parser::Expression;
use crate::{
    color::Color,
    fault,
    parser::{Call, IncludeStatement, LetStatement, Statement},
    run::run,
};

#[derive(Debug,PartialEq)]
pub enum EvalFault {
    NotFound { target_name: String },
    NoSuchFile { path: String },
    TodoRename2 { err: AbsFilePathError },
    IsNotFunction { target_name: String },
    NumArgments { req: usize, got: usize },
    ArgType // { req: String, got: String },
}

#[derive(Debug,PartialEq)]
enum Value {
    Color(Color),
    Int(ColorInt),
}

impl From<AbsFilePathError> for EvalFault {
    fn from(value: AbsFilePathError) -> Self {
        EvalFault::TodoRename2 { err: value }
    }
}

impl fault::Fault for EvalFault {
    fn msg(&self) -> String {
        match self {
            EvalFault::NotFound { target_name } => {
                format!("EvalError: {} is Not Found", target_name)
            }
            EvalFault::NoSuchFile { path } => {
                format!("EvalError: No such file. path:{}", path)
            }
            EvalFault::TodoRename2 { err } => format!("EvalError: {:?}", err),
            EvalFault::IsNotFunction { target_name } => {
                format!("EvalError: {} is not function", target_name)
            },
            EvalFault::NumArgments { req, got } => {
                format!("EvalError: Wrong number of arguments. req={} got={}",req,got)
            },
            // EvalFault::ArgType { req, got } => {
            //     format!("EvalError: The type of the argument is differentent. req={} got={}",req,got)
            // }
            EvalFault::ArgType => {
                format!("EvalError: The type of the argument is differentent.")
            }
        }
    }
}

pub fn eval_include_stmt(
    include_stmt: IncludeStatement,
    env: &mut Envroiment,
) -> Result<(), EvalFault> {
    let current_file_path = env.include_file_stack.get_current_file();
    let file_path = app_path::join_or_abs(current_file_path, &include_stmt.path)?;
    let file_string = match read_to_string(file_path.get()) {
        Ok(str) => str,
        Err(_) => {
            return Err(EvalFault::NoSuchFile {
                path: include_stmt.path,
            });
        }
    };

    run(env, file_string.chars().collect(), file_path);

    Ok(())
}

pub fn eval(stmt: Statement, env: &mut Envroiment) -> Result<(), EvalFault> {
    match stmt {
        Statement::Let(let_stmt) => eval_let_statement(let_stmt, env),
        Statement::Include(include_stmt) => eval_include_stmt(include_stmt, env),
    }
}

fn eval_let_statement(let_stmt: LetStatement, env: &mut Envroiment) -> Result<(), EvalFault> {
    let value = eval_expression(let_stmt.right, env)?;
    let Value::Color(color) = value else { todo!() };

    env.set(let_stmt.left, color);
    Ok(())
}

fn eval_identifer(name: String, env: &mut Envroiment) -> Result<Value, EvalFault> {
    match env.get(&name) {
        Some(color) => Ok(Value::Color(color)),
        None => Err(EvalFault::NotFound { target_name: name }),
    }
}

fn eval_call(call: Call, env: &mut Envroiment) -> Result<Value, EvalFault> {
    if call.name == "plus" {
        eval_plus_function(call, env)
    } else if call.name == "minus" {
        eval_minus_function(call, env)
    } else if call.name == "rgb" {
        eval_rgb_function(call)
    } else {
        Err(EvalFault::IsNotFunction {
            target_name: call.name,
        })
    }
}

fn eval_expression(exp: Expression, env: &mut Envroiment) -> Result<Value, EvalFault> {
    let value = match exp {
        Expression::Color(color) => Value::Color(color),
        Expression::Identifier(name) => eval_identifer(name, env)?,
        Expression::Call(call) => eval_call(call, env)?,
        Expression::Int(int) => Value::Int(int),
    };

    Ok(value)
}

fn eval_plus_function(mut call: Call, env: &mut Envroiment) -> Result<Value, EvalFault> {
    if call.args.len() != 4 {
        return Err(EvalFault::NumArgments { req: 4, got: call.args.len() })
    };

    let Expression::Int(b) = call.args.pop().expect("bug") else {
        return Err(EvalFault::ArgType);
    };

    let Expression::Int(g) = call.args.pop().expect("bug") else {
        return Err(EvalFault::ArgType);
    };

    let Expression::Int(r) = call.args.pop().expect("bug") else {
        return Err(EvalFault::ArgType);
    };

    let value = eval_expression(call.args.pop().expect("bug"), env)?;

    let Value::Color(color) = value else {
        return Err(EvalFault::ArgType);
    };

    Ok(Value::Color(color.plus(r, g, b)))
}

fn eval_rgb_function(call: Call) -> Result<Value, EvalFault> {
    if call.args.len() != 3 {
        return Err(EvalFault::NumArgments { req: 3, got: call.args.len() })
    };
    let Expression::Int(r) = call.args[0] else {
        return Err(EvalFault::ArgType);
    };
    let Expression::Int(g) = call.args[1] else {
        return Err(EvalFault::ArgType);
    };
    let Expression::Int(b) = call.args[2] else {
        return Err(EvalFault::ArgType);
    };

    Ok(Value::Color(Color::new(r, g, b)))
}

fn eval_minus_function(mut call: Call, env: &mut Envroiment) -> Result<Value, EvalFault> {
    if call.args.len() != 4 {
        return Err(EvalFault::NumArgments { req: 4, got: call.args.len() })
    };

    let Expression::Int(b) = call.args.pop().expect("bug") else {
        return Err(EvalFault::ArgType);
    };

    let Expression::Int(g) = call.args.pop().expect("bug") else {
        return Err(EvalFault::ArgType);
    };

    let Expression::Int(r) = call.args.pop().expect("bug") else {
        return Err(EvalFault::ArgType);
    };

    let value = eval_expression(call.args.pop().expect("bug"), env)?;

    let Value::Color(color) = value else {
        return Err(EvalFault::ArgType);
    };

    Ok(Value::Color(color.minus(r, g, b)))
}

#[cfg(test)]
mod test {
    use std::vec;

    use crate::{color::Color, envroiment::Envroiment, eval::eval_minus_function, parser::{Call, Expression}};

    use super::{eval_plus_function, EvalFault, Value};

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

        let call = Call { name: "minus".to_string(), args };
        
        let result = eval_minus_function(call, &mut env).unwrap_err();
        
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

        let call = Call { name: "minus".to_string(), args };
        
        let result = eval_minus_function(call, &mut env).unwrap();
        
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

        let call = Call { name: "plus".to_string(), args };
        
        let result = eval_plus_function(call, &mut env).unwrap_err();
        
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

        let call = Call { name: "plus".to_string(), args };
        
        let result = eval_plus_function(call, &mut env).unwrap();
        
        assert_eq!(result,assert_val);
    }
}