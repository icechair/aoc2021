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
    let input = "\
      #1 @ 1,3: 4x4\n\
      #2 @ 3,1: 4x4\n\
      #3 @ 5,5: 2x2\n";
    assert_eq!(&part1(input), "4");

    let input = "\
    #1 @ 1,3: 4x3\n\
    #2 @ 2,1: 2x4\n\
    #3 @ 3,2: 4x2\n\
    #4 @ 6,1: 3x5\n";
    assert_eq!(&part1(input), "8");
  }

  #[test]
  fn test_p2() {}
}
