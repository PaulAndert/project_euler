
fn get_len(mut number: u64) -> u8 {
    let mut ret: u8 = 0;

    for _ in 0..100 {
        if number >= 10 {
            ret += 1;
            number /= 10;
        }else{
            ret += 1;
            return ret;
        }
    }
    return ret;
}

fn check_pandigital(mut a: u64, mut b: u64, mut c: u64) -> bool {
    let mut check: Vec<bool> = vec![false, false, false, false, false, false, false, false, false, false];

    for _ in 0..10 {
        if a > 0 {
            let val: usize = (a % 10).try_into().unwrap();
            if !check[val] {
                check[val] = true;
                a /= 10;
            }else {
                return false;
            }
        }
        if b > 0 {
            let val: usize = (b % 10).try_into().unwrap();
            if !check[val] {
                check[val] = true;
                b /= 10;
            }else {
                return false;
            }
        }
        if c > 0 {
            let val: usize = (c % 10).try_into().unwrap();
            if !check[val] {
                check[val] = true;
                c /= 10;
            }else {
                return false;
            }
        }
    }  

    let mut cnt: u8 = 0;
    for i in 1..check.len() {
        if check[i] {
            cnt += 1;
        }
    }

    if cnt == 9 {
        true
    }else {
        false
    }
}

pub fn main() {
    // a x b = c
    let mut cnt: u64 = 0;
    let mut pans: Vec<u64> = Vec::new();

    for a in 1..5000 {
        for b in 1..5000 {
            let a_len: u8 = get_len(a);
            let b_len: u8 = get_len(b);
            let c = a * b;
            if a_len + b_len + get_len(c) == 9 {
                if check_pandigital(a, b, c) {
                    println!("{} x {} = {}", a, b, c);
                    if !pans.contains(&c) {
                        pans.push(c);
                        cnt += 1;
                    }
                }
            }
        }
    }
    let mut sum: u64 = 0;
    for i in 0..pans.len() {
        sum += pans[i];
    }
    println!("cnt: {} ==> {}", cnt, sum);

    //println!("{}", check_pandigital(132, 456, 789));
}