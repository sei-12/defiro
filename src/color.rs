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

pub type ColorInt = u8;

#[derive(Debug, Clone, PartialEq)]
pub struct Color {
    r: ColorInt,
    g: ColorInt,
    b: ColorInt,
}

impl Color {
    pub fn new(r: ColorInt, g: ColorInt, b: ColorInt) -> Self {
        Color { r, g, b }
    }

    pub fn plus(&self, r: ColorInt, g: ColorInt, b: ColorInt) -> Self {
        let new_r = self.r.checked_add(r).unwrap_or(255);
        let new_g = self.g.checked_add(g).unwrap_or(255);
        let new_b = self.b.checked_add(b).unwrap_or(255);

        Self {
            r: new_r,
            g: new_g,
            b: new_b,
        }
    }
    pub fn minus(&self, r: ColorInt, g: ColorInt, b: ColorInt) -> Self {
        let new_r = self.r.checked_sub(r).unwrap_or(0);
        let new_g = self.g.checked_sub(g).unwrap_or(0);
        let new_b = self.b.checked_sub(b).unwrap_or(0);

        Self {
            r: new_r,
            g: new_g,
            b: new_b,
        }
    }

    pub fn from_hex_chars(chars: &mut dyn Iterator<Item = char>) -> Option<Self> {
        let two_chars = pairwise_concat(chars);

        let Ok(r) = u8::from_str_radix(two_chars[0].as_str(), 16) else {
            return None;
        };
        let Ok(g) = u8::from_str_radix(two_chars[1].as_str(), 16) else {
            return None;
        };
        let Ok(b) = u8::from_str_radix(two_chars[2].as_str(), 16) else {
            return None;
        };

        Some(Color { r, g, b })
    }

    pub fn to_hex_string(&self) -> String {
        format!("#{:02x}{:02x}{:02x}", self.r, self.g, self.b)
    }
}
