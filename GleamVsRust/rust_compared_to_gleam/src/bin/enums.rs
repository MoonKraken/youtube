#[derive(Debug)]
enum Thing {
    VariantOne,
    VariantTwo {a: u64, b: String},
    VariantThree(String, String)
}

pub fn main() {
    let a = Thing::VariantTwo {
        a: 64,
        b: "something".to_string()
    };

    dbg!(a);
}
