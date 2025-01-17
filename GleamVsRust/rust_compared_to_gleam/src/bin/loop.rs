pub fn main() {
    let nums = vec!["one", "two", "three"];

    let mut excited: Vec<String> = Vec::new();
    for num in nums {
        excited.push(format!("{}{}", num, "!"));
    }

    dbg!(excited);
}
