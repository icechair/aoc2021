use std::collections::HashMap;
use std::fmt;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
  x: i64,
  y: i64,
}

impl Point {
  fn from(text: &str) -> Point {
    let values: Vec<i64> = text
      .trim()
      .split(',')
      .flat_map(|x| x.parse::<i64>())
      .collect();
    let x = values[0];
    let y = values[1];
    return Point { x, y };
  }
}
impl fmt::Debug for Point {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "({}, {})", self.x, self.y)
  }
}

#[derive(PartialEq, Eq)]
struct Line {
  from: Point,
  to: Point,
}

struct LinePoints<'a> {
  line: &'a Line,
  current: Point,
}
impl<'a> Iterator for LinePoints<'a> {
  type Item = Point;
  fn next(&mut self) -> Option<Self::Item> {
    if self.line.is_vertical() {
      if self.current.y > self.line.to.y {
        return None;
      }
      let next = self.current.clone();
      self.current.y += 1;
      return Some(next);
    } else if self.line.is_horizontal() {
      //vertical line
      if self.current.x > self.line.to.x {
        return None;
      }
      let next = self.current.clone();
      self.current.x += 1;
      return Some(next);
    }
    return None;
  }
}

impl fmt::Debug for Line {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{:?} -> {:?}", self.from, self.to)
  }
}

impl Line {
  fn from(text: &str) -> Line {
    let mut points = text.trim().split("->");
    let a = Point::from(points.next().expect("from shoudnt be empty"));
    let b = Point::from(points.next().expect("from shoudnt be empty"));
    let mut flip = false;

    if b.x < a.x {
      flip = true;
    } else if b.y < a.y {
      flip = true;
    }
    let (from, to) = match flip {
      true => (b, a),
      false => (a, b),
    };
    return Line { from, to };
  }

  fn is_horizontal(&self) -> bool {
    self.from.y == self.to.y
  }

  fn is_vertical(&self) -> bool {
    self.from.x == self.to.x
  }

  fn is_axis_aligned(&self) -> bool {
    self.is_horizontal() || self.is_vertical()
  }

  fn points<'a>(&'a self) -> LinePoints<'a> {
    return LinePoints {
      line: &self,
      current: self.from,
    };
  }

  fn is_point_on_line(&self, point: &Point) -> bool {
    self.points().any(|p| point == &p)
  }
}

fn count_overlaps(lines: &mut Vec<Line>) -> usize {
  let mut pointMap: HashMap<Point, i64> = HashMap::new();

  for line in lines {
    for point in line.points() {
      let e = pointMap.entry(point).or_insert(0);
      *e += 1;
    }
  }

  return pointMap.iter().filter(|&(_, v)| v > &1).count();
}

pub fn part1(input: &str) -> String {
  let mut lines: Vec<Line> = input
    .trim()
    .lines()
    .map(|l| Line::from(l))
    .filter(|l| l.is_axis_aligned())
    .collect();

  return format!("{}", count_overlaps(&mut lines));
}

pub fn part2(_input: &str) -> String {
  return format!("{}", 0);
}

#[cfg(test)]
mod test {
  use super::*;

  const INPUT: &str = "\
0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2
";

  #[test]
  fn test_line() {
    let line = Line::from("0,9 -> 5,9");
    assert_eq!(line.is_horizontal(), true);
    assert_eq!(line.is_vertical(), false);
    assert_eq!(line.points().count(), 6);

    let line = Line::from("8,0 -> 0,8");
    assert_eq!(line.is_horizontal(), false);
    assert_eq!(line.is_vertical(), false);
    assert_eq!(line.points().count(), 0);

    let line = Line::from("7,0 -> 7,4");
    assert_eq!(line.is_horizontal(), false);
    assert_eq!(line.is_vertical(), true);
    assert_eq!(line.points().count(), 5);
  }

  #[test]
  fn test_p1() {
    assert_eq!(&part1(INPUT), "5");
  }

  #[test]
  fn test_p2() {
    assert_eq!(&part2(INPUT), "0");
  }
}
