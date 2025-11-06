use itertools::Itertools;

fn vec_to_u64(perm: Vec<&u64>) -> u64 {
    let mut val: u64 = 0;
    for i in 0..perm.len() {
        val += perm[perm.len() - (i + 1)] * u64::pow(10, i as u32);
    }
    val
}

fn check_divisibility_property(perm: Vec<&u64>) -> bool {

    let divisors: Vec<u64> = vec![2,3,5,7,11,13,17];

    for i in 1..8 {
        let temp = vec_to_u64(vec![perm[i], perm[i+1], perm[i+2]]);
        if temp % divisors[i - 1] as u64 != 0 {
            return false;
        }
    }
    return true;
}

fn get_all_pandigitals() -> u64 {

    let mut cnt: u64 = 0;
    let options: Vec<u64> = vec![9,8,7,6,5,4,3,2,1,0];

    for perm in options.iter().permutations(10) {
        if check_divisibility_property(perm.clone()) {
            cnt += vec_to_u64(perm);
        }
    }
    return cnt;
}

pub fn main() {

    println!("Result: {}", get_all_pandigitals());

}