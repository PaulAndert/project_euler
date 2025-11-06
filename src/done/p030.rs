
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

fn check (number: u64) -> bool {
    let num_vec: Vec<u64> = get_vec(number);

    let mut val: u64 = 0;
    for i in 0..num_vec.len() {
        val += num_vec[i].pow(5);
    }
    if val == number {
        true
    }else{
        false
    }
}

pub fn main() {
    let mut res: Vec<u64> = Vec::new();

    for i in 11..1000000 {
        if check(i) {
            res.push(i as u64);
        }
    }

    println!("{:?}", res);
    let mut res_sum: u64 = 0;
    for i in res {
        res_sum += i;
    }
    println!("Sum: {}", res_sum);

}