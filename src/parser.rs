use crate::{
    color::Color,
    lexer::{Token, TokenInt},
};
use std::collections::VecDeque;

pub struct RgbFunctoin {
    pub arg1_r: TokenInt,
    pub arg2_g: TokenInt,
    pub arg3_b: TokenInt,
}

pub enum Function {
    Rgb(RgbFunctoin),
}

pub enum Expression {
    Identifier(String),
    Raw(Color),
    Function(Function),
}

pub struct LetStatement {
    pub left: String,
    pub right: Expression,
}

pub enum Statement {
    Let(LetStatement),
}

#[derive(Debug)]
pub enum ParseFault {
    TODO,
    Syntax,
}
impl ParseFault {
    pub fn msg(&self) -> String{
        match self {
            ParseFault::Syntax => {
                format!("ParseError: Syntax")
            },
            ParseFault::TODO => {
                format!("ParseError: TODO")
            }
        }
    } 
}
fn is_function_name(name: &String) -> bool {
    if name == "rgb" {
        return true;
    };

    false
}

fn check_next_token(tokens: &mut VecDeque<Token>, assert: Token) -> Result<(), ParseFault> {
    let Some(tkn) = tokens.pop_front() else {
        return Err(ParseFault::Syntax);
    };
    if tkn != assert {
        return Err(ParseFault::Syntax);
    };
    Ok(())
}

fn parse_rgb_function(tokens: &mut VecDeque<Token>) -> Result<Expression, ParseFault> {
    check_next_token(tokens, Token::LeftPare)?;
    let Some(Token::Int(r)) = tokens.pop_front() else {
        return Err(ParseFault::Syntax);
    };
    check_next_token(tokens, Token::Comma)?;
    let Some(Token::Int(g)) = tokens.pop_front() else {
        return Err(ParseFault::Syntax);
    };
    check_next_token(tokens, Token::Comma)?;
    let Some(Token::Int(b)) = tokens.pop_front() else {
        return Err(ParseFault::Syntax);
    };
    check_next_token(tokens, Token::RightPare)?;

    Ok(Expression::Function(Function::Rgb(RgbFunctoin {
        arg1_r: r,
        arg2_g: g,
        arg3_b: b,
    })))
}

fn parse_function(name: String, tokens: &mut VecDeque<Token>) -> Result<Expression, ParseFault> {
    if name == "rgb" {
        return parse_rgb_function(tokens);
    };

    panic!("bug")
}

fn parse_expression(tokens: &mut VecDeque<Token>) -> Result<Expression, ParseFault> {
    let Some(front_token) = tokens.pop_front() else {
        return Err(ParseFault::TODO);
    };

    let exp = match front_token {
        Token::HexColor(color) => Ok(Expression::Raw(color)),
        Token::Identifier(name) => {
            if is_function_name(&name) {
                parse_function(name, tokens)
            } else {
                Ok(Expression::Identifier(name))
            }
        }
        _ => Err(ParseFault::TODO),
    }?;

    if tokens.len() != 0 {
        return Err(ParseFault::Syntax);
    };
    
    Ok(exp)
}

fn parse_let_statement(mut tokens: VecDeque<Token>) -> Result<LetStatement, ParseFault> {
    let Some(iden_token) = tokens.pop_front() else {
        return Err(ParseFault::TODO);
    };

    let identifier = match iden_token {
        Token::Identifier(id) => id,
        _ => {
            return Err(ParseFault::TODO);
        }
    };

    let Some(assgin_token) = tokens.pop_front() else {
        return Err(ParseFault::TODO);
    };

    if assgin_token != Token::Assign {
        return Err(ParseFault::TODO);
    };

    let exp = parse_expression(&mut tokens)?;

    Ok(LetStatement {
        left: identifier,
        right: exp,
    })
}

pub fn parse_tokens_to_statement(
    mut line_tokens: VecDeque<Token>,
) -> Result<Statement, ParseFault> {
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
    use super::{parse_tokens_to_statement, Statement};
    use crate::{color::Color, lexer::lexer, parser::{Expression, Function}};

    #[test]
    fn _parse_token_to_stmt() {
        let mut test = "let a = #0a0a0a".chars().collect();
        let tokens = lexer(&mut test).unwrap();
        let stmt = parse_tokens_to_statement(tokens).unwrap();
        match stmt {
            Statement::Let(le) => {
                assert_eq!(le.left, "a".to_string());
                match le.right {
                    Expression::Raw(color) => {
                        assert_eq!(
                            color,
                            Color {
                                r: 10,
                                g: 10,
                                b: 10
                            }
                        )
                    }
                    _ => panic!(),
                }
            }
        }

        let mut test = "let a = bbb".chars().collect();
        let tokens = lexer(&mut test).unwrap();
        let stmt = parse_tokens_to_statement(tokens).unwrap();
        match stmt {
            Statement::Let(le) => {
                assert_eq!(le.left, "a".to_string());
                match le.right {
                    Expression::Identifier(name) => {
                        assert_eq!(name, "bbb".to_string())
                    }
                    _ => panic!(),
                }
            }
        }
        
        let mut test = "let a = rgb(1,2,3)".chars().collect();
        let tokens = lexer(&mut test).unwrap();
        let stmt = parse_tokens_to_statement(tokens).unwrap();
        match stmt {
            Statement::Let(le) => {
                assert_eq!(le.left, "a".to_string());
                match le.right {
                    Expression::Function(f) => {
                        match f {
                            Function::Rgb(rgbf) => {
                                assert_eq!(rgbf.arg1_r,1);
                                assert_eq!(rgbf.arg2_g,2);
                                assert_eq!(rgbf.arg3_b,3);
                            },
                            // _ => panic!()
                        }
                    }
                    _ => panic!(),
                }
            }
        }

        let mut test = "let a = rgb(1,2,3)()".chars().collect();
        let tokens = lexer(&mut test).unwrap();
        let stmt = parse_tokens_to_statement(tokens);
        assert!(stmt.is_err());

        let mut test = "let a = rgb(hello,2,3)()".chars().collect();
        let tokens = lexer(&mut test).unwrap();
        let stmt = parse_tokens_to_statement(tokens);
        assert!(stmt.is_err());

        let mut test = "let a rgb(1,2,3)()".chars().collect();
        let tokens = lexer(&mut test).unwrap();
        let stmt = parse_tokens_to_statement(tokens);
        assert!(stmt.is_err()); 

        let mut test = "let a = a g".chars().collect();
        let tokens = lexer(&mut test).unwrap();
        let stmt = parse_tokens_to_statement(tokens);
        assert!(stmt.is_err()); 
    }
}
