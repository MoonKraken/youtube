import gleam/io

pub type Thing{
    VariantOne
    VariantTwo(a: Int, b: String)
    VariantThree(String, String)
}

pub fn main() {
    let a = VariantTwo (
        a: 64,
        b: "something"
    )

    io.debug(a)
}
