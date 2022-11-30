use std::collections::HashSet;

macro_rules! nothing_burger {
    () => {};
}

macro_rules! hello_ferris {
    // this branch third
    () => {
        println!("Where's Ferris?")
    };

    //this branch first
    ("Ferris") => {
        println!("Hello, Ferris");
    };

    //this branch second
    ("Bob") => {
        println!("Hello, Bob");
    };
}

macro_rules! hello_x {
    //this branch second
    ("Charlie") => { println!("Just for Charlie.") };

    // fragment examples
    //implement this branch first
    ($s:literal) => { println!("Hello, {}", $s) };

    // this branch second
    ($s:ident) => { println!("Hello, {}", $s) };

    // then replace both of them with this branch
    ($s:expr) => { println!("Hello, {}", $s) };
}

macro_rules! set {
    ($($l:expr) *) => {
        HashSet::from([$($l,)*])
    };
}

fn main() {
    //apsiration
    let some_set = HashSet::from([1,2,3]);
    let some_set = set!(1 2 3);
    dbg!(some_set);

    //match
    let thing = 5;
    match thing {
        a => println!("a arm"),
        5 => println!("5 arm"),
    }
    //0
    nothing_burger!();

    //1
    hello_ferris!("Ferris");
    hello_ferris!("Bob");
    hello_ferris!();

    //2
    let a = "Megan";
    hello_x!("Alice");
    hello_x!("Charlie");
    hello_x!(a);

    //3
    let set = set!(1 2 3);
    dbg!(set);

    let num = 4;
    let num2 = 5;
    let set = set!(num2 num);
    dbg!(set);

    let set = set!(num 10 num2);
    dbg!(set);
}

