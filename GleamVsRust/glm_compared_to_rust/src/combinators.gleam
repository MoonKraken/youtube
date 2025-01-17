import gleam/list
import gleam/io

pub fn main() {
  let nums = ["one", "two", "three"]
  let excited = list.map(nums, fn(element) {element <> "!"})
  io.debug(excited)
}
