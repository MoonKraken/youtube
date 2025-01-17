import gleam/int

pub fn do_something(num: Int, message: String) -> String {
  message <> int.to_string(num)
}
