use std::{
    collections::VecDeque, io::{BufReader, Read}
};

fn peek_take_while<T>(iter: &mut VecDeque<T>,check: fn(&T) -> bool )-> VecDeque<T> {
    let mut ret_vec = VecDeque::new();
    loop {
        let Some(item) = iter.front() else {
            break;
        };
        if check(item) {
            break;
        }
        let Some(item) = iter.pop_front() else {
            break;
        };
        ret_vec.push_back(item);
    }
    ret_vec
}

#[derive(Debug, PartialEq)]
struct Color {
    r: u32,
    g: u32,
    b: u32,
}

fn pairwise_concat(chars: &mut dyn Iterator<Item = char>) -> Vec<String> {
    let mut ret_vec = Vec::new();
    loop {
        let Some(str1) = chars.next() else {
            break;
        };
        let Some(str2) = chars.next() else {
            break;
        };
        ret_vec.push(format!("{}{}", str1, str2))
    }

    ret_vec
}

impl Color {
    fn from_hex_chars(chars: &mut dyn Iterator<Item = char>) -> Option<Self> {
        let two_chars = pairwise_concat(chars);

        let Ok(r) = u32::from_str_radix(two_chars[0].as_str(), 16) else {
            return None;
        };
        let Ok(g) = u32::from_str_radix(two_chars[1].as_str(), 16) else {
            return None;
        };
        let Ok(b) = u32::from_str_radix(two_chars[2].as_str(), 16) else {
            return None;
        };

        Some(Color { r, g, b })
    }
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
enum ParseFault {
    Value,
}

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

fn parse_line(mut chars: &mut VecDeque<char>) -> Result<VecDeque<Token>, ParseFault> {
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
                return Err(ParseFault::Value);
            }
            let mut hex_iter = hex.into_iter();
            let Some(color) = Color::from_hex_chars(&mut hex_iter) else {
                return Err(ParseFault::Value);
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
enum Token {
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

fn main() {
    let mut reader = BufReader::new(std::io::stdin());
    let mut stdin_string = String::new();
    reader.read_to_string(&mut stdin_string).unwrap();
    let mut file_chars: VecDeque<char> = stdin_string.chars().collect();

    loop {
        if file_chars.front().is_none() {
            break;
        }
        
        let line = peek_take_while(&mut file_chars, |&ch| ch == ';');
        file_chars.pop_front();

        let line_string: String = line.into_iter().collect();
        let mut tmp = line_string.chars().collect();
        let _tokens = parse_line(&mut tmp);

    }
}

#[cfg(test)]
mod test {

    
    use std::collections::VecDeque;

    use crate::{parse_line, peek_take_while, Color, Token};

    #[test]
    fn _peek_take_while(){
        let mut deque = VecDeque::from(vec![1, 2, 3, 4, 5]);
        let result = peek_take_while(&mut deque, |&x| x == 3);
        assert_eq!(result, VecDeque::from(vec![1, 2]));
        assert_eq!(deque, VecDeque::from(vec![3, 4, 5]));

        let mut deque : VecDeque<usize>= VecDeque::new();
        let result = peek_take_while(&mut deque, |&x| x == 3);
        assert_eq!(result, VecDeque::new());
        assert_eq!(deque, VecDeque::new());

        let mut deque = VecDeque::from(vec![1, 2, 3, 4, 5]);
        let result = peek_take_while(&mut deque, |&x| x == 10);
        assert_eq!(result, VecDeque::from(vec![1, 2, 3, 4, 5]));
        assert_eq!(deque, VecDeque::new());
    }

    #[test]
    fn _parse_line() {
        let mut test = "let hello = #ffffff".chars().collect();
        let parsed = Vec::from(parse_line(&mut test).unwrap());
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
        let parsed = Vec::from(parse_line(&mut test).unwrap());
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
        let parsed = Vec::from(parse_line(&mut test).unwrap());
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
        let parsed = Vec::from(parse_line(&mut test).unwrap());
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
        let parsed = Vec::from(parse_line(&mut test).unwrap());
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
        let parsed = parse_line(&mut test);
        assert!(parsed.is_err());

        let mut test = "#aa".chars().collect();
        let parsed = parse_line(&mut test);
        assert!(parsed.is_err());

        let mut test = "#gg".chars().collect();
        let parsed = parse_line(&mut test);
        assert!(parsed.is_err());

        let mut test = "aaaa#aaaaa".chars().collect();
        let parsed = parse_line(&mut test);
        assert!(parsed.is_err());

        let mut test = "aaaa#aa".chars().collect();
        let parsed = parse_line(&mut test);
        assert!(parsed.is_err());

        
    }
}
