
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

fn check_prime_sum(primes: Vec<u64>, num: u64) -> u64 {
    let mut start: usize = 0;
    let mut sum: u64 = 0;
    let mut cnt: u64 = 0;

    for p in primes.clone() {
        if sum == num {
            return cnt;
        }else if sum < num {
            cnt += 1;
            sum += p;
        }else if sum > num {
            sum -= primes[start];
            cnt -= 1;
            start += 1;
        }
        if (start != 0 && sum <= 0) || (start != 0 && cnt == 0) {
            return 0;
        }
    }
    0
}

pub fn main() {

    let mut primes: Vec<u64> = Vec::new();
    let limit: u64 = 1000000;
    for i in 0..limit {
        if check_prime(i) {
            primes.push(i);
        }
    }

    let mut biggest_prime: u64 = 0;
    let mut biggest_count: u64 = 0;

    for p in primes.clone() {
        let temp: u64 = check_prime_sum(primes.clone(), p);
        if temp > biggest_count {
            biggest_count = temp;
            biggest_prime = p;
        }
    }
    println!("Result: {}", biggest_prime);
}