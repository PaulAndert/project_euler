

pub fn main() {
    let mut known_cubes: Vec<u64> = Vec::new();

    for index in 1u64..=500 {
        let cubed: u64 = index.pow(3);
        println!("{} => {}", index, cubed);


        check_permutations(known_cubes.clone(), cubed);

    }
}


fn check_permutations(known_cubes: Vec<u64>, cubed: u64) {

}