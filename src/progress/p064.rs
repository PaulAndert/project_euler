// First you need to compute the quadratic numbers until 10_000 to later excluded them
// Then you can check for ever number <= 10_000 except those
// For every number you can get the next chain-divider-number with this formular: (remainder + whole_root) / numerator
// a_0 is the starting point with the whole numeber of the root of the start number
// then you can calculate the new remainder like that: numerator * a_n - old_remainder
// then you can calculate the new numerator like that: (start - remainder^2) / numerator;

// Elapsed: 32.48ms

pub fn main() {

    let mut quadratic_numbers: Vec<u64> = Vec::new();
    let mut index: u64 = 1;
    loop {
        let qudratic = index * index;
        quadratic_numbers.push(qudratic);
        if qudratic >= 10_000 {
            break;
        }
        index += 1;
    }

    let mut cnt: u64 = 0;
    for i in 1..=10_000 {
        if quadratic_numbers.contains(&i) {
            continue;
        }

        let period: u64 = compute_period(i);
        if period % 2 == 1 {
            cnt += 1
        }
    }

    println!("C: {}", cnt);
}

fn compute_period(start: u64) -> u64 {

    let mut remainder: u64 = 0;
    let mut numerator: u64 = 1;
    let a_0: u64 = f64::sqrt(start as f64) as u64;
    let mut a_n: u64 = a_0;
    let mut period_cnt: u64 = 1;

    loop {
        remainder = numerator * a_n - remainder;
        numerator = (start - u64::pow(remainder, 2)) / numerator;
        a_n = (a_0 + remainder) / numerator;

        if a_n == 2 * a_0 {
            break;
        }
        period_cnt += 1;
    }

    return period_cnt;
}