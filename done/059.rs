use std::fs;

pub fn main() {
    let contents: String = fs::read_to_string("resources/059_cipher.txt").expect("Should have been able to read the file");

    let mut matrix: Vec<Vec<u8>> = Vec::new();
    for _i in 0..3 {
        matrix.push(Vec::new());
    }
    let mut c: usize = 0;
    for num in contents.split(",") {
        matrix[c].push(num.parse::<u8>().unwrap());
        if c < 2 {
            c += 1;
        }else {
            c = 0;
        }
    }
    // a - z as ascii
    let lowercase_letters: Vec<u8> = vec![97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122];
    let mut xors: Vec<u8> = vec![0,0,0];

    for k in 0..3 {
        let mut dist: Vec<u16> = vec![0; 200];
        for j in 0..lowercase_letters.len() {
            let mut temp: u16 = 0;
            for i in 0..matrix[k].len() {
                let t = matrix[k][i]^lowercase_letters[j];
                if t == 101 { // get most probable option via most common letter "e"
                    temp += 1;
                }
            }
            dist[lowercase_letters[j] as usize] = temp;
        }
        xors[k] = dist.iter().position(|&r| r == *dist.iter().max().unwrap()).unwrap() as u8;
    }
    let mut cnt: u64 = 0;
    for i in 0..3 {
        for j in 0..matrix[i].len() {
            cnt += (matrix[i][j]^xors[i]) as u64;
        }
    }
    println!("{}", cnt);    
}