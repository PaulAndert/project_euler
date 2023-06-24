

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

fn add_2_strings(one: String, two: String) -> String {

    let one_array: Vec<u8> = one.as_bytes().iter().map(|b| b - 48).collect();
    let two_array: Vec<u8> = two.as_bytes().iter().map(|b| b - 48).collect();

    let mut sum: String = String::new();
    let mut carry: u8 = 0;

    for i in 0..one_array.len() {
        let temp = one_array[one_array.len() - (i + 1)] + two_array[one_array.len() - (i + 1)] + carry;
        carry = 0;
        if temp > 9 {
            sum = format!("{}{}", sum, temp - 10);
            carry = temp / 10;
        }else {
            sum = format!("{}{}", sum, temp);
        }
    }
    if carry != 0 {
        sum = format!("{}{}", sum, carry);
    }

    return sum.chars().rev().collect::<String>();

}

fn check_lychrel(mut number: String) -> bool {
    let mut number_rev: String;
    let mut sum: String;

    for _i in 0..50 {
        number_rev = number.chars().rev().collect();
        sum = add_2_strings(number, number_rev);
        if check_palindrome(sum.clone()) {
            return false;
        }else {
            number = sum;
        }
    }
    return true;
}

pub fn main() {

    let mut cnt = 0;
    for i in 1..=10000 {
        if check_lychrel(format!("{}", i)) {
            cnt += 1;
        }
    }
    println!("{}", cnt);
}