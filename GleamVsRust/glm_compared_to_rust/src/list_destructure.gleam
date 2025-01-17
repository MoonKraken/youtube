import gleam/list

pub fn reverse(l: List(Int)) -> List(Int) {
  case l {
    [] -> []
    [first, ..rest] -> list.concat([reverse(rest), [first]])
  }
}
