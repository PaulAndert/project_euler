use std::collections::HashMap;
use itertools::Itertools;
use std::time::Instant;

pub fn main() {
    let start = Instant::now();


    // n^3 - n
    // -> precompute the hashset with valid numbers for n < 10.000
    let mut known_cubes: HashMap<u64, u64> = HashMap::new();
    let mut all_results: Vec<u64> = Vec::new();
    let mut results: Vec<u64>;

    let mut two: usize = 0;
    let mut three: usize = 0;
    let mut four: usize = 0;

    for index in 10u64..=10000 {
        let cubed: u64 = index.pow(3);
        known_cubes.insert(cubed, index);
    }

    println!("Precomputed: {:.2?}", start.elapsed());

    for index in 10u64..=500 {
        if all_results.contains(&index) {
            continue;
        }   
        (known_cubes, results) = check_permutations(known_cubes, index);
        if results.len() == 2 {
            two += 1;
        } else if results.len() == 3 {
            three += 1;
        } else if results.len() == 4 {
            four += 1;
        } else if results.len() == 5 {
            println!("FINISHED {:?}", results);
            break;
        }

        for i in results {
            if !all_results.contains(&i) {
                all_results.push(i);
            }
        }
    }


    println!("2: {}", two);
    println!("3: {}", three);
    println!("4: {}", four);
}


fn check_permutations(known_cubes: HashMap<u64, u64>, index: u64) -> (HashMap<u64, u64>, Vec<u64>){
    let cubed: u64 = index.pow(3);
    let mut results: Vec<u64> = Vec::new();

    for permut in get_permutations(cubed) {
        match known_cubes.get(&permut) {
            Some(known_base) => {
                if known_base > &0u64 {
                    results.push(*known_base);
                }
            },
            None => {}
        }
    }
    return (known_cubes, results);
}

fn get_permutations(num: u64) -> Vec<u64> {
    let options: Vec<u64> = get_options(num);
    let mut ret: Vec<u64> = Vec::new();

    for perm in options.iter().permutations(options.len()).unique() {
        if *perm[0] != 0 {
            let perm_u64: u64 = vec_to_u64(perm);
            ret.push(perm_u64);
        }
    }

    return ret;
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
    return val;
}
    


fn cbrt(n: u64) -> u64 {
    if n == 0 { return 0; }

    let mut low = 0u64;
    let mut high = n;
    let mut ans = 0;

    while low <= high {
        let mid = low + (high - low) / 2;
        let mid_cubed = mid.saturating_mul(mid).saturating_mul(mid); // schützt vor Overflow

        if mid_cubed == n {
            return mid;
        } else if mid_cubed < n {
            ans = mid;
            low = mid + 1;
        } else {
            high = mid - 1;
        }
    }

    ans
}

fn is_perfect_cube(n: u64) -> bool {
    if n == 0 || n == 1 {
        return true;
    }

    let mut low = 1u64;
    let mut high = n;

    while low <= high {
        let mid = low + (high - low) / 2;
        let mid_cubed = mid.saturating_mul(mid).saturating_mul(mid);

        if mid_cubed == n {
            return true; // n ist eine perfekte Kubikzahl
        } else if mid_cubed < n {
            low = mid + 1;
        } else {
            high = mid - 1;
        }
    }

    false
}