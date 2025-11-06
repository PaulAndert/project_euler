

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

fn check_goldbach_conjecture(primes: Vec<u64>, num: u64) -> bool {
    let mut i: usize = 0;

    while primes[i] < num {
        let mut rest: u64 = num - primes[i];
        if rest % 2 == 0 {
            rest /= 2;

            let mut j: u64 = 2;
            let mut square: u64 = 1;
            while square <= rest {

                if square == rest {
                    return true;
                }else {
                    square = u64::pow(j, 2);
                    j += 1;
                }
            }
        }
        i += 1;
    }

    return false;
}
pub fn main() {

    let mut primes: Vec<u64> = Vec::new();

    for i in 0..10000 {
        if check_prime(i) {
            primes.push(i);
        }
    }

    for i in (3..10000).step_by(2) {
        match primes.binary_search(&i) {
            Ok(_) => {},
            Err(_) => {
                if !check_goldbach_conjecture(primes.clone(), i) {
                    println!("Result: {}", i);
                    return ();
                }
            },
        }
    }
}