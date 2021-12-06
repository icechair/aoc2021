fn parse_lanterns(input: &str) -> Vec<usize> {
  input.trim().split(',').flat_map(|x| x.parse()).collect()
}

fn simulate_lanterns(list: &[usize], days: usize) -> u128 {
  let mut lantern_fishes = [0; 9];
  for &age in list {
    lantern_fishes[age] += 1;
  }
  for _ in 0..days {
    let newborn = lantern_fishes[0];
    for age in 0..8 {
      lantern_fishes[age] = lantern_fishes[age + 1]
    }
    lantern_fishes[6] += newborn;
    lantern_fishes[8] = newborn;
  }

  return lantern_fishes.iter().sum();
}

pub fn part1(input: &str) -> String {
  let lanterns = parse_lanterns(input);
  let amount = simulate_lanterns(&lanterns, 80);
  return format!("{}", amount);
}

pub fn part2(input: &str) -> String {
  let lanterns = parse_lanterns(input);
  let amount = simulate_lanterns(&lanterns, 256);
  return format!("{}", amount);
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
    assert_eq!(&part2(INPUT), "26984457539");
  }
}
