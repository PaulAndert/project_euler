
fn check_palindrome(pal: String) -> bool {
    let size: usize = pal.len();
    let chars: Vec<char> = pal.chars().collect();
    
    if size % 2 == 0 { // even
        for i in 0..(size / 2) {
            if chars[i] == chars[chars.len() - (i + 1)] {
            }else {
                return false;
            }
        }
        return true;
    }else { // odd
        for i in 0..(size / 2 + 1) {
            if chars[i] == chars[chars.len() - (i + 1)] {
            }else {
                return false;
            }
        }
        return true;
    }
}

fn base_10_to_2(mut pal: u64) -> String {
    let mut pal_vec: Vec<u64> = Vec::new();
    let mut ret_str: String = String::new();

    while pal >= 1 {
        pal_vec.push(pal % 2);
        pal /= 2;
    }

    pal_vec.reverse();

    for i in pal_vec {
        ret_str = format!("{}{}", ret_str, i);
    }
    return ret_str;
}

pub fn main() {
    let mut palindromes: Vec<u64> = Vec::new();
    let mut cnt: u64 = 0;

    for i in 1..1000000 {
        if check_palindrome(format!("{}", i)) {
            palindromes.push(i);
        }
    }

    for i in palindromes {
        if check_palindrome(base_10_to_2(i)) {
            cnt += i;
        }
    }

    println!("Cnt: {}", cnt);    
}