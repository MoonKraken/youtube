#[derive(Debug)]
struct SummationError;

fn to_int(s: &str) -> Option<i32> {
    s.parse().ok()
}

fn sum_str_vec(strs: Vec<String>) -> Result<String, SummationError> {
    let mut accum = 0i32;
    for s in strs {
        accum += to_int(&s).ok_or(SummationError)?;
    }

    Ok(accum.to_string())
}

fn main() {
    let v = vec![String::from("3"), String::from("4")];
    let total = sum_str_vec(v);
    println!("{:?}", total);

    let v = vec![String::from("3"), String::from("abc")];
    let total = sum_str_vec(v);
    println!("{:?}", total);
}
