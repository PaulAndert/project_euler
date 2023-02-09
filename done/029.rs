use num_bigint::BigUint;

pub fn main() {

    let mut numbers: Vec<BigUint> = Vec::new();

    for a in 2..=100 {
        for b in 2..=100 {
            let a_big: BigUint = BigUint::new(vec![a]);
            let temp_big: BigUint = a_big.pow(b);
            if !numbers.contains(&temp_big){
                numbers.push(temp_big);
            }
        }
    }

    println!("Count: {}", numbers.len());
}