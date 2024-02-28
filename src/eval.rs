// use std::fs;
use std::{collections::HashMap, fs::read_to_string  };

mod include_file_stack;
use include_file_stack::IncludeFileStack;
use crate::app_path::{self, AbsFilePathError};
use crate::{
    color::Color, fault , parser::{
        ColorExpression, Function, IncludeStatement, LetStatement, MinusFunction, PlusFunction, RgbFunctoin, Statement
    }, run::run
};

use self::include_file_stack::IncludeFileStackFault;

pub struct Envroiment {
    map: HashMap<String, Color>,
    pub faults: Vec<Box<dyn fault::Fault>>,
    pub include_file_stack: IncludeFileStack
}

impl Envroiment {
    pub fn set(&mut self, name: String, color: Color) {
        self.map.insert(name, color);
    }

    pub fn get(&self, name: &String) -> Option<Color> {
        match self.map.get(name) {
            Some(c) => Some(c.clone()),
            None => None
        }
    }

    pub fn new() -> Self {
        Envroiment {
            map: HashMap::new(),
            faults: Vec::new(),
            include_file_stack: IncludeFileStack::new()
        }
    }
    
    pub fn print_vars(&self){
        for var in &self.map {
            println!("{} {:?}", var.0, var.1)
        }
    }
}

pub enum RuntimeFault {
    NotFound { target_name: String },
    NoSuchFile { path: String },
    TodoRename {},
    TodoRename2 {err: AbsFilePathError}
}

impl From<IncludeFileStackFault> for RuntimeFault {
    fn from(_value: IncludeFileStackFault) -> Self {
        RuntimeFault::TodoRename {  }
    }    
}

impl From<AbsFilePathError> for RuntimeFault {
    fn from(value: AbsFilePathError) -> Self {
       RuntimeFault::TodoRename2 { err: value } 
    } 
}

impl fault::Fault for RuntimeFault {
    fn msg(&self) -> String {
        match self {
            RuntimeFault::NotFound { target_name } => format!("RuntimeError: {} is Not Found", target_name) ,
            RuntimeFault::NoSuchFile { path } => format!("RuntimeError: No such file. path:{}", path),
            RuntimeFault::TodoRename {  } => format!("todo"),
            RuntimeFault::TodoRename2 { err } => format!("RuntimeError: {:?}",err)
        }
    }    
}

pub fn eval_include_stmt(include_stmt: IncludeStatement, env: &mut Envroiment) -> Result<(), RuntimeFault> {
    let current_file_path = env.include_file_stack.get_current_file();
    let file_path = app_path::join_or_abs(current_file_path, &include_stmt.path)?;
    let file_string = match read_to_string(file_path.get()) {
        Ok(str) => str,
        Err(_) => {
            return Err(RuntimeFault::NoSuchFile { path: include_stmt.path });    
        }
    };
    
    run(env, file_string.chars().collect(), file_path);

    Ok(()) 
}

pub fn eval(stmt: Statement, env: &mut Envroiment) -> Result<(), RuntimeFault> {
    match stmt {
        Statement::Let(let_stmt) => eval_let_statement(let_stmt, env),
        Statement::Include(include_stmt) => eval_include_stmt(include_stmt, env)
    }
}

fn eval_let_statement(
    let_stmt: LetStatement,
    env: &mut Envroiment,
) -> Result<(), RuntimeFault> {
    let value = eval_expression(let_stmt.right, env)?;
    env.set(let_stmt.left, value);
    Ok(())
}

fn eval_expression(exp: ColorExpression, env: &mut Envroiment) -> Result<Color, RuntimeFault> {
    let color = match exp {
        ColorExpression::Raw(color) => color,
        ColorExpression::Identifier(name) => match env.get(&name) {
            Some(c) => c,
            None => {
                return Err(RuntimeFault::NotFound {
                    target_name: name.clone(),
                });
            }
        },
        ColorExpression::Function(f) => match f {
            Function::Rgb(rgb_f) => eval_rgb_function(rgb_f),
            Function::Plus(plus_f) => eval_plus_function(plus_f, env)?,
            Function::Minus(minus_f) => eval_minus_function(minus_f, env)?,
        },
    };

    Ok(color)
}

fn eval_minus_function(
    minus_f: MinusFunction,
    env: &mut Envroiment,
) -> Result<Color, RuntimeFault> {
    let color = eval_expression(*minus_f.arg_expression, env)?;
    Ok(color.minus(minus_f.arg_r,minus_f.arg_g,minus_f.arg_b))
}

fn eval_plus_function(
    plus_f: PlusFunction,
    env: &mut Envroiment,
) -> Result<Color, RuntimeFault> {
    let color = eval_expression(*plus_f.arg_expression, env)?;
    Ok(color.plus(plus_f.arg_r,plus_f.arg_g,plus_f.arg_b))
}

fn eval_rgb_function(rgbf: RgbFunctoin) -> Color {
    let r= rgbf.arg1_r;
    let g= rgbf.arg2_g;
    let b= rgbf.arg3_b;
    Color::new(r, g, b)
}
