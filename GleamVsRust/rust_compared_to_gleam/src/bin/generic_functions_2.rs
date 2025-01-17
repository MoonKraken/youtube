use std::fmt::Debug;

pub fn generic_function<T>(a: T, b: T) -> T
where
    T: Debug,
{
  dbg!(&a);
  dbg!(b);

  a
}
