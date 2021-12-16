use std::cmp;
use std::collections::{BinaryHeap, HashMap};
use std::fmt;
type RowCol = (usize, usize);

#[derive(Copy, Clone, Eq, PartialEq)]
struct Priority {
  index: RowCol,
  cost: usize,
}

fn wrap_add(a: u8, b: u8) -> u8 {
  let sum = a + b;
  if sum > 9 {
    return sum - 9;
  }
  return sum;
}

impl cmp::Ord for Priority {
  fn cmp(&self, other: &Self) -> cmp::Ordering {
    other
      .cost
      .cmp(&self.cost)
      .then_with(|| self.index.cmp(&other.index))
  }
}

impl cmp::PartialOrd for Priority {
  fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
    Some(self.cmp(other))
  }
}

struct Grid {
  list: Vec<Vec<u8>>,
  width: usize,
  height: usize,
}

fn parse_grid(input: &str) -> Grid {
  let mut list = Vec::new();
  let mut width = 0;
  let mut height = 0;
  for line in input.trim().lines() {
    height += 1;
    width = line.trim().len();
    list.push(line.trim().bytes().map(|b| b - b'0').collect());
  }
  return Grid {
    list,
    width,
    height,
  };
}

impl Grid {
  fn neighbours(&self, id: RowCol) -> impl Iterator<Item = RowCol> {
    let mut nexts = Vec::new();
    let (row, col) = id;
    if row > 0 {
      let up = (row - 1, col);
      nexts.push(up);
    }
    if col > 0 {
      let left = (row, col - 1);
      nexts.push(left);
    }
    if col + 1 < self.width {
      let right = (row, col + 1);
      nexts.push(right);
    }
    if row + 1 < self.height {
      let down = (row + 1, col);
      nexts.push(down);
    }
    return nexts.into_iter();
  }

  pub fn shortest_path(&self, start: RowCol, target: RowCol) -> Option<usize> {
    let mut dist: HashMap<RowCol, usize> = HashMap::new();
    let mut heap = BinaryHeap::new();

    dist.insert(start, 0);
    heap.push(Priority {
      index: start,
      cost: 0,
    });
    while let Some(Priority { index, cost }) = heap.pop() {
      if index == target {
        return Some(cost);
      }
      if cost > *dist.entry(index).or_insert(usize::MAX) {
        continue;
      }
      for (row, col) in self.neighbours(index) {
        let next = Priority {
          index: (row, col),
          cost: cost + usize::from(self.list[row][col]),
        };
        if next.cost < *dist.entry(next.index).or_insert(usize::MAX) {
          heap.push(next);
          dist.entry(next.index).and_modify(|x| *x = next.cost);
        }
      }
    }
    return None;
  }

  pub fn expand(&mut self, by: usize) {
    let mut tiles: Vec<Vec<Vec<u8>>> = Vec::new();
    for i in 0..by * by {
      tiles.push(
        self
          .list
          .clone()
          .into_iter()
          .map(|row| row.into_iter().map(|col| wrap_add(col, i as u8)).collect())
          .collect(),
      )
    }
    self.list.clear();
    for row_tile in 0..by {
      for row in 0..self.width {
        let mut new_row = Vec::new();
        for col in 0..by {
          new_row.extend(tiles[row_tile + col][row].clone());
        }
        self.list.push(new_row);
      }
    }
    self.width *= by;
    self.height *= by;
  }
}

impl fmt::Debug for Grid {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let mut out = format!("grid:\n");
    for row in &self.list {
      for v in row {
        out.push_str(&format!("{}", v));
      }
      out.push_str("\n");
    }
    return write!(f, "{}", out.trim_end());
  }
}

pub fn part1(input: &str) -> String {
  let grid = parse_grid(input);
  if let Some(cost) = grid.shortest_path((0, 0), (grid.width - 1, grid.height - 1)) {
    return format!("{}", cost);
  }
  return format!("0");
}

pub fn part2(input: &str) -> String {
  let mut grid = parse_grid(input);
  println!("before:\n{:?}", grid);
  grid.expand(5);
  println!("after:\n{:?}", grid);
  if let Some(cost) = grid.shortest_path((0, 0), (grid.width - 1, grid.height - 1)) {
    return format!("{}", cost);
  }
  return format!("0");
}

#[cfg(test)]
mod test {
  use super::*;

  const INPUT: &str = "\
1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581
";

  #[test]
  fn test_wrap_add() {
    assert_eq!(wrap_add(9, 1), 1);
    assert_eq!(wrap_add(9, 5), 5);
    assert_eq!(wrap_add(8, 5), 4);
  }
  #[test]
  fn test_p1() {
    assert_eq!(&part1(INPUT), "40");
  }

  #[test]
  fn test_p2() {
    assert_eq!(&part2(INPUT), "315");
  }
}
