use crate::{
    color::{self, Color}, fault, lexer::Token
};
use std::collections::VecDeque;

pub struct RgbFunctoin {
    pub arg1_r: color::ColorInt,
    pub arg2_g: color::ColorInt,
    pub arg3_b: color::ColorInt,
}

pub struct PlusFunction {
    pub arg_expression: Box<ColorExpression>,
    pub arg_r: color::ColorInt,
    pub arg_g: color::ColorInt,
    pub arg_b: color::ColorInt,
}

pub struct MinusFunction {
    pub arg_expression: Box<ColorExpression>,
    pub arg_r: color::ColorInt,
    pub arg_g: color::ColorInt,
    pub arg_b: color::ColorInt,
}

pub enum Function {
    Rgb(RgbFunctoin),
    Plus(PlusFunction),
    Minus(MinusFunction),
}

pub enum ColorExpression {
    Identifier(String),
    Raw(Color),
    Function(Function),
}

pub struct LetStatement {
    pub left: String,
    pub right: ColorExpression,
}

pub struct IncludeStatement {
    pub path: String    
}

pub enum Statement {
    Let(LetStatement),
    Include(IncludeStatement),
}

#[derive(Debug)]
pub enum ParseFault {
    Syntax,
    IsFunction { target_name: String },
}
impl fault::Fault for ParseFault {
    fn msg(&self) -> String {
        match self {
            ParseFault::Syntax => {
                format!("ParseError: Syntax")
            }
            ParseFault::IsFunction { target_name } => {
                format!("ParseError: {} is Function", target_name)
            }
        }
    }
}
fn is_function_name(name: &String) -> bool {
    if name == "rgb" {
        return true;
    };
    if name == "plus" {
        return true;
    };
    if name == "minus" {
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

fn parse_rgb_function(tokens: &mut VecDeque<Token>) -> Result<ColorExpression, ParseFault> {
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

    Ok(ColorExpression::Function(Function::Rgb(RgbFunctoin {
        arg1_r: r,
        arg2_g: g,
        arg3_b: b,
    })))
}

fn parse_minus_function(tokens: &mut VecDeque<Token>) -> Result<ColorExpression, ParseFault> {
    let (exp, r, g, b) = parse_minus_and_plus(tokens)?;
    Ok(ColorExpression::Function(Function::Minus(MinusFunction {
        arg_expression: Box::new(exp),
        arg_r: r,
        arg_g: g,
        arg_b: b,
    })))
}

fn parse_minus_and_plus(
    tokens: &mut VecDeque<Token>,
) -> Result<(ColorExpression, color::ColorInt, color::ColorInt, color::ColorInt), ParseFault> {
    check_next_token(tokens, Token::LeftPare)?;
    let exp = parse_expression(tokens)?;

    check_next_token(tokens, Token::Comma)?;
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

    Ok((exp, r, g, b))
}

fn parse_plus_fucntion(tokens: &mut VecDeque<Token>) -> Result<ColorExpression, ParseFault> {
    let (exp, r, g, b) = parse_minus_and_plus(tokens)?;
    Ok(ColorExpression::Function(Function::Plus(PlusFunction {
        arg_expression: Box::new(exp),
        arg_r: r,
        arg_g: g,
        arg_b: b,
    })))
}

fn parse_function(
    name: String,
    tokens: &mut VecDeque<Token>,
) -> Result<ColorExpression, ParseFault> {
    if name == "rgb" {
        return parse_rgb_function(tokens);
    };
    if name == "plus" {
        return parse_plus_fucntion(tokens);
    };
    if name == "minus" {
        return parse_minus_function(tokens);
    }

    panic!("bug")
}

fn parse_expression(tokens: &mut VecDeque<Token>) -> Result<ColorExpression, ParseFault> {
    let Some(front_token) = tokens.pop_front() else {
        return Err(ParseFault::Syntax);
    };

    let exp = match front_token {
        Token::HexColor(color) => Ok(ColorExpression::Raw(color)),
        Token::Identifier(name) => {
            if is_function_name(&name) {
                parse_function(name, tokens)
            } else {
                Ok(ColorExpression::Identifier(name))
            }
        }
        _ => Err(ParseFault::Syntax),
    }?;

    Ok(exp)
}

fn parse_let_statement(tokens: &mut VecDeque<Token>) -> Result<LetStatement, ParseFault> {
    let Some(iden_token) = tokens.pop_front() else {
        return Err(ParseFault::Syntax);
    };

    let identifier = match iden_token {
        Token::Identifier(id) => id,
        _ => {
            return Err(ParseFault::Syntax);
        }
    };

    if is_function_name(&identifier) {
        return Err(ParseFault::IsFunction {
            target_name: identifier,
        });
    };

    let Some(assgin_token) = tokens.pop_front() else {
        return Err(ParseFault::Syntax);
    };

    if assgin_token != Token::Assign {
        return Err(ParseFault::Syntax);
    };

    let exp = parse_expression(tokens)?;

    Ok(LetStatement {
        left: identifier,
        right: exp,
    })
}

fn parse_short_let_statement(identifier: String, tokens: &mut VecDeque<Token>)-> Result<LetStatement, ParseFault> {
    if is_function_name(&identifier) {
        return Err(ParseFault::IsFunction {
            target_name: identifier,
        });
    };

    let Some(assgin_token) = tokens.pop_front() else {
        return Err(ParseFault::Syntax);
    };

    if assgin_token != Token::Assign {
        return Err(ParseFault::Syntax);
    };

    let exp = parse_expression(tokens)?;

    Ok(LetStatement {
        left: identifier,
        right: exp,
    })
}

fn parse_include_statement(tokens: &mut VecDeque<Token>) -> Result<IncludeStatement,ParseFault>{
    let Some(path_token) = tokens.pop_front() else {
        return Err(ParseFault::Syntax);
    };
    
    let path = match path_token {
        Token::Identifier(str) => { str },
        _ => { return Err(ParseFault::Syntax);}
    };
    
    Ok(IncludeStatement { path })
}

pub fn parse_tokens_to_statement(
    mut line_tokens: VecDeque<Token>,
) -> Result<Statement, ParseFault> {
    let Some(front_token) = line_tokens.pop_front() else {
        return Err(ParseFault::Syntax);
    };

    let stmt = match front_token {
        Token::Let => Statement::Let(parse_let_statement(&mut line_tokens)?),
        Token::Identifier(identifier) => Statement::Let(parse_short_let_statement(identifier, &mut line_tokens)?),
        Token::Include => Statement::Include(parse_include_statement(&mut line_tokens)?),
        _ => {
            return Err(ParseFault::Syntax);
        }
    };

    if line_tokens.len() != 0 {
        return Err(ParseFault::Syntax);
    };

    Ok(stmt)
}

#[cfg(test)]
mod test {
    use super::{parse_tokens_to_statement, Statement};
    use crate::{
        color::Color,
        lexer::lexer,
        parser::{ColorExpression, Function},
    };

    #[test]
    fn _parse_token_to_stmt() {
        let mut test = "let a = #0a0a0a".chars().collect();
        let tokens = lexer(&mut test).unwrap();
        let stmt = parse_tokens_to_statement(tokens).unwrap();
        match stmt {
            Statement::Let(le) => {
                assert_eq!(le.left, "a".to_string());
                match le.right {
                    ColorExpression::Raw(color) => {
                        assert_eq!(
                            color,
                            Color ::new( 10,  10,  10 )
                        )
                    }
                    _ => panic!(),
                }
            },
            _ => panic!()
        }

        let mut test = "let a = bbb".chars().collect();
        let tokens = lexer(&mut test).unwrap();
        let stmt = parse_tokens_to_statement(tokens).unwrap();
        match stmt {
            Statement::Let(le) => {
                assert_eq!(le.left, "a".to_string());
                match le.right {
                    ColorExpression::Identifier(name) => {
                        assert_eq!(name, "bbb".to_string())
                    }
                    _ => panic!(),
                }
            },
            _ => panic!()
        }

        let mut test = "let a = rgb(1,2,3)".chars().collect();
        let tokens = lexer(&mut test).unwrap();
        let stmt = parse_tokens_to_statement(tokens).unwrap();
        match stmt {
            Statement::Let(le) => {
                assert_eq!(le.left, "a".to_string());
                match le.right {
                    ColorExpression::Function(f) => match f {
                        Function::Rgb(rgbf) => {
                            assert_eq!(rgbf.arg1_r, 1);
                            assert_eq!(rgbf.arg2_g, 2);
                            assert_eq!(rgbf.arg3_b, 3);
                        }
                        _ => panic!(),
                    },
                    _ => panic!(),
                }
            },
            _ => panic!()
        }
        
        let mut test = "a = #0a0a0a".chars().collect();
        let tokens = lexer(&mut test).unwrap();
        let stmt = parse_tokens_to_statement(tokens).unwrap();
        match stmt {
            Statement::Let(le) => {
                assert_eq!(le.left, "a".to_string());
                match le.right {
                    ColorExpression::Raw(color) => {
                        assert_eq!(
                            color,
                            Color ::new( 10,  10,  10 )
                        )
                    }
                    _ => panic!(),
                }
            },
            _ => panic!()
        }

        let mut test = "a = bbb".chars().collect();
        let tokens = lexer(&mut test).unwrap();
        let stmt = parse_tokens_to_statement(tokens).unwrap();
        match stmt {
            Statement::Let(le) => {
                assert_eq!(le.left, "a".to_string());
                match le.right {
                    ColorExpression::Identifier(name) => {
                        assert_eq!(name, "bbb".to_string())
                    }
                    _ => panic!(),
                }
            },
            _ => panic!()
        }
        
        
        let mut test = "a = rgb(1,2,3)".chars().collect();
        let tokens = lexer(&mut test).unwrap();
        let stmt = parse_tokens_to_statement(tokens).unwrap();
        match stmt {
            Statement::Let(le) => {
                assert_eq!(le.left, "a".to_string());
                match le.right {
                    ColorExpression::Function(f) => match f {
                        Function::Rgb(rgbf) => {
                            assert_eq!(rgbf.arg1_r, 1);
                            assert_eq!(rgbf.arg2_g, 2);
                            assert_eq!(rgbf.arg3_b, 3);
                        }
                        _ => panic!(),
                    },
                    _ => panic!(),
                }
            },
            _ => panic!()
        }
        let mut test = "a = rgb(1,2,3)()".chars().collect();
        let tokens = lexer(&mut test).unwrap();
        let stmt = parse_tokens_to_statement(tokens);
        assert!(stmt.is_err());

        let mut test = "a = rgb(hello,2,3)()".chars().collect();
        let tokens = lexer(&mut test).unwrap();
        let stmt = parse_tokens_to_statement(tokens);
        assert!(stmt.is_err());

        let mut test = "a rgb(1,2,3)()".chars().collect();
        let tokens = lexer(&mut test).unwrap();
        let stmt = parse_tokens_to_statement(tokens);
        assert!(stmt.is_err());

        let mut test = "a = a g".chars().collect();
        let tokens = lexer(&mut test).unwrap();
        let stmt = parse_tokens_to_statement(tokens);
        assert!(stmt.is_err());

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
