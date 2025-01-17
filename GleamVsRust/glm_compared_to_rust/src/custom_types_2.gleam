import gleam/io
import gleam/list

pub type Thing{
    VariantOne(a: Int)
    VariantTwo(a: Int, b: String)
    VariantThree(a: Int, b: String, c: String)
}

pub fn main() {
    let some_list: List(Thing) = [
      VariantOne(a: 1),
      VariantTwo(a: 2, b: "two"),
      VariantThree(a: 3, b: "two", c: "two"),
    ]

    list.each(some_list, fn(item) {io.debug(item.a)})
}
