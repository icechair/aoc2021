use std::collections::HashSet;
use std::fmt;
use std::iter::FromIterator;
#[derive(Debug)]
enum Fold {
  X(i64),
  Y(i64),
}

fn parse_fold(line: &str) -> Fold {
  let (fold, value) = line.trim().split_once('=').unwrap();
  let value = value.parse().unwrap();
  if fold.ends_with("x") {
    return Fold::X(value);
  }
  return Fold::Y(value);
}

#[derive(Hash, Eq, PartialEq)]
struct Point {
  x: i64,
  y: i64,
}

impl fmt::Debug for Point {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "({},{})", self.x, self.y)
  }
}

impl Point {
  fn from(line: &str) -> Point {
    let mut parts = line.trim().split(',');
    let x = parts.next().unwrap().parse().unwrap();
    let y = parts.next().unwrap().parse().unwrap();

    return Point { x, y };
  }
}

fn parse_paper(input: &str) -> (Vec<Point>, Vec<Fold>) {
  let mut dots = Vec::new();
  let mut folds = Vec::new();
  for line in input.trim().lines() {
    if line.starts_with("fold along") {
      folds.push(parse_fold(line));
    } else if line.len() > 0 {
      dots.push(Point::from(line));
    }
  }
  return (dots, folds);
}

fn fold_paper(dots: &mut Vec<Point>, fold: &Fold) {
  if let &Fold::X(at) = fold {
    for p in dots.iter_mut() {
      if p.x > at {
        let folded = p.x - at;
        p.x = at - folded;
      }
    }
  } else if let &Fold::Y(at) = fold {
    for p in dots.iter_mut() {
      if p.y > at {
        let folded = p.y - at;
        p.y = at - folded;
      }
    }
  }
}

fn print_paper(dots: &HashSet<&Point>) {
  let min_x = dots
    .iter()
    .fold(i64::MAX, |acc, c| if c.x < acc { c.x } else { acc });
  let max_x = dots
    .iter()
    .fold(0, |acc, c| if c.x > acc { c.x } else { acc });
  let min_y = dots
    .iter()
    .fold(i64::MAX, |acc, c| if c.y < acc { c.y } else { acc });
  let max_y = dots
    .iter()
    .fold(0, |acc, c| if c.y > acc { c.y } else { acc });

  let mut out = format!("paper(({},{}) -> ({},{}))\n", min_x, min_y, max_x, max_y);
  for y in min_y..max_y + 1 {
    for x in min_x..max_x + 1 {
      if dots.contains(&Point { x, y }) {
        out.push_str("#");
      } else {
        out.push_str(" ");
      }
    }
    out.push_str("\n");
  }
  println!("{}", out.trim_end());
}

pub fn part1(input: &str) -> String {
  let (mut dots, folds) = parse_paper(input);

  println!("fold:{:?}", &folds[0]);
  //  print_paper(&dots);
  fold_paper(&mut dots, &folds[0]);
  //print_paper(&dots);
  let visible_dots: HashSet<&Point> = HashSet::from_iter(dots.iter());
  return format!("{}", visible_dots.len());
}

pub fn part2(input: &str) -> String {
  let (mut dots, folds) = parse_paper(input);

  println!("fold:{:?}", &folds[0]);
  //  print_paper(&dots);
  for fold in folds {
    fold_paper(&mut dots, &fold);
  }
  let visible_dots: HashSet<&Point> = HashSet::from_iter(dots.iter());
  print_paper(&visible_dots);
  return format!("0");
}

#[cfg(test)]
mod test {
  use super::*;

  const INPUT: &str = "\
6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5
";

  #[test]
  fn test_p1() {
    assert_eq!(&part1(INPUT), "17");
  }

  #[test]
  fn test_p2() {
    assert_eq!(&part2(INPUT), "0");
  }
}
