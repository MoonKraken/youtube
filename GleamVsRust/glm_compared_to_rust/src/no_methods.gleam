import gleam/io

pub type Thing {
    Thing (
        a: String,
        b: String,
    )
}

fn combined_fields(thing: Thing) -> String {
  thing.a <> thing.b
}

pub fn main() {

  let thing = Thing(
    a: "hello",
    b: "world"
  )

  io.println(combined_fields(thing))
}
