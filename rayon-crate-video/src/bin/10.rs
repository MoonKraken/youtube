fn main() {
    let (mut a, mut b, mut c) = (0, 0, 0);

    rayon::scope(|s| {
        s.spawn(|_| a += 1);
        s.spawn(|_| b += 2);
        s.spawn(|_| c += 3);
    });

    println!("{a}, {b}, {c}"); 
}
