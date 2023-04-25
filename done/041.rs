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

fn vec_to_u64(perm: Vec<&u64>) -> u64 {
    let mut val: u64 = 0;
    for i in 0..perm.len() {
        val += perm[perm.len() - (i + 1)] * u64::pow(10, i as u32);
    }
    val
}

fn get_pandigital_prime() -> u64 {

    let mut options: Vec<u64> = vec![9,8,7,6,5,4,3,2,1];
    for i in 0..9 {
        for perm in options.iter().permutations(9 - i) {
            let temp = vec_to_u64(perm);
            if check_prime(temp) {
                return temp;
            }
        }
        options.remove(0);
    }
    return 0;
}

pub fn main() {
    let mut options: Vec<u64> = vec![9,8,7,6,5,4,3,2,1];

    for i in 0..9 {
        for perm in options.iter().permutations(9 - i) {
            let temp = vec_to_u64(perm);
            if check_prime(temp) {
                println!("{}", temp);
                return ();
            }
        }
        options.remove(0);
    }
}