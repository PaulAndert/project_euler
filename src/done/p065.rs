// First you need to compute the 100 fraction values a
// Then you can add these in reverse order in the folloing format: new_nominator = a * old_denominator + old_nominator
// Because this adds the value of a to the old value to get the new value
// After that you need to compute 1 / new value to complete one whole step, this is done by switching denominator and nominator
// After doind this 100x, one needs to add the value 2 (a0) to the number to get the final number

// Elapsed: 577.50µs

pub fn main() {
    let mut numbers: Vec<u64> = Vec::new();

    for i in 1..=100 {
        if i % 3 == 2 {
            numbers.push(2 * ((i + 1) / 3));
        } else {
            numbers.push(1);
        }
    }

    let mut numerator: Vec<u8> = vec![0];
    let mut denominator: Vec<u8> = vec![1];

    for i in (0..99).rev() {
        match numbers.get(i) {
            Some(a) => {
                // x + an
                numerator = vec_add(numerator, vec_mul(*a, denominator.clone()));
                
                // 1 / x
                let temp = denominator;
                denominator = numerator;
                numerator = temp;
            },
            None => { println!("Error on {}", i) }
        }
    }
    let result = vec_add(numerator.clone(), vec_mul(2, denominator.clone()));
    println!("R {}", result.iter().map(|v| *v as u64).sum::<u64>());
}

fn vec_add(mut summand_01: Vec<u8>, mut summand_02: Vec<u8>) -> Vec<u8> {
    summand_01.reverse();
    summand_02.reverse();
    
    let mut carry: u64 = 0;
    let mut vector: Vec<u8> = Vec::new();

    let limit = if summand_01.len() > summand_02.len() {
        summand_01.len()
    } else {
        summand_02.len()
    };

    for index in 0..limit {
        let a = if index < summand_01.len() {
            summand_01[index]
        } else {
            0
        };

        let b = if index < summand_02.len() {
            summand_02[index]
        } else {
            0
        };

        let sum = a as u64+ b as u64 + carry;
        vector.push((sum % 10) as u8);
        carry = sum as u64/ 10;
    }
    while carry > 0 {
        vector.push((carry % 10) as u8);
        carry /= 10;
    }

    vector.reverse();
    return vector;
}

fn vec_mul(factor_01: u64, factor_02: Vec<u8>) -> Vec<u8> {
    let mut carry: u64 = 0;
    let mut vector: Vec<u8> = Vec::new();

    for number in factor_02.iter().rev() {
        let product = *number as u64 * factor_01 + carry;
        vector.push((product % 10) as u8);
        carry = product / 10;
    }
    while carry > 0 {
        vector.push((carry % 10) as u8);
        carry /= 10;
    }

    vector.reverse();
    return vector;
}