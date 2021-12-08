pub fn count_unique_segments(line: &str) -> usize {
  let mut parts = line.split('|');
  let _ = parts.next();
  let output_values = parts.next().unwrap();
  return output_values
    .trim()
    .split_ascii_whitespace()
    .filter(|x| x.len() == 2 || x.len() == 3 || x.len() == 4 || x.len() == 7)
    .count();
}

pub fn part1(input: &str) -> String {
  let mut unique_values = 0;
  for line in input.trim().lines() {
    unique_values += count_unique_segments(line);
  }
  return format!("{}", unique_values);
}

pub fn part2(_input: &str) -> String {
  return format!("{}", 0);
}

#[cfg(test)]
mod test {
  use super::*;

  const INPUT: &str = "\
    be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe\n\
    edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc\n\
    fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg\n\
    fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb\n\
    aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea\n\
    fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb\n\
    dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe\n\
    bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef\n\
    egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb\n\
    gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce\n\
";

  #[test]
  fn test_p1() {
    assert_eq!(&part1(INPUT), "26");
  }

  #[test]
  fn test_p2() {
    assert_eq!(&part2(INPUT), "0");
  }
}
