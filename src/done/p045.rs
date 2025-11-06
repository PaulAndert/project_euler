
fn generate_triangle(num: u64) -> u64 {
    return num * (num + 1) / 2; // n(n+1)/2
}

fn generate_pentagonal(num: u64) -> u64 {
    return num * (3 * num - 1) / 2; // n(3n−1)/2
}

fn generate_hexagonal(num: u64) -> u64 {
    return num * (2 * num - 1); // n(2n−1)
}

pub fn main() {

    let mut hexa: u64 = 6;
    let mut hexa_n: u64 = 2;
    let mut pent: u64 = 5;
    let mut pent_n: u64 = 2;
    let mut tria: u64 = 3;
    let mut tria_n: u64 = 2;

    loop {
        if tria == 40755 { // remove the first hit on 40755
            tria_n += 1;
            tria = generate_triangle(tria_n);
            pent_n += 1;
            pent = generate_pentagonal(pent_n);
            hexa_n += 1;
            hexa = generate_hexagonal(hexa_n);
        }
        
        if tria < pent {
            tria_n += 1;
            tria = generate_triangle(tria_n);
        }else if pent < hexa {
            pent_n += 1;
            pent = generate_pentagonal(pent_n);
        }else if hexa < pent {
            hexa_n += 1;
            hexa = generate_hexagonal(hexa_n);
        }
        
        if tria != 40755 { // exclude the first hit on 40755
            if tria == pent && pent == hexa {
                println!("{}", hexa);
                break;
            }
        }
    }

}