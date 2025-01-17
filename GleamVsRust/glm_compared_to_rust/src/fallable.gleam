import gleam/io

pub fn main() {
    let res = do_something_fallable(4)
    io.debug(res)
}

pub fn do_something_fallable(input: Int) -> Result(Int, String) {
  case input {
    x if x > 3 -> Ok(input)
    _ -> Error("number too small")
  }
}
