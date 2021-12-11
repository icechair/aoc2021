mod index;
use index::*;
use std::fmt::Debug;
fn parse_lightmap(input: &str) -> (Vec<u8>, usize, usize) {
  let mut lightmap: Vec<u8> = Vec::with_capacity(input.trim().len());
  let mut height = 0;
  let mut width = 0;
  for line in input.trim().lines() {
    height += 1;
    width = line.trim().len();
    for b in line.trim().bytes() {
      lightmap.push(b - b'0')
    }
  }
  return (lightmap, width, height);
}

fn print_map<T>(map: &[T], width: usize, height: usize)
where
  T: Debug,
{
  let mut out = String::new();
  for row in 0..height {
    for col in 0..width {
      let index = index(row, col, width);
      out.push_str(&format!(" {:?} ", map[index]))
    }
    out.push('\n');
  }
  println!("{}", out.trim_end())
}

pub fn part1(input: &str) -> String {
  return format!("{}", 0);
}

pub fn part2(_input: &str) -> String {
  return format!("0");
}

#[cfg(test)]
mod test {
  use super::*;

  const INPUT: &str = "\
5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526
";

  #[test]
  fn test_p1() {
    assert_eq!(&part1(INPUT), "1656");
  }

  #[test]
  fn test_p2() {
    assert_eq!(&part2(INPUT), "0");
  }
}
