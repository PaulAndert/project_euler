

pub fn main() {
    let mut res: u64 = 0;
    for i in 1..=1000 {
        let mut temp: u64 = 1;
        for _j in 0..i {
            temp *= i;
            temp = temp % 10000000000;
        }
        res += temp;
        res = res % 10000000000;
    }
    println!("Result: {}", res);
}