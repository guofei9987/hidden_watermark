use crypt_tool::{system_random, CryptConverter};

pub struct TextBlindWM {
    crypt_converter: CryptConverter,
    chr0: char,
    chr1: char,
}

impl TextBlindWM {
    pub fn new(pwd: &[u8]) -> Self {
        Self {
            crypt_converter: CryptConverter::new(pwd),
            chr0: char::from_u32(0x2060).unwrap(),
            chr1: char::from_u32(0xFEFF).unwrap(),
        }
    }

    pub fn new_with_char(pwd: &[u8], chr0: u32, chr1: u32) -> Self {
        Self {
            crypt_converter: CryptConverter::new(pwd),
            chr0: char::from_u32(chr0).unwrap(),
            chr1: char::from_u32(chr1).unwrap(),
        }
    }

    pub fn generate_watermark(&self, wm: &[u8]) -> String {
        let wm_bin = self.crypt_converter.encode(wm);

        wm_bin
            .into_iter()
            .map(|bit| {
                if bit == 0 { self.chr0 } else { self.chr1 }
            })
            .collect()
    }

    pub fn add_wm_at_idx(&self, text: &str, wm: &[u8], byte_idx: usize) -> String {
        let text = self.remove_watermark(text);
        let wm_dark = self.generate_watermark(wm);

        let byte_idx = byte_idx.min(text.len());

        // 如果 byte_idx 不在字符边界上，向前查找最近的有效边界
        let valid_byte_idx = if text.is_char_boundary(byte_idx) {
            byte_idx
        } else {
            (0..=byte_idx)
                .rev()
                .find(|&i| text.is_char_boundary(i))
                .unwrap_or(0)
        };

        // 构建结果字符串
        let mut res = String::with_capacity(text.len() + wm_dark.len());

        res.push_str(&text[..valid_byte_idx]);
        res.push_str(&wm_dark);
        res.push_str(&text[valid_byte_idx..]);

        res
    }

    pub fn add_wm_at_last(&self, text: &str, wm: &[u8]) -> String {
        self.add_wm_at_idx(text, wm, text.len())
    }

    /// add watermark in random index
    pub fn add_wm_rnd(&self, text: &str, wm: &[u8]) -> String {
        let idx = (system_random() as usize) % text.len();
        self.add_wm_at_idx(text, wm, idx)
    }

    pub fn remove_watermark(&self, text: &str) -> String {
        let text_char: Vec<char> = text.chars().collect();
        let mut res = String::with_capacity(text.len());
        for chr in text_char {
            if chr != self.chr0 && chr != self.chr1 {
                res.push(chr);
            }
        }
        res
    }

    pub fn extract(&self, text_with_wm: &str) -> Vec<u8> {
        let mut idx_left: Option<usize> = None;
        let mut idx_right: Option<usize> = None;

        for (idx, chr) in text_with_wm.chars().enumerate() {
            if chr == self.chr0 || chr == self.chr1 {
                if idx_left.is_none() {
                    idx_left = Some(idx);
                }
            } else if idx_left.is_some() && idx_right.is_none() {
                idx_right = Some(idx);
                break;
            }
        }

        if idx_left.is_some() {
            if idx_right.is_none() {
                idx_right = Some(text_with_wm.len());
            }
        } else {
            return vec![];
        }

        let wm_bin: Vec<u8> = text_with_wm
            .chars()
            .skip(idx_left.unwrap())
            .take(idx_right.unwrap() - idx_left.unwrap())
            .map(|x| if x == self.chr0 { 0u8 } else { 1u8 })
            .collect();

        self.crypt_converter.decode(&wm_bin)
    }
}
