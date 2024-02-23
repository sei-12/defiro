use std::collections::HashMap;

use crate::{color::Color, parser::{Expression, LetStatement, Statement}};

pub struct Envroiment {
    map: HashMap<String,Color> 
}

impl Envroiment {
    pub fn new() -> Self {
        Envroiment { map: HashMap::new() }
    }
    
    pub fn print(&self) {
        for (var_id,var_value) in &self.map {
            println!("{} = {:?}",var_id,var_value)
        }
    }
}

pub fn eval(stmt: Statement, env: &mut Envroiment ){
    match stmt {
        Statement::Let( let_stmt ) => eval_let_statement(let_stmt, env)
    } 
}

fn eval_let_statement(let_stmt: LetStatement, env: &mut Envroiment) {
    let value = match let_stmt.right {
        Expression::Raw(color) => color
    };
    env.map.insert(let_stmt.left, value);
}