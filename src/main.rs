use std::collections::HashMap;

fn get_stats(tokens: &[u8]) -> HashMap<(u8, u8), i32> {
    let mut freq: HashMap<(u8, u8), i32> = HashMap::new();
    for byte_pair in tokens.windows(2) {
        if let [byte_a, byte_b] = byte_pair {
            *freq.entry((*byte_a, *byte_b)).or_insert(0) += 1;
        }
    }
    freq
}


fn merge(ids: &[u8], pair: (u8, u8), idx: u8) -> Vec<u8> {
    let mut new_ids: Vec<u8> = Vec::new();
    let mut i = 0;
    while i < ids.len() {
        if i < ids.len() - 1 && ids[i] == pair.0 && ids[i + 1] == pair.1 {
            new_ids.push(idx);
            i += 2;
        }
        else {
            new_ids.push(ids[i]);
            i += 1;
        }
    }
    new_ids
}

fn main() {
    let text = "Ｕｎｉｃｏｄｅ! 🅤🅝🅘🅒🅞🅓🅔‽ 🇺‌🇳‌🇮‌🇨‌🇴‌🇩‌🇪! 😄 The very name strikes fear and awe into the hearts of programmers worldwide. We all know we ought to “support Unicode” in our software (whatever that means—like using wchar_t for all the strings, right?). But Unicode can be abstruse, and diving into the thousand-page Unicode Standard plus its dozens of supplementary annexes, reports, and notes can be more than a little intimidating. I don’t blame programmers for still finding the whole thing mysterious, even 30 years after Unicode’s inception.";
    let tokens: &[u8] = text.as_bytes();
    let stats = get_stats(tokens);
    let top_pair = stats.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap().0;
    
    println!("{:?}", merge(tokens, *top_pair, 99))
}  