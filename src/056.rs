
fn u64_to_vec(mut num: u64) -> Vec<u8> {
    let mut ret: Vec<u8> = Vec::new();

    while num > 0 {
        ret.push((num % 10) as u8);
        num /= 10;
    }
    ret.reverse();
    return ret;
}

fn adding_2_vec(one_vec: Vec<u8>, two_vec: Vec<u8>) -> Vec<u8> {

    let mut carry: u8 = 0;
    
    let len = if one_vec.len() > two_vec.len() { one_vec.len() } else { two_vec.len()};
    
    let mut sum: Vec<u8> = Vec::new();

    for i in 0..len {
        let a = if one_vec.len() >= (i + 1) {
            one_vec[one_vec.len() - (i + 1)]
        }else {
            0
        };
        let b = if two_vec.len() >= (i + 1) {
            two_vec[two_vec.len() - (i + 1)]
        }else {
            0
        };

        let temp = a + b + carry;
        carry = 0;
        if temp > 9 {
            sum.push(temp - 10);
            carry = temp / 10;
        }else {
            sum.push(temp);
        }
    }
    if carry != 0 {
        sum.push(carry);
    }
    // let sumrev: Vec<u8> = sum.iter().rev().collect();
    sum.reverse();
    return sum;
}

fn multiply_2_vec(one_vec: Vec<u8>, two_vec: Vec<u8>) -> Vec<u8> {

    // println!("{:?}\n{:?}", one_vec, two_vec);
    let mut product: Vec<u8> = Vec::new();
    let mut offset: u64;

    for i in 0..two_vec.len() {
        offset = i as u64;
        // println!("{}", two_vec[two_vec.len() - (i + 1)]);
        for j in 0..one_vec.len() {
            // println!("{}", one_vec[one_vec.len() - (j + 1)]);
            let temp = two_vec[two_vec.len() - (i + 1)] as u64 * one_vec[one_vec.len() - (j + 1)] as u64;
            // println!("TT {}", temp);
            let mut temp_vec = u64_to_vec(temp);
            // println!("TV {:?}\n", temp_vec);
            for _o in 0..offset {
                temp_vec.push(0);
            }
            // println!("{:?}\n", temp_vec);
            product = adding_2_vec(product, temp_vec);
            offset += 1;
        }
    }
    return product;
}


fn calculate_power(base: u64, power: u64) -> Vec<u8> {
    let base_vec: Vec<u8> = u64_to_vec(base);
    let mut product_vec: Vec<u8> = base_vec.clone();
    for _i in 1..power {
        
        product_vec = multiply_2_vec(product_vec, base_vec.clone());
        // println!("-> -> {:?}", product_vec);
    }

    return product_vec;
}


pub fn main() {
    
    // println!("{:?}", calculate_power(2, 11));
    // println!("{:?}", u64_to_vec(1203));

    let mut highest_digit: u64 = 0;
    for a in 1..100 {
        println!("{}", a);
        if a % 10 != 0 {
            for b in 80..100 {

                let result: Vec<u8> = calculate_power(a, b);
                let digits = count_digits(result.clone());

                if digits > highest_digit {
                    highest_digit = digits;
                }

            }
        }
    }
    println!("Result: {}", highest_digit);
}


fn count_digits(number_vec: Vec<u8>) -> u64 {
    let mut digits: u64 = 0;
    for i in 0..number_vec.len() {
        digits += number_vec[number_vec.len() - (i + 1)] as u64;
    }
    return digits;
}