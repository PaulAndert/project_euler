use std::collections::HashMap;

// The only valid Permutations of any n3, are ones that come from other cube roots n, so one first computates a number of n -> n3
// after that you can look up if there are permutations of a given cube in the precomputed hashmap
// to reduce computation you can only look at cubes wih the same length
// to quickly check if two numbers are a permutation of eachother, just count each number occurence and then compare the findings

// Elapsed: 13.15s

pub fn main() {
    // n^3 - n
    let mut known_cubes: HashMap<u64, u64> = HashMap::new();
    let mut matched_n_perfect_cubes: Vec<u64> = Vec::new();

    for input_n in 10u64..=10_000 {
        let input_n3: u64 = input_n.pow(3);
        known_cubes.insert(input_n3, input_n);
    }

    for input_n in 10u64..=10_000 {
        if matched_n_perfect_cubes.contains(&input_n) {
            continue;
        }
            
        let input_n3: u64 = input_n.pow(3);
        let mut results: Vec<u64> = Vec::new();

        let counter = get_number_length(input_n3);
        let lower_range_of_length = 10u64.pow((counter - 1) as u32);
        let upper_range_of_length = 10u64.pow(counter as u32);
        
        let cubes_with_same_length: HashMap<u64, u64> = known_cubes
            .iter()
            .filter(|(&k, _)| k >= lower_range_of_length && k < upper_range_of_length)
            .map(|(&k, &v)| (k, v))
            .collect();
        
        for (output_n3, output_n) in cubes_with_same_length {
            if is_permutation(input_n3, output_n3) {
                results.push(output_n);
            }
        }

        if results.len() == 5 {
            results.sort();
            match results.get(0) {
                Some(min) => { println!("FINISHED {:?} -> {}", results, min.pow(3)); },
                None => { println!("Error in {:?}", results); }
            }
            return;
        }

        for i in results {
            if !matched_n_perfect_cubes.contains(&i) {
                matched_n_perfect_cubes.push(i);
            }
        }
    }
}


fn is_permutation(n3_01: u64, n3_02: u64) -> bool {

    let mut numbers_01: Vec<usize> = vec![0;10];
    let mut numbers_02: Vec<usize> = vec![0;10];

    let options_01: Vec<usize> = get_options(n3_01);
    let options_02: Vec<usize> = get_options(n3_02);

    if options_01.len() != options_02.len() {
        return false;
    }

    for opt in options_01 {
        numbers_01[opt] += 1;
    }

    for opt in options_02 {
        if numbers_01[opt] == 0 {
            return false;
        }
        numbers_02[opt] += 1;
    }

    for i in 0..10 {
        if numbers_01.get(i) != numbers_02.get(i) {
            return false;
        }
    }
    return true;
}

fn get_options(mut num: u64) -> Vec<usize> {
    let mut ret: Vec<usize> = Vec::new();
    while num > 0 {
        ret.push((num % 10) as usize);
        num /= 10;
    }
    return ret;
}

fn get_number_length(num: u64) -> usize {
    if num == 0 {
        return 1;
    } else {
        return num.ilog10() as usize + 1;
    }
}

