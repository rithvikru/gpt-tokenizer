use std::collections::HashMap;

// HYPERPARAMETERS //
const VOCAB_SIZE: u16 = 276;
const NUM_MERGES: u16 = VOCAB_SIZE - 256;

fn get_stats(tokens: &[u16]) -> HashMap<(u16, u16), i32> {
    let mut freq: HashMap<(u16, u16), i32> = HashMap::new();
    for byte_pair in tokens.windows(2) {
        if let [byte_a, byte_b] = byte_pair {
            *freq.entry((*byte_a, *byte_b)).or_insert(0) += 1;
        }
    }
    freq
}

fn merge(ids: &[u16], pair: (u16, u16), idx: u16) -> Vec<u16> {
    let mut new_ids: Vec<u16> = Vec::new();
    let mut i = 0;
    while i < ids.len() {
        if i < ids.len() - 1 && ids[i] == pair.0 && ids[i + 1] == pair.1 {
            new_ids.push(idx);
            i += 2;
        }
        else {
            new_ids.push(ids[i] as u16);
            i += 1;
        }
    }   
    new_ids
}

fn main() {
    let text = "Ｕｎｉｃｏｄｅ! 🅤🅝🅘🅒🅞🅓🅔‽ 🇺‌🇳‌🇮‌🇨‌🇴‌🇩‌🇪! 😄 The very name strikes fear and awe into the hearts of programmers worldwide. We all know we ought to “support Unicode” in our software (whatever that means—like using wchar_t for all the strings, right?). But Unicode can be abstruse, and diving into the thousand-page Unicode Standard plus its dozens of supplementary annexes, reports, and notes can be more than a little intimidating. I don’t blame programmers for still finding the whole thing mysterious, even 30 years after Unicode’s inception.";
    let tokens: Vec<u16> = text.as_bytes().iter().map(|&b| b as u16).collect();
    
    let mut ids = tokens.clone();
    let mut merges: HashMap<(u16, u16), u16> = HashMap::new();

    for i in 0..NUM_MERGES {
        let stats = get_stats(&ids);
        let top_pair = stats.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap().0;
        let new_token = 256 + i;
        
        println!("Merging {:?} into new token {:?}", top_pair, new_token);
        ids = merge(&ids, *top_pair, new_token);
        merges.insert(*top_pair, new_token);
    }

    println!("Before merge: {:?}", tokens.len());
    println!("After merge: {:?}", ids.len());
}  