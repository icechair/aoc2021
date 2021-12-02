pub fn part1(_input: &str) -> String {
  format!("{}", 0)
}

pub fn part2(_input: &str) -> String {
  format!("{}", 0)
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
  fn test_p2() {}
}
