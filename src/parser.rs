use crate::{
    color::{Color, ColorInt},
    fault,
    lexer::Token,
};
use std::collections::VecDeque;

#[derive(Debug)]
pub struct Call {
    pub name: String,
    pub args: Vec<Expression>,
}

#[derive(Debug)]
pub enum Expression {
    Int(ColorInt),
    Color(Color),
    Identifier(String),
    Call(Call),
}

#[derive(Debug)]
pub struct LetStatement {
    pub left: String,
    pub right: Expression,
}

#[derive(Debug)]
pub struct IncludeStatement {
    pub path: String,
}

#[derive(Debug)]
pub enum Statement {
    Let(LetStatement),
    Include(IncludeStatement),
}

#[derive(Debug,PartialEq)]
pub enum ParseFault {
    Syntax,
}
impl fault::Fault for ParseFault {
    fn msg(&self) -> String {
        match self {
            ParseFault::Syntax => {
                format!("ParseError: Syntax")
            }
        }
    }
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

fn peek_token_is(tokens: &mut VecDeque<Token>, assert: Token) -> bool {
    let Some(tkn) = tokens.front() else {
        return false;
    };

    tkn == &assert 
}

fn parse_function(name: String, tokens: &mut VecDeque<Token>) -> Result<Call, ParseFault> {
    check_next_token(tokens, Token::LeftPare)?;
    if peek_token_is(tokens, Token::RightPare) {
        return Ok(Call { name, args: Vec::new() });
    };

    let mut args = Vec::new();

    args.push(parse_expression(tokens)?);

    while !peek_token_is(tokens, Token::RightPare) {
        check_next_token(tokens, Token::Comma)?;
        args.push(parse_expression(tokens)?);    
    }
    
    check_next_token(tokens, Token::RightPare)?;
    
    Ok(Call { name, args })
}


fn parse_expression(tokens: &mut VecDeque<Token>) -> Result<Expression, ParseFault> {
    let Some(front_token) = tokens.pop_front() else {
        return Err(ParseFault::Syntax);
    };

    let exp = match front_token {
        Token::HexColor(color) => Ok(Expression::Color(color)),
        Token::Identifier(name) => {
            if peek_token_is(tokens, Token::LeftPare) {
                Ok(Expression::Call(parse_function(name, tokens)?))
            } else {
                Ok(Expression::Identifier(name))
            }
        },
        Token::Int(int) => Ok(Expression::Int(int)),
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

fn parse_short_let_statement(
    identifier: String,
    tokens: &mut VecDeque<Token>,
) -> Result<LetStatement, ParseFault> {
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

fn parse_include_statement(tokens: &mut VecDeque<Token>) -> Result<IncludeStatement, ParseFault> {
    let Some(path_token) = tokens.pop_front() else {
        return Err(ParseFault::Syntax);
    };

    let path = match path_token {
        Token::Identifier(str) => str,
        _ => {
            return Err(ParseFault::Syntax);
        }
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
        Token::Identifier(identifier) => {
            Statement::Let(parse_short_let_statement(identifier, &mut line_tokens)?)
        }
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
    use super::{parse_tokens_to_statement, LetStatement, ParseFault, Statement};
    use crate::{
        color::Color,
        lexer::lexer,
        parser::{Call, Expression},
    };

    #[test]
    fn test_parse_tokens_to_statement() {
        test_parse_statement(
            "1",
            "let hello = #ffffff",
            Statement::Let(LetStatement {
                left: "hello".to_string(),
                right: Expression::Color(Color::new(255, 255, 255)),
            }),
        );
        test_parse_statement(
            "2",
            "let color1 = #ffffff",
            Statement::Let(LetStatement {
                left: "color1".to_string(),
                right: Expression::Color(Color::new(255, 255, 255)),
            }),
        );
        test_parse_statement(
            "3",
            "let color1 = #abcdef",
            Statement::Let(LetStatement {
                left: "color1".to_string(),
                right: Expression::Color(Color::new(0xab, 0xcd, 0xef)),
            }),
        );
        test_parse_statement(
            "4",
            "include /hello/world",
            Statement::Include(super::IncludeStatement {
                path: "/hello/world".to_string(),
            }),
        );
        test_parse_statement(
            "5",
            "let hello = rgb(10,20,30)",
            Statement::Let(LetStatement {
                left: "hello".to_string(),
                right: Expression::Call(Call {
                    name: "rgb".to_string(),
                    args: vec![
                        Expression::Int(10),
                        Expression::Int(20),
                        Expression::Int(30),
                    ],
                }),
            }),
        );
        test_parse_statement(
            "6",
            "let hello = plus(rgb(10,10,10),10,20,30)",
            Statement::Let(LetStatement {
                left: "hello".to_string(),

                right: Expression::Call(Call {
                    name: "plus".to_string(),
                    args: vec![
                        Expression::Call(Call {
                            name: "rgb".to_string(),
                            args: vec![
                            Expression::Int(10),
                            Expression::Int(10),
                            Expression::Int(10),
                            ]
                        }),
                        Expression::Int(10),
                        Expression::Int(20),
                        Expression::Int(30),
                    ],
                }),
            }),
        );
        test_parse_statement(
            "7 shrort let",
            "hello = plus(rgb(10,10,10),10,20,30)",
            Statement::Let(LetStatement {
                left: "hello".to_string(),

                right: Expression::Call(Call {
                    name: "plus".to_string(),
                    args: vec![
                        Expression::Call(Call {
                            name: "rgb".to_string(),
                            args: vec![
                            Expression::Int(10),
                            Expression::Int(10),
                            Expression::Int(10),
                            ]
                        }),
                        Expression::Int(10),
                        Expression::Int(20),
                        Expression::Int(30),
                    ],
                }),
            }),
        );

    }
    
    #[test]
    fn parse_tokens_to_statement_err(){
        test_parse_statement_err(
            "1", 
            "hello",
            ParseFault::Syntax
        );
        test_parse_statement_err(
            "2", 
            "let hello hello = hello",
            ParseFault::Syntax
        );
        test_parse_statement_err(
            "3", 
            "let hello = hello( 10 ",
            ParseFault::Syntax
        );
        test_parse_statement_err(
            "4", 
            "let hello = hello( 10 ",
            ParseFault::Syntax
        );
        test_parse_statement_err(
            "5", 
            " = cargo ",
            ParseFault::Syntax
        );
        test_parse_statement_err(
            "6", 
            "include 1",
            ParseFault::Syntax
        );
        test_parse_statement_err(
            "7", 
            "include hello hello",
            ParseFault::Syntax
        );
    }

    
    fn test_parse_statement_err(
        test_name: &str,
        stmt_str: &str,
        assert: ParseFault
    ){
        println!("test {}",test_name);
        let mut chars = stmt_str.chars().collect();
        let tokens = lexer(&mut chars).unwrap();
        let parsed = parse_tokens_to_statement(tokens).unwrap_err();
        assert_eq!(parsed,assert);
    }

    fn assert_function(test_name: &str, a: Call, b: Call) {
        assert_eq!(a.name, b.name, "{}", test_name);
        assert_eq!(a.args.len(), b.args.len(), "{}", test_name);
        for (a_i, b_i) in a.args.into_iter().zip(b.args.into_iter()) {
            assert_expression(test_name, a_i, b_i)
        }
    }

    fn assert_expression(test_name: &str, a: Expression, b: Expression) {
        match a {
            Expression::Color(a_val) => match b {
                Expression::Color(b_val) => {
                    assert_eq!(a_val, b_val, "{}", test_name)
                }
                _ => panic!("{}", test_name),
            },
            Expression::Call(a_val) => match b {
                Expression::Call(b_val) => assert_function(test_name, a_val, b_val),
                _ => panic!("{}", test_name),
            },
            Expression::Int(a_val) => match b {
                Expression::Int(b_val) => {
                    assert_eq!(a_val, b_val, "{}", test_name);
                }
                _ => panic!("{}", test_name),
            },
            Expression::Identifier(a_val) => match b {
                Expression::Identifier(b_val) => {
                    assert_eq!(a_val, b_val, "{}", test_name)
                }
                _ => panic!("{}", test_name),
            },
        }
    }

    fn assert_let_stmt(test_name: &str, a: LetStatement, b: LetStatement) {
        assert_eq!(a.left, b.left, "{}", test_name);
        assert_expression(test_name, a.right, b.right);
    }

    fn test_parse_statement(test_name: &str, stmt_str: &str, assert_stmt: Statement) {
        println!("test {}",test_name);
        let mut chars = stmt_str.chars().collect();
        let tokens = lexer(&mut chars).unwrap();
        let parsed = parse_tokens_to_statement(tokens).unwrap();

        match parsed {
            Statement::Include(include_stmt) => match assert_stmt {
                Statement::Include(a_include_stmt) => {
                    assert_eq!(include_stmt.path, a_include_stmt.path, "{}", test_name)
                }
                _ => panic!("{}", test_name),
            },
            Statement::Let(let_stmt) => match assert_stmt {
                Statement::Let(a_let_stmt) => assert_let_stmt(test_name, let_stmt, a_let_stmt),
                _ => panic!("{}", test_name),
            },
        }
    }
}
