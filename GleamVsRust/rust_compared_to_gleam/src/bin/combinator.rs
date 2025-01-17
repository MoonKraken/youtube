pub fn main() {
    let nums = vec!["one", "two", "three"];
    let excited: Vec<String> = nums
        .into_iter()
        .map(|element| format!("{}{}", element, "!"))
        .collect();

    dbg!(excited);
}
