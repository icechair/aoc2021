fn parse_command(line: &str) -> Vec<i64> {
  let mut parts = line.trim().split_ascii_whitespace();
  let cmd = parts.next();
  let value = match parts.next() {
    None => panic!("invalid cmd, no value: {}", line),
    Some(num) => match num.parse::<i64>() {
      Err(e) => panic!("invalid cmd value: {}", e),
      Ok(v) => v,
    },
  };
  match cmd {
    None => panic!("invalid cmd, empty string"),
    Some(c) => match c {
      "forward" => vec![value, 0],
      "down" => vec![0, value],
      "up" => vec![0, -value],
      _ => panic!("invalid cmd: {}", c),
    },
  }
}

fn add_command(position: &mut Vec<i64>, cmd: &[i64]) {
  for i in 0..position.len() {
    position[i] += cmd[i]
  }
}

pub fn part1(input: &str) -> String {
  let mut position = vec![0, 0];
  for line in input.trim().lines() {
    let cmd = parse_command(line);
    add_command(&mut position, &cmd);
  }
  format!("{}", position.iter().fold(1, |acc, c| acc * c))
}

pub fn part2(_input: &str) -> String {
  format!("{}", 0)
}

#[cfg(test)]
mod test {
  use super::*;
  #[test]
  fn test_p1() {
    let input = "\
      forward 5\n\
      down 5\n\
      forward 8\n\
      up 3\n\
      down 8\n\
      forward 2";
    assert_eq!(&part1(input), "150");
  }

  #[test]
  fn test_p2() {}
}
