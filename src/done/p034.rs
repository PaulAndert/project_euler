use std::collections::HashMap;
use factorial::Factorial;

fn get_vec(mut number: u64) -> Vec<u64> {
    let mut num_vec: Vec<u64> = Vec::new();

    for _ in 0..100 {
        if number >= 10 {
            num_vec.push(number % 10);
            number /= 10;
        }else{
            num_vec.push(number);
            break;
        }
    }
    num_vec.reverse();
    num_vec
}

fn check(number: u64, mut factorials: HashMap<u64, u64>) -> (bool, HashMap<u64, u64> ) {
    let num_vec: Vec<u64> = get_vec(number);
    let mut sum: u64 = 0;
    for i in 0..num_vec.len() {
        match factorials.get(&num_vec[i]) {
            None => {
                let fac: u64 = num_vec[i].factorial();
                sum += fac;
                factorials.insert(num_vec[i], fac);
            },
            Some(val) => {
                sum += val;
            },
        }
    }
    if sum == number {
        (true, factorials)
    }else{
        (false, factorials)
    }
}

pub fn main() {
    let mut factorials: HashMap<u64, u64> = HashMap::new();
    let mut curious: Vec<u64> = Vec::new();

    for i in 3..100000 {
        let (boo, new_fact) = check(i, factorials.clone());
        factorials = new_fact;
        if boo {
            curious.push(i);
        }
    }

    println!("{:?}", curious.iter().sum::<u64>());
}