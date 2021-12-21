use std::ops;
#[derive(Debug, Eq, PartialEq, Clone)]
struct Point {
  x: i64,
  y: i64,
}

fn Point(x: i64, y: i64) -> Point {
  return Point { x, y };
}
fn Velocity(x: i64, y: i64) -> Point {
  return Point(x, y);
}

impl ops::Add for &Point {
  type Output = Point;
  fn add(self, rhs: &Point) -> Point {
    let x = self.x + rhs.x;
    let y = self.y + rhs.y;
    return Point { x, y };
  }
}
impl ops::Sub for &Point {
  type Output = Point;
  fn sub(self, rhs: &Point) -> Point {
    let x = self.x - rhs.x;
    let y = self.y - rhs.y;
    return Point { x, y };
  }
}

#[derive(Debug, Eq, PartialEq)]
struct Rect {
  tl: Point,
  br: Point,
}

impl Rect {
  fn from_str(line: &str) -> Rect {
    let (a, b) = line.trim().split_once(", ").unwrap();
    let (_, ab) = a.trim().split_once("=").unwrap();
    let (_, bb) = b.trim().split_once("=").unwrap();
    let (x1, x2) = ab.trim().split_once("..").unwrap();
    let (y1, y2) = bb.trim().split_once("..").unwrap();
    let x1 = x1.parse().unwrap();
    let x2 = x2.parse().unwrap();
    let y1 = y1.parse().unwrap();
    let y2 = y2.parse().unwrap();

    let tl = Point { x: x1, y: y2 };
    let br = Point { x: x2, y: y1 };
    return Rect { tl, br };
  }
  fn is_left(&self, p: &Point) -> bool {
    p.x < self.tl.x
  }

  fn is_right(&self, p: &Point) -> bool {
    p.x > self.br.x
  }

  fn is_below(&self, p: &Point) -> bool {
    p.y < self.br.y
  }

  fn is_inside(&self, p: &Point) -> bool {
    let x_inside = p.x >= self.tl.x && p.x <= self.br.x;
    let y_inside = if self.tl.y > 0 {
      p.y >= self.tl.y && p.y <= self.br.y
    } else {
      p.y <= self.tl.y && p.y >= self.br.y
    };
    //println!("{:?} inside {:?} ? x:{},y:{}", p, self, x_inside, y_inside);
    x_inside && y_inside
  }
}

#[derive(Debug, Eq, PartialEq)]
struct Probe {
  position: Point,
  velocity: Point,
}

fn Probe(velocity: Point) -> Probe {
  let position = Point(0, 0);
  return Probe { position, velocity };
}

impl Probe {
  fn update(&mut self) {
    self.position.x += self.velocity.x;
    self.position.y += self.velocity.y;
    self.velocity.x += match self.velocity.x {
      x if x > 0 => -1,
      x if x < 0 => 1,
      _ => 0,
    };
    self.velocity.y += -1
  }
}

fn print_simulation(list: &[Point], target: &Rect) {
  let min_x = list.iter().min_by(|a, b| a.x.cmp(&b.x)).unwrap().x;
  let max_x = list.iter().max_by(|a, b| a.x.cmp(&b.x)).unwrap().x;

  let min_y = list.iter().min_by(|a, b| a.y.cmp(&b.y)).unwrap().y;
  let max_y = list.iter().max_by(|a, b| a.y.cmp(&b.y)).unwrap().y;

  let mut out = String::from("");
  out.push_str(&format!("x/y\t{:?}\n", (min_x..max_x)));

  for y in (min_y..max_y + 1).rev() {
    out.push_str(&format!("{0:<4}\t", y));
    for x in min_x..max_x + 1 {
      let p = Point(x, y);
      out.push(match p {
        _ if list.contains(&p) => '#',
        _ if target.is_inside(&p) => 'T',
        _ => '.',
      });
    }
    out.push('\n')
  }

  println!("{}", out.trim_end())
}

enum SimErr {
  Undershot,
  Overshot,
  Drift,
  Timeout,
}

fn simulate(target: &Rect, velocity: &Point) -> Result<i64, SimErr> {
  let mut probe = Probe(velocity.clone());
  let mut height = i64::MIN;
  let mut turn = 1;
  let mut points = Vec::new();
  points.push(velocity.clone());
  loop {
    probe.update();
    points.push(probe.position.clone());
    height = i64::max(height, probe.position.y);
    if target.is_inside(&probe.position) {
      println!("simulate: {:?} ok {}", velocity, height);
      //print_simulation(&points, &target);
      return Ok(height);
    }
    if probe.velocity.x == 0 && target.is_left(&probe.position) {
      //println!("simulate: {:?} UNDER", velocity);
      //print_simulation(&points, &target);
      return Err(SimErr::Undershot);
    }
    if target.is_right(&probe.position) {
      //println!("simulate: {:?} over:", velocity);
      //print_simulation(&points, &target);
      return Err(SimErr::Overshot);
    }
    if target.is_below(&probe.position) {
      //println!("simulate: {:?} DRIFTTT", velocity);
      //print_simulation(&points, &target);
      return Err(SimErr::Drift);
    }
    turn += 1;
    if turn > 10000 {
      return Err(SimErr::Timeout);
    }
  }
}

pub fn part1(input: &str) -> String {
  let target = Rect::from_str(input);
  let mut height = i64::MIN;
  let max = 1000;
  for x in -max..max {
    for y in -max..max {
      let velocity = Velocity(x, y);
      match simulate(&target, &velocity) {
        Err(e) => match e {
          SimErr::Undershot => break,
          SimErr::Overshot => return format!("{}", height),
          //SimErr::Drift => break,
          _ => (),
        },
        Ok(h) => height = i64::max(height, h),
      }
    }
  }
  return format!("{}", height);
}

pub fn part2(input: &str) -> String {
  let target = Rect::from_str(input);
  let mut hits = 0;
  let max = 1000;
  for x in 1..2 * target.tl.x {
    for y in -max..max {
      let velocity = Velocity(x, y);
      match simulate(&target, &velocity) {
        Err(e) => match e {
          SimErr::Undershot => break,
          //SimErr::Overshot => return format!("{}", hits),
          //SimErr::Drift => break,
          _ => (),
        },
        Ok(_) => hits += 1,
      }
    }
  }
  return format!("{}", hits);
}

#[cfg(test)]
mod test {
  use super::*;

  const INPUT: &str = "\
  target area: x=20..30, y=-10..-5
";

  #[test]
  fn test_rect_parse() {
    let rect = Rect::from_str(INPUT);
    assert_eq!(rect.tl, Point { x: 20, y: -5 });
    assert_eq!(rect.br, Point { x: 30, y: -10 });
    assert_eq!(rect.is_inside(&Point(28, -7)), true);
  }

  #[test]
  fn test_probe() {
    let mut probe = Probe(Velocity(7, 2));
    probe.update();
    assert_eq!(probe.position, Point(7, 2));
    probe.update();
    assert_eq!(probe.position, Point(13, 3));
    probe.update();
    assert_eq!(probe.position, Point(18, 3));
    probe.update();
    assert_eq!(probe.position, Point(22, 2));
    probe.update();
    assert_eq!(probe.position, Point(25, 0));

    probe.update();
    assert_eq!(probe.position, Point(27, -3));
  }

  #[test]
  fn test_p1() {
    assert_eq!(&part1(INPUT), "0");
  }

  #[test]
  fn test_p2() {
    assert_eq!(&part2(INPUT), "0");
  }
}
