/*
[5,2,4,3,1]
-> [5,2,4,3,1] compare first element (5)
-> [2,5,4,3,1] compare second element (2)
               less than first element on left so swap
-> [2,4,5,3,1] compare third element (4)
               less then second element on left so swap,
               no less than first element on left so now
               the first 3 elements are sorted
-> [2,4,3,5,1] compare fourth element (4)
   [2,3,4,5,1] continue swapping to the left until no
               longer smaller then next left element
-> [2,3,4,5,1]
   [2,3,4,1,5]
   [2,3,1,4,5]
   [2,1,3,4,5]
   [1,2,3,4,5]
*/

/*
The whole 'std::iter::once(e).chain(xs.iter().copied()).collect()'
is equivalent to:
  {
    let mut v = Vec::new();
    v.push(e);
    v.extend(xs);
    v
  }
*/

use std::marker::Copy;
use std::cmp::Ord;

fn isort<T: Copy + Ord>(ns: &[T]) -> Vec<T> {
  fn compare<T: Copy + Ord>(e: T, xs: &[T]) -> Vec<T> {
    match xs {
      [] =>
        vec![e],
      [head, _tail @ ..] if e <= *head =>
        std::iter::once(e)
          .chain(xs.iter().copied())
          .collect(),
      [head, tail @ ..] =>
        std::iter::once(*head)
          .chain(compare(e, tail))
          .collect(),
    }
  }

  match ns {
    [] =>
      vec![],
    [head, tail @ ..] =>
      compare(*head, &isort(tail)),
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_isort_1() {
    assert_eq!(isort(&[5,4,3,2,1]), &[1,2,3,4,5]);
  }

  #[test]
  fn test_isort_2() {
    assert_eq!(
      isort(&[345,19283,11,200,389283]),
      &[11,200,345,19283,389283],
    );
  }

  #[test]
  fn test_isort_3() {
    assert_eq!(isort(&[2,5,7,4,1,3]), &[1,2,3,4,5,7]);
  }

  #[test]
  fn test_isort_4() {
    assert_eq!(
      isort(&['b','r','t','a','i','s']),
      &['a','b','i','r','s','t'],
    );
  }
}
