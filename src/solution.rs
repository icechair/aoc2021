mod index;
use index::*;
use std::collections::VecDeque;
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

fn get_low_points(heightmap: &[u8], width: usize, height: usize) -> Vec<(usize, u8)> {
  let mut low_points = Vec::new();
  for (ida, h_a) in heightmap.iter().enumerate() {
    let mut is_lowest = true;
    for (_, h_b) in index::neighbours(&heightmap, ida, width, height) {
      if h_a >= h_b {
        is_lowest = false;
      }
    }
    if is_lowest {
      low_points.push((ida, *h_a));
    }
  }
  return low_points;
}

fn get_basin(heightmap: &[u8], width: usize, height: usize, start: usize) -> Vec<usize> {
  let mut basin = Vec::new();
  basin.push(start);

  let mut queue = VecDeque::new();
  queue.push_front(start);
  while let Some(current) = queue.pop_front() {
    for (idn, &h_n) in neighbours(&heightmap, current, width, height) {
      if basin.contains(&idn) == false && h_n < 9 {
        basin.push(idn);
        queue.push_back(idn);
      }
    }
  }
  return basin;
}

pub fn part1(input: &str) -> String {
  let (heightmap, width, height) = parse_heightmap(input);
  let low_points = get_low_points(&heightmap, width, height);
  return format!(
    "{}",
    low_points
      .iter()
      .fold(0 as usize, |acc, &(_, c)| acc + 1 + usize::from(c))
  );
}

fn print_low_points(heightmap: &[u8], low_points: &[(usize, u8)], width: usize, height: usize) {
  let mut out = format!("low_points({}):\n", low_points.len());
  for row in 0..height {
    for col in 0..width {
      let id = index(row, col, width);
      let h = heightmap[id];
      if low_points.contains(&(id, h)) {
        out.push_str(&format!("{}", h))
      } else {
        out.push_str(&format!("."));
      }
    }
    out.push_str("\n");
  }
  println!("{}", out);
}

fn print_basin(heightmap: &[u8], basin: &[usize], width: usize, height: usize) {
  let mut out = format!("basin({}):\n", basin.len());
  for row in 0..height {
    for col in 0..width {
      let id = index(row, col, width);
      let h = heightmap[id];
      if basin.contains(&id) {
        out.push_str(&format!("{}", h))
      } else {
        out.push_str(&format!("."));
      }
    }
    out.push_str("\n");
  }
  println!("{}", out);
}

pub fn part2(input: &str) -> String {
  let (heightmap, width, height) = parse_heightmap(input);
  let low_points = get_low_points(&heightmap, width, height);
  print_low_points(&heightmap, &low_points, width, height);

  let mut basins = Vec::new();
  for &(id, _) in &low_points {
    let basin = get_basin(&heightmap, width, height, id);
    print_basin(&heightmap, &basin, width, height);
    basins.push(basin);
  }
  basins.sort_by(|a, b| b.len().cmp(&a.len()));
  return format!("{}", basins[0].len() * basins[1].len() * basins[2].len());
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
    assert_eq!(&part2(INPUT), "1134");
  }
}
