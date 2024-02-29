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

pub enum RuntimeFault {
    NotFound { target_name: String },
    NoSuchFile { path: String },
    TodoRename2 { err: AbsFilePathError },
    IsNotFunction { target_name: String }
}

enum Value {
    Color(Color),
    Int(ColorInt),
}

impl From<AbsFilePathError> for RuntimeFault {
    fn from(value: AbsFilePathError) -> Self {
        RuntimeFault::TodoRename2 { err: value }
    }
}

impl fault::Fault for RuntimeFault {
    fn msg(&self) -> String {
        match self {
            RuntimeFault::NotFound { target_name } => {
                format!("RuntimeError: {} is Not Found", target_name)
            }
            RuntimeFault::NoSuchFile { path } => {
                format!("RuntimeError: No such file. path:{}", path)
            }
            RuntimeFault::TodoRename2 { err } => format!("RuntimeError: {:?}", err),
            RuntimeFault::IsNotFunction { target_name } => format!("RuntimeError: {} is not function",target_name)
        }
    }
}

pub fn eval_include_stmt(
    include_stmt: IncludeStatement,
    env: &mut Envroiment,
) -> Result<(), RuntimeFault> {
    let current_file_path = env.include_file_stack.get_current_file();
    let file_path = app_path::join_or_abs(current_file_path, &include_stmt.path)?;
    let file_string = match read_to_string(file_path.get()) {
        Ok(str) => str,
        Err(_) => {
            return Err(RuntimeFault::NoSuchFile {
                path: include_stmt.path,
            });
        }
    };

    run(env, file_string.chars().collect(), file_path);

    Ok(())
}

pub fn eval(stmt: Statement, env: &mut Envroiment) -> Result<(), RuntimeFault> {
    match stmt {
        Statement::Let(let_stmt) => eval_let_statement(let_stmt, env),
        Statement::Include(include_stmt) => eval_include_stmt(include_stmt, env),
    }
}

fn eval_let_statement(let_stmt: LetStatement, env: &mut Envroiment) -> Result<(), RuntimeFault> {
    let value = eval_expression(let_stmt.right, env)?;
    let Value::Color(color) = value else { todo!() };

    env.set(let_stmt.left, color);
    Ok(())
}

fn eval_identifer(name: String, env: &mut Envroiment) -> Result<Value, RuntimeFault> {
    match env.get(&name) {
        Some(color) => Ok(Value::Color(color)),
        None => Err(RuntimeFault::NotFound { target_name: name }),
    }
}

fn eval_call(call: Call, env: &mut Envroiment) -> Result<Value, RuntimeFault>{

    if call.name == "plus" {
        eval_plus_function(call, env)
    }else if call.name == "minus" {
        eval_minus_function(call, env)
    }else if call.name == "rgb" {
        eval_rgb_function(call)
    }else{
        Err(RuntimeFault::IsNotFunction { target_name: call.name })
    }
}

fn eval_expression(exp: Expression, env: &mut Envroiment) -> Result<Value, RuntimeFault> {
    let value = match exp {
        Expression::Color(color) => Value::Color(color),
        Expression::Identifier(name) => eval_identifer(name, env)?,
        Expression::Call(call) => eval_call(call, env)?,
        Expression::Int(int) => Value::Int(int)
    };

    Ok(value)
}

fn eval_plus_function(
    mut call: Call,
    env: &mut Envroiment,
) -> Result<Value, RuntimeFault> {
    if call.args.len() != 4 {
        // runtime error
        todo!()
    };

    let Expression::Int(b) = call.args.pop().expect("bug") else {
        todo!()
    };

    let Expression::Int(g) = call.args.pop().expect("bug") else {
        todo!()
    };

    let Expression::Int(r) = call.args.pop().expect("bug") else {
        todo!()
    };

    let value = eval_expression( call.args.pop().expect("bug"), env)?;

    let Value::Color(color) = value else {
        todo!()
    };

    Ok(Value::Color(color.plus(r,g,b)))
}

fn eval_rgb_function(
    call: Call,
) -> Result<Value, RuntimeFault> {
    if call.args.len() != 3 {
        // runtime error
        todo!()
    };
    let Expression::Int(r) = call.args[0] else {
        todo!()
    };
    let Expression::Int(g) = call.args[1] else {
        todo!()
    };
    let Expression::Int(b) = call.args[2] else {
        todo!()
    };

    Ok(Value::Color(Color::new(r, g, b)))
}

fn eval_minus_function(
    mut call: Call,
    env: &mut Envroiment,
) -> Result<Value, RuntimeFault> {
    if call.args.len() != 4 {
        // runtime error
        todo!()
    };

    let Expression::Int(b) = call.args.pop().expect("bug") else {
        todo!()
    };

    let Expression::Int(g) = call.args.pop().expect("bug") else {
        todo!()
    };

    let Expression::Int(r) = call.args.pop().expect("bug") else {
        todo!()
    };

    let value = eval_expression( call.args.pop().expect("bug"), env)?;

    let Value::Color(color) = value else {
        todo!()
    };

    Ok(Value::Color(color.minus(r,g,b)))
}