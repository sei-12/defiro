// use std::fs;

mod function;

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

use self::function::{eval_minus_function, eval_plus_function, eval_rgb_function};

#[derive(Debug, PartialEq)]
pub enum EvalFault {
    NotFound { target_name: String },
    NoSuchFile { path: String },
    TodoRename2 { err: AbsFilePathError },
    IsNotFunction { target_name: String },
    NumArgments { req: usize, got: usize },
    ArgType, // { req: String, got: String },
}

#[derive(Debug, PartialEq)]
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
            }
            EvalFault::NumArgments { req, got } => {
                format!(
                    "EvalError: Wrong number of arguments. req={} got={}",
                    req, got
                )
            }
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
        eval_plus_function(call.args, env)
    } else if call.name == "minus" {
        eval_minus_function(call.args, env)
    } else if call.name == "rgb" {
        eval_rgb_function(call.args)
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
