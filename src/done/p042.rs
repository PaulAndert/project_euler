use std::fs;

fn check_triangle_number(num: u64) -> bool {
    let mut check: u64 = 0;
    let mut n: f32 = 1.0;
    while num > check {
        check = ( n * 0.5 * (n + 1.0) ) as u64;
        if check == num {
            return true;
        }
        n += 1.0;
    }
    false
}

fn string_to_number(word: String) -> u64 {
    return word.chars().map(|c| c as u64 - 64).sum();
}

pub fn main() {
    let mut words: Vec<String> = Vec::new();
    let contents: String = fs::read_to_string("resources/042_words.txt").expect("Should have been able to read the file");
    let words_split = contents.split(",");
    for word in words_split {
        let mut chars = word.chars();
        chars.next(); // remove first and last char ("..")
        chars.next_back();
        words.push(chars.as_str().to_string());
    }
    
    let mut result: u64 = 0;
    for word in words {
        if check_triangle_number(string_to_number(word)) {
            result += 1;
        }
    }
    println!("Result: {}", result);
}