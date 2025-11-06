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

pub fn main() {
    let mut current: u64 = 1;
    let mut step: u64 = 2;

    let mut prime_count: u64 = 0;
    let mut number_count: u64 = 1;

    loop {
        for j in 0..4 {
            current += step;
            if check_prime(current) {
                prime_count += 1;
            }
            number_count += 1;
        }
        if (prime_count as f64) / (number_count as f64) < 0.1 {
            println!("{}", step + 1);
            break;
        }
        step += 2;
    }
}