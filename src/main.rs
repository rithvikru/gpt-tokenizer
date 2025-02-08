mod tokenizer;

use std::collections::HashMap;

// HYPERPARAMETERS //
const VOCAB_SIZE: u16 = 276;
const NUM_MERGES: u16 = VOCAB_SIZE - 256;

fn main() {
    let text = "Ｕｎｉｃｏｄｅ! 🅤🅝🅘🅒🅞🅓🅔‽ 🇺‌🇳‌🇮‌🇨‌🇴‌🇩‌🇪! 😄 The very name strikes fear and awe into the hearts of programmers worldwide. We all know we ought to “support Unicode” in our software (whatever that means—like using wchar_t for all the strings, right?). But Unicode can be abstruse, and diving into the thousand-page Unicode Standard plus its dozens of supplementary annexes, reports, and notes can be more than a little intimidating. I don’t blame programmers for still finding the whole thing mysterious, even 30 years after Unicode’s inception.";
    let tokens: Vec<u16> = tokenizer::encode(text);
    
    let mut ids = tokens.clone();
    let mut merges: HashMap<(u16, u16), u16> = HashMap::new();

    for i in 0..NUM_MERGES {
        let stats = tokenizer::get_stats(&ids);
        let top_pair = stats.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap().0;
        let new_token = 256 + i;
        
        println!("Merging {:?} into new token {:?}", top_pair, new_token);
        ids = tokenizer::merge(&ids, *top_pair, new_token);
        merges.insert(*top_pair, new_token);
    }

    println!("Before encoding: {:?}", String::from_utf16(&tokens).unwrap());
    println!("Before merge: {:?}", tokens.len());

    println!("--------------------------------");

    println!("After encoding: {:?}", String::from_utf16(&ids).unwrap());
    println!("After merge: {:?}", ids.len());

    println!("Compression ratio: {:.2}X", tokens.len() as f32 / ids.len() as f32);

    let mut vocab: HashMap<u16, Vec<u16>> = (0..=255)
        .map(|idx| (idx, vec![idx]))
        .collect();

    for ((byte_a, byte_b), merged_token) in merges.iter() {
        let mut new_tokens = Vec::new();
        if let Some(tokens_a) = vocab.get(byte_a) {
            if let Some(tokens_b) = vocab.get(byte_b) {
                new_tokens.extend(tokens_a);
                new_tokens.extend(tokens_b);
                vocab.insert(*merged_token, new_tokens);
            }
        }
    }

    println!("Vocab: {:?}", vocab);
    println!("Decoded: {:?}", tokenizer::decode(&ids, &vocab));
}