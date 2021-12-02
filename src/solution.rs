pub fn part1(input: &str) -> String {
  let mut prev: i64 = i64::MAX;
  let mut increments = 0;
  for line in input.trim().lines() {
    let measurement = match line.parse::<i64>() {
      Err(e) => panic!("{}", e),
      Ok(v) => v,
    };
    if measurement > prev {
      increments += 1;
    }
    prev = measurement;
  }

  format!("{}", increments)
}

pub fn part2(input: &str) -> String {
  let measurements: Vec<i64> = input
    .trim()
    .lines()
    .flat_map(|x| x.parse::<i64>())
    .collect();
  let mut increments = 0;
  for i in 0..measurements.len() - 3 {
    let window_a: i64 = measurements[i..i + 3].iter().sum();
    let window_b: i64 = measurements[i + 1..i + 4].iter().sum();
    if window_b > window_a {
      increments += 1;
    }
  }

  format!("{}", increments)
}

#[cfg(test)]
mod test {
  use super::*;
  #[test]
  fn test_p1() {
    let input = "199\n\
        200\n\
        208\n\
        210\n\
        200\n\
        207\n\
        240\n\
        269\n\
        260\n\
        263";
    assert_eq!(&part1(input), "7");
  }

  #[test]
  fn test_p2() {
    let input = "199\n\
        200\n\
        208\n\
        210\n\
        200\n\
        207\n\
        240\n\
        269\n\
        260\n\
        263";
    assert_eq!(&part2(input), "5");
  }
}
