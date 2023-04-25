

fn calculate_number(mut num: u64 ) -> u64 {
    let mut cnt = 1;
    let mut loops = 0;
    let mut barrier = 9;
    let mut test = 1;
    while num > 0 {
        if cnt > num {
            break;
        }
        num -= cnt;
        loops += 1;
        test += 1;

        if loops >= barrier {
            barrier *= 10;
            cnt += 1;
            loops = 0;
        }

    }
    if num == 0 {
        return (test - 1) % 10;
    }
    if num > 0 {
        return (test / u64::pow(10, (cnt - num) as u32)) % 10;
    }else {
        println!("ERR");
        return 0;
    }

}

pub fn main() {

    let result: u64 = calculate_number(1) * calculate_number(10) * calculate_number(100) * calculate_number(1000) * calculate_number(10000) * calculate_number(100000) * calculate_number(1000000);

    println!("{}", result);
}