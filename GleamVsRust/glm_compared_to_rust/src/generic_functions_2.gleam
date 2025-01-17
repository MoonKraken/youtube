import gleam/io

pub fn generic_function(a: foo, b: foo) -> foo {
  io.debug(a)
  io.debug(b)

  a
}
