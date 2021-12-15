use std::cmp;
use std::collections::BinaryHeap;
mod index;

#[derive(Copy, Clone, Eq, PartialEq)]
struct Priority {
  index: usize,
  cost: usize,
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
  list: Vec<u8>,
  width: usize,
  height: usize,
}

fn parse_grid(input: &str) -> Grid {
  let mut list = Vec::with_capacity(input.trim().len());
  let mut width = 0;
  let mut height = 0;
  for line in input.trim().lines() {
    height += 1;
    width = line.trim().len();
    for b in line.trim().bytes() {
      list.push(b - b'0');
    }
  }
  return Grid {
    list,
    width,
    height,
  };
}

impl Grid {
  pub fn shortest_path(&self, start: usize, target: usize) -> Option<usize> {
    let mut dist: Vec<_> = (0..self.list.len()).map(|_| usize::MAX).collect();
    let mut heap = BinaryHeap::new();

    dist[start] = 0;
    heap.push(Priority {
      index: start,
      cost: 0,
    });
    while let Some(Priority { index, cost }) = heap.pop() {
      if index == target {
        return Some(cost);
      }
      if cost > dist[index] {
        continue;
      }
      for idn in index::neighbours(index, self.width, self.height, false) {
        let next = Priority {
          index: idn,
          cost: cost + usize::from(self.list[idn]),
        };
        if next.cost < dist[next.index] {
          heap.push(next);
          dist[next.index] = next.cost;
        }
      }
    }
    return None;
  }
}

pub fn part1(input: &str) -> String {
  let grid = parse_grid(input);
  if let Some(cost) = grid.shortest_path(0, grid.list.len() - 1) {
    return format!("{}", cost);
  }
  return format!("0");
}

pub fn part2(_input: &str) -> String {
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
  fn test_p1() {
    assert_eq!(&part1(INPUT), "40");
  }

  #[test]
  fn test_p2() {
    assert_eq!(&part2(INPUT), "0");
  }
}
