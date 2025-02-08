use std::collections::HashMap;

pub fn get_stats(tokens: &[u16]) -> HashMap<(u16, u16), i32> {
    let mut freq: HashMap<(u16, u16), i32> = HashMap::new();
    for byte_pair in tokens.windows(2) {
        if let [byte_a, byte_b] = byte_pair {
            *freq.entry((*byte_a, *byte_b)).or_insert(0) += 1;
        }
    }
    freq
}

pub fn merge(ids: &[u16], pair: (u16, u16), idx: u16) -> Vec<u16> {
    let mut new_ids: Vec<u16> = Vec::new();
    let mut i = 0;
    while i < ids.len() {
        if i < ids.len() - 1 && ids[i] == pair.0 && ids[i + 1] == pair.1 {
            new_ids.push(idx);
            i += 2;
        } else {
            new_ids.push(ids[i] as u16);
            i += 1;
        }
    }
    new_ids
}

pub fn encode(text: &str) -> Vec<u16> {
    let tokens: Vec<u16> = text.as_bytes().iter().map(|&b| b as u16).collect();
    tokens
}

pub fn decode(tokens: &[u16], vocab: &HashMap<u16, Vec<u16>>) -> String {
    let mut text = String::new();
    for token in tokens {
        if let Some(tokens) = vocab.get(token) {
            text.push_str(&tokens.iter().map(|&t| t as u8 as char).collect::<String>());
        } else {
            text.push(*token as u8 as char);
        }
    }
    text
}