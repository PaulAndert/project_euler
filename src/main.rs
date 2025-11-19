use std::time::Instant;

// mod done;
mod progress;

use progress::p064;

fn main() {
    let now = Instant::now();
    p064::main();
    println!("Elapsed: {:.2?}", now.elapsed());
}
