use std::collections::VecDeque;
use crate::{color::Color, lexer::Token};



pub enum Expression {
    Raw(Color)    
}

pub struct LetStatement {
    pub left: String,
    pub right: Expression    
}

pub enum Statement {
    Let(LetStatement)
}

#[derive(Debug)]
pub enum ParseFault {
    TODO,    
}

fn parse_expression(tokens: &mut VecDeque<Token>) -> Result<Expression,ParseFault>{ 
    let Some(front_token) = tokens.pop_front() else {
        return Err(ParseFault::TODO);        
    }; 
    
    match front_token {
        Token::HexColor(color) => Ok(Expression::Raw(color)),
        _ => Err(ParseFault::TODO)
    }
}

fn parse_let_statement(mut tokens: VecDeque<Token>) -> Result<LetStatement,ParseFault> {
    let Some(iden_token) = tokens.pop_front() else {
        return Err(ParseFault::TODO);        
    }; 

    let identifier = match iden_token {
        Token::Identifier(id) => id,
        _ => { return Err(ParseFault::TODO);}
    };

    let Some(assgin_token) = tokens.pop_front() else {
        return Err(ParseFault::TODO);        
    }; 

    if assgin_token != Token::Assign {
        return Err(ParseFault::TODO);        
    };
    
    let exp = parse_expression(&mut tokens)?;
    
    
    Ok(LetStatement { left: identifier, right: exp })
}

pub fn parse_tokens_to_statement(mut line_tokens: VecDeque<Token>) -> Result<Statement,ParseFault> {
    let Some(front_token) = line_tokens.pop_front() else {
        return Err(ParseFault::TODO);
    }; 
    
    if front_token == Token::Let {
        return Ok(Statement::Let(parse_let_statement(line_tokens)?));
    };
    
    Err(ParseFault::TODO)
}

#[cfg(test)]
mod test {
    use crate::{color::Color, lexer::lexer, parser::Expression};
    use super::{parse_tokens_to_statement, Statement};
    
    #[test]
    fn _parse_token_to_stmt(){
        let mut test = "let a = #0a0a0a".chars().collect();
        let tokens = lexer(&mut test).unwrap();
        let stmt = parse_tokens_to_statement(tokens).unwrap();
        match stmt {
            Statement::Let(le) => {
                assert_eq!(le.left,"a".to_string());
                match le.right {
                    Expression::Raw(color) => {
                        assert_eq!(color,Color { r: 10, g: 10, b: 10 })
                    }
                }
            }
        }

    }

        
}

