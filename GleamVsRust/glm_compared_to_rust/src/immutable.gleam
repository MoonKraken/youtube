import gleam/io

pub type Thing {
    Thing (
        a: Int,
        b: String,
    )
}

pub fn main() {
    let thing_one = Thing (
        a: 3,
        b: "something"
    )

    let thing_two = Thing (..thing_one, b: "else")

    io.debug(thing_two)
}
