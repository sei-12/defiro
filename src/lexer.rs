use crate::{
    color::{self, Color},
    fault,
    utils::peek_take_while,
};
use std::collections::VecDeque;

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
    if ch == '(' {
        return true;
    }
    if ch == ')' {
        return true;
    }
    if ch == ',' {
        return true;
    }
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
impl fault::Fault for LexFault {
    fn msg(&self) -> String {
        match self {
            LexFault::Value => {
                format!("LexError: value error")
            }
        }
    }
}

// TODO refactor
fn remove_comment_line(chars: &mut VecDeque<char>) -> VecDeque<char> {
    let mut ret_chars = VecDeque::new();

    let mut old_char = match chars.pop_front() {
        Some(c) => c,
        None => {
            return ret_chars;
        }
    };

    loop {
        let Some(current) = chars.pop_front() else {
            ret_chars.push_back(old_char);
            break;
        };

        if old_char == '/' && current == '/' {
            peek_take_while(chars, |&ch| ch == '\n');

            match chars.pop_front() {
                Some(c) => old_char = c,
                None => {
                    break;
                }
            }
        } else {
            ret_chars.push_back(old_char);
            old_char = current;
        }
    }

    ret_chars
}

pub fn lexer(chars: &mut VecDeque<char>) -> Result<VecDeque<Token>, LexFault> {
    let mut chars = remove_comment_line(chars);
    let mut tokens = VecDeque::new();

    loop {
        let Some(ch) = chars.pop_front() else {
            break;
        };

        if is_skip_char(ch) {
            continue;
        }

        if ch == '=' {
            tokens.push_back(Token::Assign);
            continue;
        }
        if ch == '(' {
            tokens.push_back(Token::LeftPare);
            continue;
        }
        if ch == ')' {
            tokens.push_back(Token::RightPare);
            continue;
        }
        if ch == ',' {
            tokens.push_back(Token::Comma);
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

        let word_vec = peek_take_while(&mut chars, |&ch| is_skip_char(ch) || is_token_char(ch));

        let word: String = format!("{}{}", ch, word_vec.into_iter().collect::<String>());

        if word == "let" {
            tokens.push_back(Token::Let);
            continue;
        }

        if word == "include" {
            tokens.push_back(Token::Include);
            continue;
        }

        if let Ok(int) = word.parse::<color::ColorInt>() {
            tokens.push_back(Token::Int(int));
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
    Include,
    HexColor(Color),
    Identifier(String), // 標準搭載された関数も含める
    Int(color::ColorInt),
    Assign,
    LeftPare,
    RightPare,
    Comma,
}

#[cfg(test)]
mod test {
    use crate::{
        color::Color,
        lexer::{lexer, Token},
    };
    fn test_lexer(line: &str, assert: Vec<Token>) {
        let mut test = line.chars().collect();
        let parsed = Vec::from(lexer(&mut test).unwrap());
        assert_eq!(parsed, assert);
    }

    #[test]
    fn _parse_line() {
        test_lexer(
            "hello\n//hello\nfoo // hello \nlet hello",
            vec![
                Token::Identifier("hello".to_string()),
                Token::Identifier("foo".to_string()),
                Token::Let,
                Token::Identifier("hello".to_string()),
            ],
        );

        test_lexer(
            "hello\n//hello\nfoo",
            vec![
                Token::Identifier("hello".to_string()),
                Token::Identifier("foo".to_string()),
            ],
        );

        test_lexer("hello/", vec![Token::Identifier("hello/".to_string())]);

        test_lexer("hello//", vec![Token::Identifier("hello".to_string())]);

        test_lexer("// hello", vec![]);

        test_lexer(
            "\
            // hello
            let hello
            // hello
            ",
            vec![Token::Let, Token::Identifier("hello".to_string())],
        );

        test_lexer(
            "\
            include / /
            ",
            vec![
                Token::Include,
                Token::Identifier("/".to_string()),
                Token::Identifier("/".to_string()),
            ],
        );

        let mut test = "()=,".chars().collect();
        let parsed = Vec::from(lexer(&mut test).unwrap());
        assert_eq!(
            parsed,
            vec![
                Token::LeftPare,
                Token::RightPare,
                Token::Assign,
                Token::Comma,
            ]
        );

        let mut test = "let hello = #ffffff".chars().collect();
        let parsed = Vec::from(lexer(&mut test).unwrap());
        assert_eq!(
            parsed,
            vec![
                Token::Let,
                Token::Identifier("hello".to_string()),
                Token::Assign,
                Token::HexColor(Color::new(255, 255, 255))
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
                Token::HexColor(Color::new(255, 255, 255))
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
                Token::HexColor(Color::new(16, 16, 16)),
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

        let mut test = "((((()))))aaa((((()))))".chars().collect();
        let parsed = Vec::from(lexer(&mut test).unwrap());
        assert_eq!(
            parsed,
            vec![
                Token::LeftPare,
                Token::LeftPare,
                Token::LeftPare,
                Token::LeftPare,
                Token::LeftPare,
                Token::RightPare,
                Token::RightPare,
                Token::RightPare,
                Token::RightPare,
                Token::RightPare,
                Token::Identifier("aaa".to_string()),
                Token::LeftPare,
                Token::LeftPare,
                Token::LeftPare,
                Token::LeftPare,
                Token::LeftPare,
                Token::RightPare,
                Token::RightPare,
                Token::RightPare,
                Token::RightPare,
                Token::RightPare,
            ]
        );
        let mut test = "let aaa = hello(aaa)".chars().collect();
        let parsed = Vec::from(lexer(&mut test).unwrap());
        assert_eq!(
            parsed,
            vec![
                Token::Let,
                Token::Identifier("aaa".to_string()),
                Token::Assign,
                Token::Identifier("hello".to_string()),
                Token::LeftPare,
                Token::Identifier("aaa".to_string()),
                Token::RightPare,
            ]
        );

        let mut test = "let( )  aaa()   ( aaa )".chars().collect();
        let parsed = Vec::from(lexer(&mut test).unwrap());
        assert_eq!(
            parsed,
            vec![
                Token::Let,
                Token::LeftPare,
                Token::RightPare,
                Token::Identifier("aaa".to_string()),
                Token::LeftPare,
                Token::RightPare,
                Token::LeftPare,
                Token::Identifier("aaa".to_string()),
                Token::RightPare,
            ]
        );
        let mut test = "let()aaa()(aaa)".chars().collect();
        let parsed = Vec::from(lexer(&mut test).unwrap());
        assert_eq!(
            parsed,
            vec![
                Token::Let,
                Token::LeftPare,
                Token::RightPare,
                Token::Identifier("aaa".to_string()),
                Token::LeftPare,
                Token::RightPare,
                Token::LeftPare,
                Token::Identifier("aaa".to_string()),
                Token::RightPare,
            ]
        );

        let mut test = "a-b 100-10".chars().collect();
        let parsed = Vec::from(lexer(&mut test).unwrap());
        assert_eq!(
            parsed,
            vec![
                Token::Identifier("a-b".to_string()),
                Token::Identifier("100-10".to_string()),
            ]
        );

        let mut test = "255 256 0 -1".chars().collect();
        let parsed = Vec::from(lexer(&mut test).unwrap());
        assert_eq!(
            parsed,
            vec![
                Token::Int(255),
                Token::Identifier("256".to_string()),
                Token::Int(0),
                Token::Identifier("-1".to_string()),
            ]
        );

        let mut test = "a(1,3,4,hello,  aaa,a-b, \n aaa\na,s)".chars().collect();
        let parsed = Vec::from(lexer(&mut test).unwrap());
        assert_eq!(
            parsed,
            vec![
                Token::Identifier("a".to_string()),
                Token::LeftPare,
                Token::Int(1),
                Token::Comma,
                Token::Int(3),
                Token::Comma,
                Token::Int(4),
                Token::Comma,
                Token::Identifier("hello".to_string()),
                Token::Comma,
                Token::Identifier("aaa".to_string()),
                Token::Comma,
                Token::Identifier("a-b".to_string()),
                Token::Comma,
                Token::Identifier("aaa".to_string()),
                Token::Identifier("a".to_string()),
                Token::Comma,
                Token::Identifier("s".to_string()),
                Token::RightPare
            ]
        );

        let mut test = "let a10 = rgb(10) 100 a0 0xa0 4294967297".chars().collect();
        let parsed = Vec::from(lexer(&mut test).unwrap());
        assert_eq!(
            parsed,
            vec![
                Token::Let,
                Token::Identifier("a10".to_string()),
                Token::Assign,
                Token::Identifier("rgb".to_string()),
                Token::LeftPare,
                Token::Int(10),
                Token::RightPare,
                Token::Int(100),
                Token::Identifier("a0".to_string()),
                Token::Identifier("0xa0".to_string()),
                Token::Identifier("4294967297".to_string()),
            ]
        );
    }
}
