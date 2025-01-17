use std::fmt::Debug;

pub fn generic_function<A, B>(a: A, b: B) -> A
where
    A: Debug,
    B: Debug,
{
  dbg!(&a);
  dbg!(b);

  a
}
