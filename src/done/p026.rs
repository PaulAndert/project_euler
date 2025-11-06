type F = fraction::Fraction;

fn other_methode(stri: String, start_diff: usize) -> Vec<char> {
    let mut current: Vec<char> = stri.chars().collect();
    current.remove(0);
    current.remove(0);
    let mut longest_overall: Vec<char> = Vec::new();

    let mut a: usize = 0;
    let mut b: usize = start_diff;

    for k in 0..10 {
        a = a+k;
        for i in 0..current.len()/2 {
            if i+a < current.len() && i+b < current.len() && current[i+a] == current[i+b] {
                let mut cnt = 0;
                for j in 0..(b-a) {
                    if i+a+j < current.len() && i+b+j < current.len() && current[i+a+j] == current[i+b+j] {
                        cnt += 1;
                    }
                }
                if cnt == (b-a) {
                    longest_overall = current.drain(a..b).collect();
                }
            }else {
                if (b+1) < (current.len()/2) {
                    b += 1;
                }else{
                    break;
                }
            }
        }
    }
    return longest_overall
}

pub fn main() {
    
    let mut fractions: Vec<String> = Vec::new();

    for i in 2..1000 { fractions.push(format!("{:.2000}",F::from(1) / F::from(i))); }

    let mut highest_number: u16 = 0;
    let mut highest_value: u16 = 0;

    for i in 0..fractions.len() {
        if fractions[i].len() > 1000 {
            let long: String = other_methode(fractions[i].clone(), 20).into_iter().collect();
            if long.len() as u16 > highest_value {
                highest_value = long.len() as u16;
                highest_number = i as u16;
            }
        }
    }
    println!("1/{} with {} numbers per cycle", highest_number+2, highest_value);
    
}