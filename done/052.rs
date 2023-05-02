
fn u64_to_vec(mut num: u64) -> Vec<u64> {
    let mut ret: Vec<u64> = Vec::new();

    while num > 0 {
        ret.push(num % 10);
        num /= 10;
    }
    ret.sort();
    return ret;
}

fn check_all_numbers(start: Vec<u64>, test: Vec<u64>) -> bool {
    if start.len() == test.len() {
        for i in 0..start.len() {
            if start[i] != test[i] {
                return false;
            }
        }
        return true;
    }else {
        return false;
    }
}

fn check_multiplications(num: u64) -> bool {

    let start: Vec<u64> = u64_to_vec(num);

    for m in 2..=6 {
        let test = num * m; 
        if !check_all_numbers(start.clone(), u64_to_vec(test)) {
            return false;
        }
    }
    return true;
}

pub fn main() {

    for i in 1..1000000 {
        if check_multiplications(i) {
            println!("Result: {}", i);
            break;
        }
    }
}