use std::{collections::HashMap, fs::read_to_string };

use crate::{
    color::Color,
    parser::{
        ColorExpression, Function, IncludeStatement, LetStatement, MinusFunction, PlusFunction, RgbFunctoin, Statement
    }, run::run,
};

// pub trait Env {
//     fn get(&self, name: &String) -> Option<Color>;
//     fn set(&mut self, name: String, color: Color);
//     fn include(&mut self, child: &dyn Env);
//     // パフォーマンス悪い気がする
//     fn vars(&self) -> Vec<(String,Color)>;
// }

// pub struct RootEnvroiment { }
// impl RootEnvroiment {
//     pub fn new() -> Self {
//         RootEnvroiment { }
//     }
// }

// impl Env for RootEnvroiment {
//     fn get(&self, _name: &String) -> Option<Color> {
//         None
//     }

//     fn set(&mut self, _name: String, _color: Color) { }
    
//     fn include(&mut self, _child: &dyn Env) { }
    
//     fn vars(&self) -> Vec<(String,Color)> {
//         vec![]
//     }
// }

pub struct Envroiment {
    map: HashMap<String, Color>,
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
    pub fn vars(&self) -> Vec<(String,Color)> {
        self.map.clone().into_iter().collect()
    }
}

pub enum RuntimeFault {
    NotFound { target_name: String },
    NoSuchFile { path: String }
}

impl RuntimeFault {
    pub fn print_msg(&self) {
        match self {
            RuntimeFault::NotFound { target_name } => {
                println!("RuntimeError: {} is Not Found", target_name)
            },
            RuntimeFault::NoSuchFile { path } => {
                println!("RuntimeError: No such file. path:{}", path)
            }
        }
    }
}

impl Envroiment {
    pub fn new() -> Self {
        Envroiment {
            map: HashMap::new(),
        }
    }
    
    pub fn print_vars(&self){
        for var in self.vars() {
            println!("{} {:?}", var.0, var.1)
        }
    }
}

pub fn eval_include_stmt(include_stmt: IncludeStatement, env: &mut Envroiment) -> Result<(), RuntimeFault> {
    let file_string = match read_to_string(include_stmt.path.clone()) {
        Ok(str) => str,
        Err(_) => {
            return Err(RuntimeFault::NoSuchFile { path: include_stmt.path });    
        }
    };
    
    let file_chars = file_string.chars().collect();
    run(env, file_chars);

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
    let mut color = eval_expression(*minus_f.arg_expression, env)?;
    color.r -= minus_f.arg_r;
    color.g -= minus_f.arg_g;
    color.b -= minus_f.arg_b;
    Ok(color)
}

fn eval_plus_function(
    plus_f: PlusFunction,
    env: &mut Envroiment,
) -> Result<Color, RuntimeFault> {
    let mut color = eval_expression(*plus_f.arg_expression, env)?;
    color.r += plus_f.arg_r;
    color.g += plus_f.arg_g;
    color.b += plus_f.arg_b;
    Ok(color)
}

fn eval_rgb_function(rgbf: RgbFunctoin) -> Color {
    Color {
        r: rgbf.arg1_r,
        g: rgbf.arg2_g,
        b: rgbf.arg3_b,
    }
}
