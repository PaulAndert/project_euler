fn check_prime(mut number: i64) -> bool {
    number = number.abs();
    if number <= 3 {
        return number > 1;
    }
    if number % 2 == 0 || number % 3 == 0 {
        return false;
    }
    let limit: i64 = (number as f64).powf(0.5) as i64;

    for i in (5..limit+1).step_by(6){
        if number % i == 0 || number % (i+2) == 0{
            return false;
        }
    }
    true
}

fn get_size(mut number: i64) -> usize {
    let mut cnt: usize = 0;

    while number > 0 {
        number /= 10;
        cnt += 1;
    }

    return cnt;
}

fn get_rotations(mut number: i64) -> Option<Vec<i64>> {
    let mut ret: Vec<i64> = Vec::new();

    let size = get_size(number);

    for _rot in 0..size {
        let leading_number: i64 = number / i64::pow(10, get_size(number) as u32 - 1) % 10;
        number = (number - (leading_number * i64::pow(10, get_size(number) as u32 - 1))) * 10 + leading_number;
        
        if !ret.contains(&number) {
            if size == get_size(number) {
                if check_prime(number) {
                    ret.push(number)
                }else {
                    return None;
                }
            }else {
                return None;
            }
        }
    }

    return Some(ret);
}

pub fn main() {

    let mut rotations: i64 = 0;
    let mut nums: Vec<i64> = Vec::new();

    for i in 1..1000000 {
        if check_prime(i) {
            nums.push(i);
        }
    }

    while nums.len() >= 1 {
        let num: i64 = nums[0];
        match get_rotations(num) {
            None => { nums.remove(0); },
            Some(all_rotations) => {
                rotations += all_rotations.len() as i64;
                for rot in all_rotations {
                    nums.retain(|x| *x != rot);
                }
            }
        }    
    }

    println!("Cnt: {}", rotations);
}