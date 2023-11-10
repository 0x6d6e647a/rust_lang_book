use std::io::{self, BufRead};

const VOWELS: &[char] = &['a', 'e', 'i', 'o', 'u', 'y'];

fn to_pig_latin(word: &str) -> String {
    // -- Handle empty string.
    let first_char = match word.chars().next() {
        Some(c) => c,
        _ => return String::from(""),
    };

    // -- Handle starts with vowel.
    if VOWELS.contains(&first_char) {
        return format!("{word}-hay");
    }

    // -- Handle starts with consonant.
    format!("{}-{first_char}ay", &word[1..])
}

fn main() {
    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        let line = match line {
            Ok(line) => line,
            Err(error) => panic!("{error:?}"),
        };

        let mut result = vec![];

        for word in line.split_whitespace() {
            // -- Word is only alphabetical.
            if word.chars().all(|c| c.is_alphabetic()) {
                result.push(to_pig_latin(word));
                continue;
            }

            // -- Word with non-alphabetical characters.
            let mut new_word = String::new();
            let mut last = 0;

            for (index, matched) in word.match_indices(|c: char| !c.is_alphabetic()) {
                new_word.push_str(&to_pig_latin(&word[last..index]));
                new_word.push_str(matched);
                last = index + matched.len();
            }

            result.push(new_word);
        }

        println!("{}", result.join(" "));
    }
}
