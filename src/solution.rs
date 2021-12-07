fn naive_fuel_cost(crabs: &[i64], pos: i64) -> i64 {
  let mut cost = 0;
  for &crab in crabs {
    cost += i64::abs(pos - crab);
  }
  return cost;
}

fn crab_fuel_cost(from: i64, to: i64) -> i64 {
  let delta = i64::abs(to - from);
  let mut cost = 0;
  for i in 0..delta + 1 {
    cost += i;
  }
  return cost;
}

fn pattern_fuel_cost(crabs: &[i64], pos: i64) -> i64 {
  let mut cost = 0;
  for &crab in crabs {
    cost += crab_fuel_cost(crab, pos);
  }
  return cost;
}

pub fn part1(input: &str) -> String {
  let mut crabs: Vec<i64> = input.trim().split(',').flat_map(|x| x.parse()).collect();
  crabs.sort();
  let min = *crabs.first().unwrap();
  let max = *crabs.last().unwrap();
  let mut cost = i64::MAX;
  for pos in min..max + 1 {
    let current = naive_fuel_cost(&crabs, pos);
    if current < cost {
      cost = current;
    }
  }

  return format!("{}", cost);
}

pub fn part2(input: &str) -> String {
  let mut crabs: Vec<i64> = input.trim().split(',').flat_map(|x| x.parse()).collect();
  crabs.sort();
  let min = *crabs.first().unwrap();
  let max = *crabs.last().unwrap();
  let mut cost = i64::MAX;
  for pos in min..max + 1 {
    let current = pattern_fuel_cost(&crabs, pos);
    if current < cost {
      cost = current;
    }
  }

  return format!("{}", cost);
}

#[cfg(test)]
mod test {
  use super::*;

  const INPUT: &str = "\
  16,1,2,0,4,2,7,1,2,14
";

  #[test]
  fn test_fuel_cost() {
    assert_eq!(crab_fuel_cost(1, 5), 10);
    assert_eq!(crab_fuel_cost(2, 5), 6);
  }

  #[test]
  fn test_p1() {
    assert_eq!(&part1(INPUT), "37");
  }

  #[test]
  fn test_p2() {
    assert_eq!(&part2(INPUT), "168");
  }
}
