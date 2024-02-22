use std::{collections::VecDeque, io::{BufReader, Read}, str::Chars};

struct Color {
    r: u32,
    g: u32,
    b: u32,
}

fn pairwise_concat(mut chars: VecDeque<char>)-> Vec<String>{
    let mut ret_vec = Vec::with_capacity(chars.len() / 2);

    loop {
        let Some(str1) = chars.pop_front() else {
            break;
        };
        let Some(str2) = chars.pop_front() else {
            break;
        };
        ret_vec.push(format!("{}{}",str1,str2))
    } 
    
    ret_vec
}

impl Color {
    fn from_hex_string(str: String) -> Option<Self>{
        let mut chars: VecDeque<char> = str.chars().collect();         

        if chars.pop_front() != Some('#') {
            return None;
        };
        
        if chars.len() != 6 {
            return None;
        }

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

fn pop_next_semicolon(file_chars: &mut dyn Iterator<Item = char>) -> String {
    let mut line = "".to_string();
    loop {
        let Some(c) = file_chars.next() else {
            break;
        }; 
        
        line.push(c);

        if c == ';' {
            break;
        }
    } 
    line
}


fn main() {
    let mut reader = BufReader::new(std::io::stdin());
    let mut stdin_string = String::new();
    reader.read_to_string(&mut stdin_string).unwrap();
    let mut file_chars = stdin_string.chars().peekable();

    loop {
        if file_chars.peek().is_none() {
            break;
        }
        
        let line = pop_next_semicolon(&mut file_chars);
        println!("line: {}",line)        
    }   
}


#[cfg(test)]
mod test {

    use crate::{pairwise_concat, pop_next_semicolon, Color};
    #[test]
    fn _pairwise_concat(){
        let test = "hello".to_string(); 
        let result = pairwise_concat(test.chars().collect());
        assert_eq!(vec!["he","ll"],result);

        let test = "konnnitiwa".to_string(); 
        let result = pairwise_concat(test.chars().collect());
        assert_eq!(vec!["ko","nn","ni","ti","wa"],result);

        let test = "a".to_string(); 
        let result = pairwise_concat(test.chars().collect());
        let check: Vec<String> = vec![];
        assert_eq!(check,result);

        let test = "".to_string(); 
        let result = pairwise_concat(test.chars().collect());
        let check: Vec<String> = vec![];
        assert_eq!(check,result);

        let test = "abcdefg".to_string(); 
        let result = pairwise_concat(test.chars().collect());
        assert_eq!(vec!["ab","cd","ef"],result);
    }
    
    #[test]
    fn _pop_next_semicolon(){
        let test = "hello;hello;".to_string(); 
        let mut chars = test.chars();
        assert_eq!(pop_next_semicolon(&mut chars),"hello;");
        assert_eq!(pop_next_semicolon(&mut chars),"hello;");

        let test = "aaa;bbb;cc".to_string(); 
        let mut chars = test.chars();
        assert_eq!(pop_next_semicolon(&mut chars),"aaa;");
        assert_eq!(pop_next_semicolon(&mut chars),"bbb;");
        assert_eq!(pop_next_semicolon(&mut chars),"cc");

        let test = ";aaa;bbb;cc".to_string(); 
        let mut chars = test.chars();
        assert_eq!(pop_next_semicolon(&mut chars),";");
        assert_eq!(pop_next_semicolon(&mut chars),"aaa;");
        assert_eq!(pop_next_semicolon(&mut chars),"bbb;");
        assert_eq!(pop_next_semicolon(&mut chars),"cc");
    }

    #[test]
    fn from_hex_string(){
        let test1 = "#ffffff".to_string();
        let color1 = Color::from_hex_string(test1).unwrap();
        assert_eq!(color1.r,255);
        assert_eq!(color1.g,255);
        assert_eq!(color1.b,255);
        
        let test1 = "#101010".to_string();
        let color1 = Color::from_hex_string(test1).unwrap();
        assert_eq!(color1.r,16);
        assert_eq!(color1.g,16);
        assert_eq!(color1.b,16);

        let test1 = "#203040".to_string();
        let color1 = Color::from_hex_string(test1).unwrap();
        assert_eq!(color1.r,32);
        assert_eq!(color1.g,48);
        assert_eq!(color1.b,64);

        let test1 = "#eebbcc".to_string();
        let color1 = Color::from_hex_string(test1).unwrap();
        assert_eq!(color1.r,238);
        assert_eq!(color1.g,187);
        assert_eq!(color1.b,204);
        
        let test1 = "203040".to_string();
        let color1 = Color::from_hex_string(test1);
        assert!(color1.is_none());
        
        let test1 = "hello".to_string();
        let color1 = Color::from_hex_string(test1);
        assert!(color1.is_none());

        let test1 = "#fgfg00".to_string();
        let color1 = Color::from_hex_string(test1);
        assert!(color1.is_none());
        
        let test1 = "#fffff".to_string();
        let color1 = Color::from_hex_string(test1);
        assert!(color1.is_none());

        let test1 = "#fffffff".to_string();
        let color1 = Color::from_hex_string(test1);
        assert!(color1.is_none());

    }
}