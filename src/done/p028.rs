

pub fn main() {

    let mut number: u64 = 1;

    for i in (3..=1001).step_by(2) {
        println!("{}", i);
        // top right
        number += (i as u64).pow(2);

        // bottom right
        number += ((i - 2) as u64).pow(2) + (i - 1);

        // top left
        let top_left: u64 = (i as u64).pow(2) - (i - 1);
        number += top_left;

        // bottom left
        number += top_left - (i - 1);
    }

    println!("End: {}", number);
}