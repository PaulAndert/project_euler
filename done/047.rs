use itertools::Itertools;

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

fn get_prime_factors(primes: Vec<u64>, mut num: u64) -> Vec<u64> {
    let mut factors: Vec<u64> = Vec::new();

    let mut i: usize = 0;
    while num != 1 {
        if num % primes[i] == 0 {
            factors.push(primes[i]);
            num /= primes[i];
            i = 0;
        }else {
            i += 1;
        }
    }

    return factors;
}

fn get_distinct_string(nums: Vec<u64>) -> (String, usize) {
    let mut ret: String = String::new();
    let mut size: usize = 0;
    for c in nums.iter().unique() {
        size += 1;
        ret = format!("{},{}", ret, c);
    }
    return (ret, size);
}

pub fn main() {
    let mut primes: Vec<u64> = Vec::new();
    let limit: u64 = 200000;
    for i in 0..limit {
        if check_prime(i) {
            primes.push(i);
        }
    }

    let mut set: Vec<String> = Vec::new();
    let mut result: u64 = 0;

    for i in 1..limit {
        match primes.binary_search(&i) {
            Ok(_) => {},
            Err(_) => {
                if !(result == 0 || result + 1 == i) { // if there is a gap -> reset
                    result = 0;
                    set = Vec::new();
                }
                let (factors, cnt) = get_distinct_string(get_prime_factors(primes.clone(), i));
                if cnt == 4 {
                    if set.len() == 0 {
                        set.push(factors);
                        result = i;
                    }else {
                        if !set.contains(&factors) {
                            set.push(factors);
                            result += 1;
                        }else {
                            set = Vec::new();
                        }
                    }
                    if set.len() == 4 {
                        println!("Result: {} with {:?}", result - 3, set);
                        return ();
                    }
                }
            },
        }
    }

    println!("{:?}", result);
}