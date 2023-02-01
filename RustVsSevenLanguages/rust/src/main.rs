use std::{io::stdin, str::SplitWhitespace};
fn sum_fn(arr: SplitWhitespace) -> i32 {
    arr.into_iter()
        .map(|elem| elem.parse::<i32>().unwrap())
        .sum::<i32>()
}

fn main() {
    loop {
        let mut buffer = String::new();
        stdin().read_line(&mut buffer).unwrap();
        let splits = buffer.split_whitespace();
        let sum = sum_fn(splits);
        println!("{}", sum);
    }
}
