
fn check_right_triangle(a: u64, b: u64, c: u64) -> bool {
    let mut rest: Vec<u64> = vec![a,b,c];
    rest.sort();    

    let sides: u64 = u64::pow(rest[0], 2) + u64::pow(rest[1], 2);
    
    if sides == u64::pow(rest[2],2) {
        return true;
    }else {
        return false;
    }
}

fn get_solutions(num: u64) -> u64 {
    let mut solutions: u64 = 0;

    for a in 1..num {
        for b in 1..(num-a) {
            if check_right_triangle(a,b,num-(a+b)) {
                solutions += 1;
            }
        }
    }
    return solutions;
}

pub fn main() {
    let mut biggest_solution: u64 = 0;
    let mut biggest_num: u64 = 0;

    for num in 10..=1000 {
        let solutions: u64 = get_solutions(num);
        if solutions > biggest_solution {
            biggest_solution = solutions;
            biggest_num = num;
        }
    }

    println!("{} with {} Solutions", biggest_num, biggest_solution);
}