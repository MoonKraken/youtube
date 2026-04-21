use rayon::prelude::*;
use std::time::Instant;

pub fn main() {
    let start = Instant::now();

    let sum: u64 = (0..10_000_000)
        .into_par_iter()
        .map(|i| { 
            i * rand::random_range(0..1000)
        })
        .reduce(|| 0, |a, b| a + b);

    dbg!(start.elapsed());
    dbg!(sum);
}
