
fn u64_to_vec(mut num: u64) -> Vec<u8> {
    
    let mut ret: Vec<u8> = Vec::new();
    while num > 0 {
        ret.push((num % 10) as u8);
        num /= 10;
    }
    ret.reverse();
    return ret;
}

fn check_numerator(mut top: Vec<u8>, bottom: Vec<u8>) -> bool {
    top = adding_2_vec(top, bottom.clone());
    if top.len() > bottom.len() {
        return true;
    }else {
        return false;
    }
}

fn adding_2_vec(one_vec: Vec<u8>, two_vec: Vec<u8>) -> Vec<u8> {

    let mut carry: u8 = 0;
    let mut sum: Vec<u8> = Vec::new();
    let len = if one_vec.len() > two_vec.len() { one_vec.len() } else { two_vec.len()};

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
    sum.reverse();
    return sum;
}

fn multiply_2_vec(one_vec: Vec<u8>, two_vec: Vec<u8>) -> Vec<u8> {

    let mut product: Vec<u8> = Vec::new();
    let mut offset: u64;

    for i in 0..two_vec.len() {
        offset = i as u64;
        for j in 0..one_vec.len() {
            let temp = two_vec[two_vec.len() - (i + 1)] as u64 * one_vec[one_vec.len() - (j + 1)] as u64;
            let mut temp_vec = u64_to_vec(temp);
            for _o in 0..offset {
                temp_vec.push(0);
            }
            product = adding_2_vec(product, temp_vec);
            offset += 1;
        }
    }
    return product;
}

pub fn main() {
    let mut top: Vec<u8> = vec![1];
    let mut bottom: Vec<u8> = vec![2];
    let mut cnt: u64 = 0;

    for i in 0..=1000 {
        let temp = adding_2_vec(top, multiply_2_vec(bottom.clone(), vec![2]));
        top = bottom.clone();
        bottom = temp;
        if check_numerator(top.clone(), bottom.clone()) {
            cnt += 1;
        }
    }
    println!("{}", cnt);
}