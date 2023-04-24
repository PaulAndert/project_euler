
fn check_pandigital(mut num: u64) -> bool {
    let mut ret: Vec<u64> = Vec::new();

    while num > 0 {
        ret.push(num % 10);
        num /= 10;
    }
    ret.sort();
    for i in 0..9 {
        if ret[i] != (i + 1) as u64 {
            return false;
        }
    }

    return true;
}

fn contains_zero(mut num: u64) -> bool {
    while num > 0 {
        if num % 10 == 0 {
            return true;
        }
        num /= 10;
    }
    return false;
}

fn check_concatenated_product(num: u64) -> Option<u64> {
    let mut num_str: String = String::new();
    let mut cnt: u64 = 1;

    while num_str.chars().count() < 9 {
        let inner_product = num * cnt;
        if contains_zero(inner_product) {
            return None;
        }
        num_str = format!("{}{}", num_str, inner_product);
        cnt += 1;
    }

    match &num_str[..9].parse::<u64>() {
        Err(e) => { println!("Error: {}", e); },
        Ok(val) => { 
            if check_pandigital(*val) {
                return Some(*val);
            }
        },
    }

    return None;
}

pub fn main() {
    let mut result: u64 = 0;
    for num in 1..10000 {
        match check_concatenated_product(num) {
            None => {},
            Some(pan) => {
                if pan > result {
                    result = pan;
                }
            },
        }
    }
    println!("Result: {}", result);
}