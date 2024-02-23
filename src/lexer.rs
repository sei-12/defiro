use std::collections::VecDeque;
use crate::{color::Color, utils::peek_take_while};


fn pops_front<T>(iter: &mut VecDeque<T>, length: usize) -> Vec<T> {
    let mut ret_vec = Vec::with_capacity(length);
    for _ in 0..length {
        let Some(item) = iter.pop_front() else {
            break;
        };
        ret_vec.push(item);
    }
    ret_vec
}

fn is_token_char(ch: char) -> bool {
    if ch == '#' {
        return true;
    };
    if ch == '=' {
        return true;
    };
    false
}

fn is_skip_char(ch: char) -> bool {
    if ch == ' ' {
        return true;
    };
    if ch == '\t' {
        return true;
    };
    if ch == '\n' {
        return true;
    };

    false
}

#[derive(Debug)]
pub enum LexFault {
    Value,
}

pub fn lexer(mut chars: &mut VecDeque<char>) -> Result<VecDeque<Token>, LexFault> {
    let mut tokens = VecDeque::new();

    loop {
        let Some(ch) = chars.pop_front() else {
            break;
        };

        if is_skip_char(ch) {
            continue;
        }

        if ch == '#' {
            let hex = pops_front(&mut chars, 6);
            if hex.len() != 6 {
                return Err(LexFault::Value);
            }
            let mut hex_iter = hex.into_iter();
            let Some(color) = Color::from_hex_chars(&mut hex_iter) else {
                return Err(LexFault::Value);
            };
            tokens.push_back(Token::HexColor(color));
            continue;
        }

        if ch == '=' {
            tokens.push_back(Token::Assign);
            continue;
        }
        
        let word_vec = peek_take_while(&mut chars, |&ch| {
            is_skip_char(ch) || is_token_char(ch)
        });
        
        let word: String = format!("{}{}",ch,word_vec.into_iter().collect::<String>());
        
        if word == "let" {
            tokens.push_back(Token::Let);
            continue; 
        }
        
        tokens.push_back(Token::Identifier(word))
    }

    Ok(tokens)
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Let,
    // LetIfNotExists,
    // Const,
    // ConstIfNotExists,
    // Include,
    HexColor(Color),
    Identifier(String), // 標準搭載された関数も含める
    // Int(String),
    Assign,
    // LeftPare,
    // RightPare,
    // Comma,
}



#[cfg(test)]
mod test {
    use crate::{color::Color, lexer::{lexer, Token}};

    #[test]
    fn _parse_line() {
        let mut test = "let hello = #ffffff".chars().collect();
        let parsed = Vec::from(lexer(&mut test).unwrap());
        assert_eq!(
            parsed,
            vec![
                Token::Let,
                Token::Identifier("hello".to_string()),
                Token::Assign,
                Token::HexColor(Color {
                    r: 255,
                    g: 255,
                    b: 255
                })
            ]
        );

        let mut test = "     let    hello    = \n \t  #ffffff".chars().collect();
        let parsed = Vec::from(lexer(&mut test).unwrap());
        assert_eq!(
            parsed,
            vec![
                Token::Let,
                Token::Identifier("hello".to_string()),
                Token::Assign,
                Token::HexColor(Color {
                    r: 255,
                    g: 255,
                    b: 255
                })
            ]
        );

        let mut test = "let hello=color1".chars().collect();
        let parsed = Vec::from(lexer(&mut test).unwrap());
        assert_eq!(
            parsed,
            vec![
                Token::Let,
                Token::Identifier("hello".to_string()),
                Token::Assign,
                Token::Identifier("color1".to_string()),
            ]
        );

        let mut test = "_hello==letaaa".chars().collect();
        let parsed = Vec::from(lexer(&mut test).unwrap());
        assert_eq!(
            parsed,
            vec![
                Token::Identifier("_hello".to_string()),
                Token::Assign,
                Token::Assign,
                Token::Identifier("letaaa".to_string()),
            ]
        );

        let mut test = "hello#101010aaa=aaa".chars().collect();
        let parsed = Vec::from(lexer(&mut test).unwrap());
        assert_eq!(
            parsed,
            vec![
                Token::Identifier("hello".to_string()),
                Token::HexColor(Color { r: 16, g: 16, b: 16 }),
                Token::Identifier("aaa".to_string()),
                Token::Assign,
                Token::Identifier("aaa".to_string()),
            ]
        );

        let mut test = "#".chars().collect();
        let parsed = lexer(&mut test);
        assert!(parsed.is_err());

        let mut test = "#aa".chars().collect();
        let parsed = lexer(&mut test);
        assert!(parsed.is_err());

        let mut test = "#gg".chars().collect();
        let parsed = lexer(&mut test);
        assert!(parsed.is_err());

        let mut test = "aaaa#aaaaa".chars().collect();
        let parsed = lexer(&mut test);
        assert!(parsed.is_err());

        let mut test = "aaaa#aa".chars().collect();
        let parsed = lexer(&mut test);
        assert!(parsed.is_err());
    }
}