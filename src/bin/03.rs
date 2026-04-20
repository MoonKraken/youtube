use rayon::prelude::*;

pub fn main() {
    let mut nums = vec![1, 2, 3, 4, 5];
    nums.par_iter_mut()
        .for_each(|n| {
            *n *= 3;
        });

    dbg!(&nums);
}
