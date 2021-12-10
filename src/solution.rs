fn parse_line(line: &str) -> Result<Vec<char>, i64> {
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
  stack.reverse();
  return Ok(stack);
}

fn autocomplete_score(stack: &[char]) -> i64 {
  let mut score = 0;
  for &ch in stack {
    score *= 5;
    score += match ch {
      ')' => 1,
      ']' => 2,
      '}' => 3,
      '>' => 4,
      _ => panic!("invalid char on stack: {}, {:?}", ch, stack),
    }
  }

  return score;
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

pub fn part2(input: &str) -> String {
  let mut scores = vec![];
  for line in input.trim().lines() {
    if let Ok(stack) = parse_line(line) {
      scores.push(autocomplete_score(&stack));
    }
  }
  scores.sort();
  let center = scores.len() / 2;
  return format!("{}", scores[center]);
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
    assert_eq!(&part2(INPUT), "288957");
  }
}
