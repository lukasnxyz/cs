fn fibonacci(n: u64) -> u64 {
  (0..n).fold((0,1), |(a,b),_| (b,a + b)).0
  //match n {
  //  0 => 0,
  //  1 => 1,
  //  x => fibonacci(x - 1) + fibonacci(x - 2),
  //}
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_fib() {
    assert_eq!(
      (1..=20).map(|i| fibonacci(i)).collect::<Vec<u64>>(),
      &[1,1,2,3,5,8,13,21,34,55,89,144,233,377,610,987,1597,2584,4181,6765],
      "first 20 fibonacci numbers",
    );
  }
}
