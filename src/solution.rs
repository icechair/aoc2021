enum Command {
  Forward(i64),
  Aim(i64),
}

fn parse_command(line: &str) -> Command {
  let mut parts = line.trim().split_ascii_whitespace();
  let cmd = parts.next().expect("line should not be empty");
  let value = parts
    .next()
    .expect("value should not be empty")
    .parse::<i64>()
    .expect("value should be a number");
  match cmd {
    "forward" => Command::Forward(value),
    "down" => Command::Aim(value),
    "up" => Command::Aim(-value),
    _ => panic!("invalid cmd: {}", cmd),
  }
}

fn run_command_v1(sub: &mut [i64; 2], cmd: &Command) {
  let [ref mut hpos, ref mut depth] = sub;
  match cmd {
    Command::Forward(value) => *hpos += value,
    Command::Aim(value) => *depth += value,
  }
}

fn run_command_v2(position: &mut [i64; 3], cmd: &Command) {
  let [ref mut hpos, ref mut depth, ref mut aim] = position;
  match cmd {
    Command::Aim(value) => *aim += value,
    Command::Forward(value) => {
      *hpos += value;
      *depth += *aim * value;
    }
  }
}

pub fn part1(input: &str) -> String {
  let mut submarine = [0; 2];
  for line in input.trim().lines() {
    let cmd = parse_command(line);
    run_command_v1(&mut submarine, &cmd);
  }
  let [hpos, depth] = submarine;
  format!("{}", hpos * depth)
}

pub fn part2(input: &str) -> String {
  let mut submarine = [0; 3];
  for line in input.trim().lines() {
    let cmd = parse_command(line);
    run_command_v2(&mut submarine, &cmd);
  }
  let [hpos, depth, _] = submarine;
  format!("{}", hpos * depth)
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
