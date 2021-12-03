pub fn part1(input: &str) -> String {
  let mut bitcounts: Vec<i64> = Vec::new();
  let mut amount: i64 = 0;
  for line in input.trim().lines() {
    if bitcounts.len() == 0 {
      for _ in 0..line.len() {
        bitcounts.push(0);
      }
    }
    for (idx, bit) in line.chars().enumerate() {
      if bit == '1' {
        bitcounts[idx] += 1;
      }
    }
    amount += 1;
  }
  let mut gamma = 0;
  let mut epsilon = 0;
  const base: i64 = 2;
  //println!("{:?}", bitcounts);
  for (bit, &count) in bitcounts.iter().enumerate() {
    //println!("{}, {}, {}, {}", bit, count, amount / 2, count > amount);
    if count > amount / 2 {
      gamma += base.pow((bitcounts.len() - 1 - bit) as u32)
    } else {
      epsilon += base.pow((bitcounts.len() - 1 - bit) as u32)
    }
  }

  format!("{}", gamma * epsilon)
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
      00100\n\
      11110\n\
      10110\n\
      10111\n\
      10101\n\
      01111\n\
      00111\n\
      11100\n\
      10000\n\
      11001\n\
      00010\n\
      01010\n";
    assert_eq!(&part1(input), "198");
  }

  #[test]
  fn test_p2() {}
}
