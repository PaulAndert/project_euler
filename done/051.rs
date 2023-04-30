
fn check_prime(number: u64) -> bool {
    if number <= 3 {
        return number > 1;
    }
    if number % 2 == 0 || number % 3 == 0 {
        return false;
    }
    let limit: u64 = (number as f64).powf(0.5) as u64;
    
    for i in (5..limit+1).step_by(6){
        if number % i == 0 || number % (i+2) == 0{
            return false;
        }
    }
    true
}

fn vec_to_u64(chars: Vec<u64>) -> u64 {
    let mut val: u64 = 0;
    for i in 0..chars.len() {
        val += chars[chars.len() - (i + 1)]* u64::pow(10, i as u32);
    }
    val
}

fn get_size(mut number: u64) -> usize {
    let mut cnt: usize = 0;
    while number > 0 {
        number /= 10;
        cnt += 1;
    }
    return cnt;
}

fn generate_numbers(num: u64) -> Option<u64> {

    let num_str: Vec<u64> = format!("{}", num).chars().map(|c| c as u64 - 48).collect();
    let mut min: u64 = u64::MAX;

    for index_a in 0..num_str.len() {
        for index_b in (index_a)..num_str.len() {
            for index_c in (index_b)..num_str.len() {
                for index_d in (index_c)..num_str.len() {
                    let mut cnt = 0;
                    let mut temp_min = u64::MAX;
                    for new_number in 0..10 {
                        let mut temp: Vec<u64> = num_str.clone();
                        temp[index_a] = new_number;
                        temp[index_b] = new_number;
                        temp[index_c] = new_number;
                        temp[index_d] = new_number;
                        let test = vec_to_u64(temp);
                        if get_size(test) == get_size(num) && check_prime(test) {
                            cnt += 1;
                            if temp_min > test {
                                temp_min = test;
                            }
                        }
                    }
                    if cnt >= 8 {
                        if min > temp_min {
                            min = temp_min;
                        }
                    }
                }
            }
        }   
    }
    if min == u64::MAX {
        return None;
    }else {
        return Some(min);
    }
}

pub fn main() {
    for i in 0..1000000 {
        if check_prime(i) {
            match generate_numbers(i) {
                None => {},
                Some(val) => {
                    println!("Result {}", val);
                    break;
                },
            }
        }
    }
}
