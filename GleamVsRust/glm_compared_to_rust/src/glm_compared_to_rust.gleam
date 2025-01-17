import gleam/io

pub fn main() {
  do_something(1, 2)
  do_something("hello", "world")
}

fn do_something(a, b) {
  io.debug(a)
  io.debug(b)
}
