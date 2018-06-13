use std::time::Instant;

pub mod problems;

fn main() {
    let now = Instant::now();
    println!("Answer: {}", problems::problem006::solve());
    println!("{:.3}ms", now.elapsed().subsec_nanos() as f64 / 1000000.0);
}
