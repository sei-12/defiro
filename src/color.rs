
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

#[derive(Debug, PartialEq)]
pub struct Color {
    pub r: u32,
    pub g: u32,
    pub b: u32,
}

impl Color {
    pub fn from_hex_chars(chars: &mut dyn Iterator<Item = char>) -> Option<Self> {
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
