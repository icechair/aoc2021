mod index;
use std::collections::VecDeque;
use std::iter::FromIterator;
struct LightMap {
  list: Vec<u8>,
  width: usize,
  height: usize,
}
impl LightMap {
  fn from(input: &str) -> LightMap {
    let mut list: Vec<u8> = Vec::with_capacity(input.trim().len());
    let mut height = 0;
    let mut width = 0;
    for line in input.trim().lines() {
      height += 1;
      width = line.trim().len();
      for b in line.trim().bytes() {
        list.push(b - b'0')
      }
    }
    return LightMap {
      list,
      width,
      height,
    };
  }

  pub fn step(&mut self) -> usize {
    let mut queue = VecDeque::from_iter(0..self.list.len());
    let mut flashes = Vec::new();
    while let Some(id) = queue.pop_front() {
      if self.list[id] <= 9 {
        self.list[id] += 1;
      }

      if self.list[id] > 9 && flashes.contains(&id) == false {
        flashes.push(id);
        for next in index::neighbours(id, self.width, self.height, true) {
          queue.push_back(next);
        }
      }
    }
    for &flash in &flashes {
      self.list[flash] = 0;
    }
    return flashes.len();
  }

  pub fn print(&self) {
    index::print_map(&self.list, self.width, self.height);
  }
}

pub fn part1(input: &str) -> String {
  let mut light_map = LightMap::from(input);
  light_map.print();
  let mut flashes = 0;
  for turn in 0..100 {
    flashes += light_map.step();
    println!("turn {}", turn + 1);
    light_map.print();
  }
  return format!("{}", flashes);
}

pub fn part2(input: &str) -> String {
  let mut light_map = LightMap::from(input);
  let mut turn = 0;
  while light_map.step() != light_map.list.len() {
    turn += 1;
  }
  return format!("{}", turn + 1);
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
    assert_eq!(&part2(INPUT), "195");
  }
}
