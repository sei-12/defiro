use std::collections::HashMap;

use crate::{color::Color, parser::{Expression, LetStatement, Statement}};

pub struct Envroiment {
    map: HashMap<String,Color> 
}

pub enum RuntimeFault {
    NotFound {
        target_name: String
    },
}

impl RuntimeFault {
    pub fn print_msg(&self){
        match self {
            RuntimeFault::NotFound{target_name} => {
                println!("RuntimeError: {} is Not Found",target_name)
            }
        }
    } 
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

pub fn eval(stmt: Statement, env: &mut Envroiment ) -> Result<(), RuntimeFault> {
    match stmt {
        Statement::Let( let_stmt ) => eval_let_statement(let_stmt, env)
    } 
}

fn eval_let_statement(let_stmt: LetStatement, env: &mut Envroiment) -> Result<(),RuntimeFault> {
    let value = match let_stmt.right {
        Expression::Raw(color) => color,
        Expression::Identifier(name) => {
            match  env.map.get( &name ) {
                Some(c) => c.clone() ,
                None => { return  Err(RuntimeFault::NotFound { target_name: name.clone() });}
            }
        },
    };
    env.map.insert(let_stmt.left, value);
    Ok(())
}