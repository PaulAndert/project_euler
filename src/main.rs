use std::time::Instant;

mod foo {
    include!("044.rs");
}

fn main() {
    let now = Instant::now();
    foo::main();
    println!("Elapsed: {:.2?}", now.elapsed());
}
