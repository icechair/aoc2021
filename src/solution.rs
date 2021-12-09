mod index;
use index::*;
fn parse_heightmap(input: &str) -> (Vec<u8>, usize, usize) {
  let mut heightmap = Vec::with_capacity(input.trim().len());
  let mut width = 0;
  let mut height = 0;
  for line in input.trim().lines() {
    height += 1;
    width = line.trim().len();
    for b in line.trim().bytes() {
      heightmap.push(b - b'0');
    }
  }
  return (heightmap, width, height);
}

pub fn part1(input: &str) -> String {
  let (mut heightmap, width, height) = parse_heightmap(input);
  let mut low_points: Vec<u8> = Vec::new();
  for (ida, h_a) in heightmap.iter().enumerate() {
    let mut is_lowest = true;
    for (idb, h_b) in index::neighbours(&heightmap, ida, width, height) {
      if h_a >= h_b {
        is_lowest = false;
      }
    }
    if is_lowest {
      low_points.push(*h_a);
    }
  }
  return format!(
    "{}",
    low_points
      .iter()
      .fold(0 as usize, |acc, &c| acc + 1 + usize::from(c))
  );
}

pub fn part2(_input: &str) -> String {
  return format!("{}", 0);
}

#[cfg(test)]
mod test {
  use super::*;

  const INPUT: &str = "\
2199943210
3987894921
9856789892
8767896789
9899965678
";

  #[test]
  fn test_p1() {
    assert_eq!(&part1(INPUT), "15");
  }

  #[test]
  fn test_p2() {
    assert_eq!(&part2(INPUT), "0");
  }
}
