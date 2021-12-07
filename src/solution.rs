pub fn fuel_cost(crabs: &[i64], pos: i64) -> i64 {
  let mut cost = 0;
  for crab in crabs {
    cost += i64::abs(pos - crab);
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
    let current = fuel_cost(&crabs, pos);
    if current < cost {
      cost = current;
    }
  }

  return format!("{}", cost);
}

pub fn part2(_input: &str) -> String {
  return format!("{}", 0);
}

#[cfg(test)]
mod test {
  use super::*;

  const INPUT: &str = "\
  16,1,2,0,4,2,7,1,2,14
";

  #[test]
  fn test_p1() {
    assert_eq!(&part1(INPUT), "37");
  }

  #[test]
  fn test_p2() {
    assert_eq!(&part2(INPUT), "0");
  }
}
