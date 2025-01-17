import gleam/io

pub fn main() {
  // No labels
  io.debug(add(1, 2))

  // Labelled arguments
  io.debug(add(a: 1, b: 2))
}

fn add(a internal_a: Int, b internal_b: Int) -> Int {
  internal_a + internal_b
}
