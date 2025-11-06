

fn generate_pentagonal(num: u64) -> u64 {
    return num * (3 * num - 1) / 2;
}

pub fn main() {

    let mut pentagonals: Vec<u64> = Vec::new();
    let mut d: u64 = u64::MAX;

    for i in 1..5000 {
        pentagonals.push(generate_pentagonal(i));
    }
    for k in 0..pentagonals.len() {
        for j in 0..k {
            let temp = pentagonals[k] + pentagonals[j];
            match pentagonals.binary_search(&temp) {
                Err(_) => {},
                Ok(_) => {
                    let temp2;
                    if pentagonals[k] < pentagonals[j] {
                        temp2 = pentagonals[j] - pentagonals[k];
                    }else {
                        temp2 = pentagonals[k] - pentagonals[j];
                    }
                    match pentagonals.binary_search(&temp2) {
                        Err(_) => {},
                        Ok(_) => {
                            if temp2 < d {
                                d = temp2;
                            }
                        },
                    }
                },
            }  
        }
    }

    println!("D: {}", d);
}