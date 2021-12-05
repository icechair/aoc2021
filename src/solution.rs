use std::collections::HashMap;
use std::fmt;
use std::ops;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
  x: i64,
  y: i64,
}

impl ops::Add for &Point {
  type Output = Point;
  fn add(self, rhs: &Point) -> Point {
    let x = self.x + rhs.x;
    let y = self.y + rhs.y;
    return Point { x, y };
  }
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
  direction: Point,
  cursor: Point,
}
impl<'a> Iterator for LinePoints<'a> {
  type Item = Point;
  fn next(&mut self) -> Option<Self::Item> {
    if self.direction.x == 1 {
      if self.cursor.x > self.line.to.x {
        return None;
      }
    }
    if self.direction.x == -1 {
      if self.cursor.x < self.line.to.x {
        return None;
      }
    }
    if self.direction.y == 1 {
      if self.cursor.y > self.line.to.y {
        return None;
      }
    }
    if self.direction.y == -1 {
      if self.cursor.y < self.line.to.y {
        return None;
      }
    }
    let next = self.cursor.clone();
    self.cursor = &self.cursor + &self.direction;
    return Some(next);
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
    let from = Point::from(points.next().expect("from shoudnt be empty"));
    let to = Point::from(points.next().expect("from shoudnt be empty"));
    return Line { from, to };
  }

  fn direction(&self) -> Point {
    let mut x = self.to.x - self.from.x;
    let mut y = self.to.y - self.from.y;
    if x > 0 {
      x = 1;
    } else if x < 0 {
      x = -1;
    } else {
      x = 0;
    }
    if y > 0 {
      y = 1;
    } else if y < 0 {
      y = -1;
    } else {
      y = 0;
    }
    return Point { x, y };
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
      direction: self.direction(),
      cursor: self.from,
    };
  }
}

fn print_map(points: &HashMap<Point, i64>) {
  let min_x = points
    .keys()
    .fold(i64::MAX, |acc, c| if acc > c.x { c.x } else { acc });
  let max_x = points
    .keys()
    .fold(0, |acc, c| if acc < c.x { c.x } else { acc });
  let min_y = points
    .keys()
    .fold(i64::MAX, |acc, c| if acc > c.y { c.y } else { acc });
  let max_y = points
    .keys()
    .fold(0, |acc, c| if acc < c.y { c.y } else { acc });

  let mut output = String::new();
  for y in min_y..max_y + 1 {
    for x in min_x..max_x + 1 {
      output.push_str(&match points.get(&Point { x, y }) {
        Some(v) => format!("{}", v),
        None => format!("."),
      })
    }
    output.push('\n');
  }
  println!("{}", output.trim_end())
}

fn count_overlaps(lines: &mut Vec<Line>) -> usize {
  let mut point_map: HashMap<Point, i64> = HashMap::new();

  for line in lines {
    for point in line.points() {
      let e = point_map.entry(point).or_insert(0);
      *e += 1;
    }
  }
  print_map(&point_map);
  return point_map.iter().filter(|&(_, v)| v > &1).count();
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

pub fn part2(input: &str) -> String {
  let mut lines: Vec<Line> = input.trim().lines().map(|l| Line::from(l)).collect();

  return format!("{}", count_overlaps(&mut lines));
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
    assert_eq!(line.points().count(), 9);

    let line = Line::from("7,0 -> 7,4");
    assert_eq!(line.is_horizontal(), false);
    assert_eq!(line.is_vertical(), true);
    assert_eq!(line.points().count(), 5);

    let line = Line::from("5,5 -> 8,2");
    assert_eq!(line.is_horizontal(), false);
    assert_eq!(line.is_vertical(), false);
    assert_eq!(line.points().count(), 4);
  }

  #[test]
  fn test_p1() {
    assert_eq!(&part1(INPUT), "5");
  }

  #[test]
  fn test_p2() {
    assert_eq!(&part2(INPUT), "12");
  }
}
