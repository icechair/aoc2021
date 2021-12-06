fn parse_lanterns(input: &str) -> Vec<u8> {
  input.trim().split(',').flat_map(|x| x.parse()).collect()
}

fn simulate_lanterns(list: &mut Vec<u8>, total_days: usize) {
  for _ in 0..total_days {
    let mut newborn: Vec<u8> = Vec::new();
    for fish in list.iter_mut() {
      if *fish == 0 {
        newborn.push(8);
        *fish = 6;
      } else {
        *fish -= 1;
      }
    }
    list.append(&mut newborn);
  }
}

pub fn part1(input: &str) -> String {
  let mut lanterns = parse_lanterns(input);
  simulate_lanterns(&mut lanterns, 80);
  return format!("{}", lanterns.len());
}

pub fn part2(_input: &str) -> String {
  return format!("{}", 0);
}

#[cfg(test)]
mod test {
  use super::*;

  const INPUT: &str = "\
  3,4,3,1,2
";

  #[test]
  fn test_p1() {
    assert_eq!(&part1(INPUT), "5934");
  }

  #[test]
  fn test_p2() {
    assert_eq!(&part2(INPUT), "0");
  }
}
