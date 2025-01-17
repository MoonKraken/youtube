#[derive(Debug)]
struct Thing {
    a: String,
    b: String,
}

impl Thing {
    pub fn combined_fields(&self) -> String {
        format!("{}{}", self.a, self.b)
    }
}

pub fn main() {
    let mut thing = Thing {
        a: "hello".to_string(),
        b: "world".to_string(),
    };

    println!("{}", thing.combined_fields())
}
