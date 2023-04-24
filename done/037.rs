fn check_prime(mut number: u64) -> bool {
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

fn get_size(mut number: u64) -> usize {
    let mut cnt: usize = 0;
    while number > 0 {
        number /= 10;
        cnt += 1;
    }
    return cnt;
}

fn check_truncate(num: u64) -> bool {
    for offset in 0..get_size(num) {
        let prime_test = num / u64::pow(10, offset as u32);
        if !check_prime(prime_test) {
            return false;
        }
    }
    for offset in 0..get_size(num) {
        let fac = u64::pow(10, (get_size(num) - offset) as u32);
        let prime_test = num - num / fac * fac;
        if !check_prime(prime_test) {
            return false;
        }
    }

    true
}

pub fn main() {
    let mut cnt: u64 = 0;
    let mut sum: u64 = 0;
    let mut num: u64 = 11;

    while cnt < 11  {
        if check_prime(num) && check_truncate(num) {
            cnt += 1;
            sum += num;
        }
        num += 1;
    }

    println!("{:?}", sum);

}