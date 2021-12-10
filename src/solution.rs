use std::collections::VecDeque;

const DELIM_END: [char; 4] = [')', ']', '}', '>'];
const DELIM_SCORE: [i64; 4] = [3, 57, 1197, 25137];

fn parse_line(line: &str) -> Result<(), i64> {
  let mut stack: Vec<char> = vec![];
  for ch in line.trim().chars() {
    let pop = match ch {
      '(' => {
        stack.push(')');
        None
      }
      '[' => {
        stack.push(']');
        None
      }
      '{' => {
        stack.push('}');
        None
      }
      '<' => {
        stack.push('>');
        None
      }
      ')' | ']' | '}' | '>' => stack.pop(),
      _ => panic!("invalid char in line: {}, {}", ch, line),
    };

    if let Some(pop) = pop {
      if pop != ch {
        return Err(match ch {
          ')' => 3,
          ']' => 57,
          '}' => 1197,
          '>' => 25137,
          _ => panic!("invalid char on stack: {}, {:?}", ch, stack),
        });
      }
    }
  }
  return Ok(());
}

pub fn part1(input: &str) -> String {
  let mut score = 0;
  for line in input.trim().lines() {
    if let Err(s) = parse_line(line) {
      score += s;
    }
  }
  return format!("{}", score);
}

pub fn part2(_input: &str) -> String {
  return format!("{}", 0);
}

#[cfg(test)]
mod test {
  use super::*;

  const INPUT: &str = "\
[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]
";

  #[test]
  fn test_p1() {
    assert_eq!(&part1(INPUT), "26397");
  }

  #[test]
  fn test_p2() {
    assert_eq!(&part2(INPUT), "0");
  }
}
