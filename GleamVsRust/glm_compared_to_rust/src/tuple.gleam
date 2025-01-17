import gleam/io

pub fn main () {
  let tuple = #("hello", 2)
  io.debug(tuple.0)
  io.debug(tuple.1)
}
