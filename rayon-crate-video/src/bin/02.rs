use rayon::prelude::*;
use std::time::Instant;

pub fn main() {
    let start = Instant::now();

    let sum: u64 = (0..25000).into_par_iter()
        .map(|i| { 
            i * rand::random_range(0..1000)
        })
        .sum();

    dbg!(start.elapsed());
    dbg!(sum);
}
