

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

pub fn main() {

    let mut checked_primes: Vec<i64> = Vec::new();
    let mut b_primes: Vec<i64> = Vec::new();
    let mut a_primes: Vec<i64> = Vec::new();

    for i in -1000..1001 {
        if check_prime(i) {
            b_primes.push(i);
        }
    }
    for i in -999..1000 {
        for b in b_primes.clone() {
            if check_prime(i + b - 1) {
                a_primes.push(i);
                break;
            }
        }
    }

    let mut best_a: i64 = 0;
    let mut best_b: i64 = 0;
    let mut best_series: i64 = 0;

    for a in a_primes.clone() {
        for b in b_primes.clone() {
            let mut longest: i64 = 0;

            for n in 0..100 {
                let result: i64 = (n as i64).pow(2) + (a as i64) * (n as i64) + (b as i64);
                if checked_primes.contains(&result){
                    longest += 1;
                }else if check_prime(result) {
                    checked_primes.push(result);
                    longest += 1;
                }else{
                    break;
                }
            }
            if longest > best_series {
                best_a = a;
                best_b = b;
                best_series = longest;
            }

        }
    }
    println!("a: {}, b: {} give a series of {}\nboth multiplied {}", best_a, best_b, best_series, best_a*best_b);
}