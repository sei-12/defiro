use std::collections::HashMap;

use crate::{color::Color, fault};

use self::include_file_stack::IncludeFileStack;
mod include_file_stack;

pub struct Envroiment {
    map: HashMap<String, Color>,
    pub faults: Vec<Box<dyn fault::Fault>>,
    pub include_file_stack: IncludeFileStack,
}

impl Envroiment {
    pub fn set(&mut self, name: String, color: Color) {
        self.map.insert(name, color);
    }

    pub fn get(&self, name: &String) -> Option<Color> {
        match self.map.get(name) {
            Some(c) => Some(c.clone()),
            None => None,
        }
    }

    pub fn new() -> Self {
        Envroiment {
            map: HashMap::new(),
            faults: Vec::new(),
            include_file_stack: IncludeFileStack::new(),
        }
    }

    pub fn vars_json(&self) -> String {
        let mut buf = String::new();
        buf += "{";
        let mut vars = Vec::new();

        for var in &self.map {
            vars.push(format!("\"{}\":\"{}\"", var.0, var.1.to_hex_string()));
        }

        vars.sort();

        buf += &vars.join(",");

        buf += "}";
        buf
    }
}

#[cfg(test)]
impl Envroiment {
    pub fn vars_len(&self) -> usize {
        self.map.len()
    }
}

#[cfg(test)]
mod test {
    use crate::color::Color;

    use super::Envroiment;

    #[test]
    fn vars_json() {
        test_vars_json(
            vec![("helo", Color::new(10, 10, 10))],
            "{\"helo\":\"#0a0a0a\"}",
        );

        test_vars_json(
            vec![("helo", Color::new(20, 16, 255))],
            "{\"helo\":\"#1410ff\"}",
        );

        test_vars_json(
            vec![
                ("a", Color::new(20, 16, 255)),
                ("b", Color::new(20, 16, 255)),
                ("c", Color::new(20, 16, 255)),
            ],
            "{\"a\":\"#1410ff\",\"b\":\"#1410ff\",\"c\":\"#1410ff\"}",
        );

        // 上書き
        test_vars_json(
            vec![
                ("a", Color::new(20, 16, 255)),
                ("c", Color::new(20, 16, 255)),
                ("a", Color::new(16, 16, 16)),
            ],
            "{\"a\":\"#101010\",\"c\":\"#1410ff\"}",
        );

        test_vars_json(vec![], "{}");

        test_vars_json(
            vec![
                ("a", Color::new(20, 16, 255)),
                ("b", Color::new(20, 16, 255)),
            ],
            "{\"a\":\"#1410ff\",\"b\":\"#1410ff\"}",
        );
    }

    fn test_vars_json(vars: Vec<(&str, Color)>, json: &str) {
        let mut env = Envroiment::new();
        for v in vars {
            env.set(v.0.to_string(), v.1);
        }
        assert_eq!(env.vars_json(), json);
    }
}
