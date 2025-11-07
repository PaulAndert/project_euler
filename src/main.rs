use std::time::Instant;

// mod done;
mod progress;

use progress::p062;

fn main() {
    let now = Instant::now();
    p062::main();
    println!("Elapsed: {:.2?}", now.elapsed());
}
