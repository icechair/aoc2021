enum Command {
  Forward(i64),
  Depth(i64),
}

fn parse_command(line: &str) -> Command {
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
      "forward" => Command::Forward(value),
      "down" => Command::Depth(value),
      "up" => Command::Depth(-value),
      _ => panic!("invalid cmd: {}", c),
    },
  }
}

fn run_command_v1(position: &mut Vec<i64>, cmd: &Command) {
  match cmd {
    Command::Forward(value) => position[0] += value,
    Command::Depth(value) => position[1] += value,
  }
}

fn run_command_v2(position: &mut Vec<i64>, cmd: &Command) {
  match cmd {
    Command::Depth(value) => position[2] += value,
    Command::Forward(value) => {
      position[0] += value;
      position[1] += position[2] * value;
    }
  }
}

pub fn part1(input: &str) -> String {
  let mut position = vec![0, 0];
  for line in input.trim().lines() {
    let cmd = parse_command(line);
    run_command_v1(&mut position, &cmd);
  }
  format!("{}", position[0] * position[1])
}

pub fn part2(input: &str) -> String {
  let mut position = vec![0, 0, 0];
  for line in input.trim().lines() {
    let cmd = parse_command(line);
    run_command_v2(&mut position, &cmd);
  }
  format!("{}", position[0] * position[1])
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
  fn test_p2() {
    let input = "\
      forward 5\n\
      down 5\n\
      forward 8\n\
      up 3\n\
      down 8\n\
      forward 2";
    assert_eq!(&part2(input), "900");
  }
}
