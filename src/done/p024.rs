

fn permutations(numbers: Vec<u8>, mut counter: u64) -> (Vec<String>, u64) {

    if numbers.len() == 0 {
        counter += 1;
        return (vec![], counter);
    }
    if numbers.len() == 1 {
        counter += 1;
        return (vec![numbers[0].to_string()], counter);
    }

    if counter < 1000000 {
        let mut perm: Vec<String> = Vec::new();

        for i in 0..numbers.len() {
    
            let m: u8 = numbers[i];
    
            let mut other_numbers: Vec<u8> = numbers.clone();
            other_numbers.remove(i);
    
            let (perm_ret, counter_ret) = permutations(other_numbers, counter);
            counter = counter_ret;
            for p in perm_ret {
                perm.push(format!("{}{}", m, p));
            }
            
        }
        (perm, counter)
    }else{
        (vec![], counter)
    }
}

pub fn main() {
    let numbers: Vec<u8> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

    let (end, _) = permutations(numbers, 0);
    
    println!("{}", end[end.len()-1]);
}
