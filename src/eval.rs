use std::collections::HashMap;

use crate::{
    color::Color,
    parser::{ColorExpression, Function, LetStatement, MinusFunction, PlusFunction, RgbFunctoin, Statement},
};

pub struct Envroiment {
    map: HashMap<String, Color>,
}

pub enum RuntimeFault {
    NotFound { target_name: String },
}

impl RuntimeFault {
    pub fn print_msg(&self) {
        match self {
            RuntimeFault::NotFound { target_name } => {
                println!("RuntimeError: {} is Not Found", target_name)
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

    pub fn print(&self) {
        for (var_id, var_value) in &self.map {
            println!("{} = {:?}", var_id, var_value)
        }
    }
}

pub fn eval(stmt: Statement, env: &mut Envroiment) -> Result<(), RuntimeFault> {
    match stmt {
        Statement::Let(let_stmt) => eval_let_statement(let_stmt, env),
    }
}

fn eval_let_statement(let_stmt: LetStatement, env: &mut Envroiment) -> Result<(), RuntimeFault> {
    let value = eval_expression(let_stmt.right, env)?;
    env.map.insert(let_stmt.left, value);
    Ok(())
}

fn eval_expression(exp: ColorExpression, env: &mut Envroiment) -> Result<Color, RuntimeFault> {
    let color = match exp {
        ColorExpression::Raw(color) => color,
        ColorExpression::Identifier(name) => match env.map.get(&name) {
            Some(c) => c.clone(),
            None => {
                return Err(RuntimeFault::NotFound {
                    target_name: name.clone(),
                });
            }
        },
        ColorExpression::Function(f) => match f {
            Function::Rgb(rgb_f) => eval_rgb_function(rgb_f),
            Function::Plus(plus_f) => eval_plus_function(plus_f,env)?,
            Function::Minus(minus_f) => eval_minus_function(minus_f, env)?,
        },
    };

    Ok(color)
}

fn eval_minus_function(minus_f: MinusFunction, env: &mut Envroiment) -> Result<Color, RuntimeFault> {
    let mut color = eval_expression(*minus_f.arg_expression, env)?;
    color.r -= minus_f.arg_r;
    color.g -= minus_f.arg_g;
    color.b -= minus_f.arg_b;
    Ok(color)
}

fn eval_plus_function(plus_f: PlusFunction, env: &mut Envroiment) -> Result<Color, RuntimeFault> {
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
