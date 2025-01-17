import gleam/int

pub fn do_something(num, message) {
  message <> int.to_string(num)
}
