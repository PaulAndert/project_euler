
pub fn main() {

    let mut count: u64 = 0;
    let mut matrix: Vec<Vec<u128>> = Vec::new();

    for n in 0..=100 {
        let mut temp_vec: Vec<u128> = Vec::new();
        for r in 0..=n {
            if r == 0 || r == n {
                temp_vec.push(1);
            }else {
                let val: u128 = matrix[n-1][r-1] + matrix[n-1][r];
                temp_vec.push(val);
                if val > 1000000 {
                    count += 1;
                }
            }
        }
        matrix.push(temp_vec);
    }
    println!("Result: {}", count);
}