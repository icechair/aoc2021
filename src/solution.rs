use std::ops;
mod index;

fn Box<T>(t: T) -> Box<T> {
  return Box::new(t);
}

#[derive(Debug, PartialEq, Eq)]
enum Element {
  Num(u8),
  Pair(Box<Element>, Box<Element>),
}

fn Num(t: u8) -> Element {
  return Element::Num(t);
}

fn Pair(a: Element, b: Element) -> Element {
  return Element::Pair(Box(a), Box(b));
}

fn Element(tokens: &mut dyn Iterator<Item = u8>) -> Element {
  while let Some(token) = tokens.next() {
    match token {
      b'[' => {
        let a = Element(tokens);
        let b = Element(tokens);
        return Pair(a, b);
      }
      b']' | b',' => return Element(tokens),
      digit => return Num(digit - b'0'),
    }
  }
  unreachable!("what happened?")
}

fn reduce_element(elem: &mut Element, depth: usize) -> Element {
  match elem {
    Element::Pair(a, b) => {
      if depth == 4 {}
      return *elem;
    }
    Element::Num(n) => {
      if *n > 9 {
        if *n % 2 == 1 {
          return Pair(Num(*n / 2), Num((*n / 2) + 1));
        }
        return Pair(Num(*n / 2), Num(*n / 2));
      }
      return *elem;
    }
  }
}

impl ops::Add for Element {
  type Output = Element;
  fn add(self, rhs: Self) -> Element {
    let mut pair = Pair(self, rhs);
    return pair;
  }
}

pub fn part1(_input: &str) -> String {
  return format!("0");
}

pub fn part2(_input: &str) -> String {
  return format!("0");
}

#[cfg(test)]
mod test {
  use super::*;

  const INPUT: &str = "\
";

  #[test]
  fn test_parse() {
    assert_eq!(Element(&mut "[1,2]".bytes()), Pair(Num(1), Num(2)));
    assert_eq!(
      Element(&mut "[[1,2],3]".bytes()),
      Pair(Pair(Num(1), Num(2)), Num(3))
    );
    assert_eq!(
      Element(&mut "[9,[8,7]]".bytes()),
      Pair(Num(9), Pair(Num(8), Num(7)))
    );
    assert_eq!(
      Element(&mut "[[1,9],[8,5]]".bytes()),
      Pair(Pair(Num(1), Num(9)), Pair(Num(8), Num(5)))
    );
  }

  #[test]
  fn test_add() {
    let a = Element(&mut "[1,2]".bytes());
    let b = Element(&mut "[[3,4],5]".bytes());
    assert_eq!(
      a + b,
      Pair(Pair(Num(1), Num(2),), Pair(Pair(Num(3), Num(4)), Num(5)))
    );
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
