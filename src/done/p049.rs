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

fn get_options(mut num: u64) -> Vec<u64> {
    let mut ret: Vec<u64> = Vec::new();
    while num > 0 {
        ret.push(num % 10);
        num /= 10;
    }
    return ret;
}

fn vec_to_u64(perm: Vec<&u64>) -> u64 {
    let mut val: u64 = 0;
    for i in 0..perm.len() {
        val += perm[perm.len() - (i + 1)] * u64::pow(10, i as u32);
    }
    val
}

fn get_permutations(num: u64) -> Vec<u64> {
    let options: Vec<u64> = get_options(num);
    let mut ret: Vec<u64> = Vec::new();

    for perm in options.iter().permutations(4).unique() {
        let perm_u64: u64 = vec_to_u64(perm);
        if perm_u64 >= 1000 && check_prime(perm_u64) {
            ret.push(perm_u64);
        }
    }

    return ret;
}

fn get_distance(mut perms: Vec<u64>) -> Option<Vec<u64>> {
    perms.sort();

    for a in perms.clone() {
        for b in perms.clone() {
            let distance = match a < b {
                true => {b - a},
                false => {a - b},
            };
            if distance != 0 {
                for c in perms.clone() {
                    match a < b {
                        true => {
                            if c > distance && c - distance == b {
                                return Some(vec![a,b,c]);
                            }
                        },
                        false => {
                            if c > distance && c - distance == a {
                                return Some(vec![b,a,c]);
                            }
                        },
                    };
                }
            }
        }
    }
    None
}

pub fn main() {
    for i in 1000..10000 {
        if check_prime(i) {
            let perms: Vec<u64> = get_permutations(i);
            match get_distance(perms) {
                None => {},
                Some(perm_vec) => {
                    if perm_vec[0] != 1487 { // not the example
                        println!("Result: {}{}{}", perm_vec[0], perm_vec[1], perm_vec[2]);
                        return ();
                    }
                }
            }
        }
    }
}