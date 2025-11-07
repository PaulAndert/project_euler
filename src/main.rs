use std::time::Instant;

// mod done;
mod progress;

// use progress::p065;

fn main() {
    let now = Instant::now();
    // p065::main();
    println!("Elapsed: {:.2?}", now.elapsed());
}
