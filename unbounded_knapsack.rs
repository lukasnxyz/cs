fn count_change(money: i32, coins: &[i32]) -> i32 {
  match (money, coins) {
    (0, _) => 1,
    (m, _) if m < 0 => 0,
    (_, c) if c.is_empty() => 0,
    (m, [x, xs @ ..]) =>
      count_change(m - x, coins) + count_change(m, xs),
    (_, []) => 0,
  }
}

fn main() {
  assert_eq!(count_change(1, &[1,2]), 1);
  assert_eq!(count_change(1, &[2,1]), 1);

  assert_eq!(count_change(4, &[2,1]), 3);
  assert_eq!(count_change(4, &[1,2]), 3);

  assert_eq!(count_change(4, &[3,1,2]), 4);
  assert_eq!(count_change(4, &[2,3,1]), 4);
  assert_eq!(count_change(4, &[2,1,3]), 4);
}
