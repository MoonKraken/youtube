fn to_int(s: &str) -> i32 {
    s.parse().unwrap_or(0)
}

fn sum_str_vec(strs: Vec<String>) -> String {
    let mut accum = 0i32;
    for s in strs {
        accum += to_int(&s);
    }

    accum.to_string()
}

fn main() {
    let v = vec![String::from("3"), String::from("4")];
    let total = sum_str_vec(v);
    println!("{:?}", total);

    let v = vec![String::from("3"), String::from("abc")];
    let total = sum_str_vec(v);
    println!("{:?}", total);
}
