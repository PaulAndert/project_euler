
fn check(zähler: u64, nenner: u64) -> bool {

    let num_one: u64 = zähler / 10 % 10;
    let num_two: u64 = zähler % 10;
    let den_one: u64 = nenner / 10 % 10;
    let den_two: u64 = nenner % 10;

    if num_one != 0 {
        if den_one != 0 && num_one == den_one {
            if num_two as f32 / den_two as f32 == zähler as f32 / nenner as f32 {
                return true;
            }
        }
        if den_two != 0 && num_one == den_two {
            if num_two as f32 / den_one as f32 == zähler as f32 / nenner as f32 {
                return true;
            }
        }
    }

    if num_two != 0 {
        if den_one !=  0 && num_two == den_one {
            if num_one as f32 / den_two as f32 == zähler as f32 / nenner as f32 {
                return true;
            }
        }
        if den_two != 0 && num_two == den_two {
            if num_one as f32 / den_one as f32 == zähler as f32 / nenner as f32 {
                return true;
            }
        }
    }
    return false;
}

pub fn main() {
    let mut numerator: u64 = 1;
    let mut denominator: u64 = 1;

    for nenner in 10..99 {
        for zähler in 10..nenner {
            if check(zähler, nenner) {
                numerator *= zähler;
                denominator *= nenner;
            }
        }
    }
    println!("Denominator: {}", denominator / numerator);
}