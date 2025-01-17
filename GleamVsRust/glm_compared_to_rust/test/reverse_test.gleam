import gleeunit
import gleeunit/should
import list_destructure

pub fn main() {
    gleeunit.main()
}

pub fn should_reverse_list_test() {
  list_destructure.reverse([1,2,3])
    |> should.equal([3,2,1])
}
