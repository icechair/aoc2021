use std::iter::FromIterator;
mod list;
use list::drain_filter;
/**
 *  aaaa
 * b    c
 * b    c
 *  dddd
 * e    f
 * e    f
 *  gggg
 */

fn parse_line(line: &str) -> (Vec<String>, Vec<String>) {
  let mut parts = line.split('|');
  let input_values = parts
    .next()
    .unwrap()
    .split_ascii_whitespace()
    .into_iter()
    .map(|x| sort_chars(x))
    .collect();
  let output_values = parts
    .next()
    .unwrap()
    .split_ascii_whitespace()
    .into_iter()
    .map(|x| sort_chars(x))
    .collect();
  return (input_values, output_values);
}

fn sort_chars(line: &str) -> String {
  let mut chars: Vec<char> = line.chars().collect();
  chars.sort();
  return String::from_iter(chars);
}

fn difference(a: &str, b: &str) -> String {
  let mut diff = String::new();
  for ch in a.chars() {
    if b.chars().any(|x| x == ch) == false {
      if !diff.contains(ch) {
        diff.push(ch);
      }
    }
  }
  return diff;
}

fn parse_segments(cypher: &mut Vec<String>) -> Vec<String> {
  let mut digits = Vec::with_capacity(10);
  let mut segments = Vec::with_capacity(7);
  for i in 0..10 {
    digits.push("".to_string());
    if i < 7 {
      segments.push("".to_string());
    }
  }
  cypher.sort_by(|a, b| b.len().cmp(&a.len()));
  digits[1] = cypher.pop().unwrap();
  segments[2] = digits[1].clone();
  segments[5] = segments[2].clone();
  digits[7] = cypher.pop().unwrap();
  segments[0] = difference(&digits[7], &digits[1]);
  segments[3] = difference(&digits[7], &digits[1]);
  digits[4] = cypher.pop().unwrap();
  segments[1] = difference(&digits[4], &digits[7]);
  segments[3] = segments[1].clone();
  cypher.reverse();
  digits[8] = cypher.pop().unwrap();
  segments[4] = difference(&digits[8], &segments[0..4].concat());
  segments[6] = segments[4].clone();
  let mut fives: Vec<String> = cypher.drain(0..3).collect();

  for digit in &fives {
    let diff3 = difference(&segments[3], digit);
    let diff6 = difference(&segments[6], digit);
    if diff3.len() == 1 {
      segments[1] = diff3;
    }
    if diff6.len() == 1 {
      segments[4] = diff6;
    }
  }
  segments[3] = difference(&segments[3], &segments[1]);
  segments[6] = difference(&segments[6], &segments[4]);
  digits[3] = format!(
    "{}{}{}{}",
    segments[0], segments[2], segments[3], segments[6]
  );
  drain_filter(&mut fives, |x| difference(&x, &digits[3]).len() == 0);
  let mut two = drain_filter(&mut fives, |x| x.contains(&segments[4]));
  digits[2] = two.pop().unwrap();
  segments[5] = difference(&segments[2], &digits[2]);
  segments[2] = difference(&segments[2], &segments[5]);
  digits[5] = fives.pop().unwrap();

  digits[0] = format!("{}{}", &segments[0..3].concat(), &segments[4..].concat());
  digits[6] = format!("{}{}", &segments[0..2].concat(), &segments[3..].concat());
  digits[9] = format!("{}{}", &segments[0..4].concat(), &segments[5..].concat());
  println!("{:?}\n{:?}\n{:?}", fives, digits, segments);
  return digits.into_iter().map(|x| sort_chars(&x)).collect();
}

pub fn count_unique_segments(line: &str) -> usize {
  let (_, output_values) = parse_line(line);
  return output_values
    .iter()
    .filter(|x| x.len() == 2 || x.len() == 3 || x.len() == 4 || x.len() == 7)
    .count();
}

fn segment_value(segments: &[String], input_values: &[String]) -> i64 {
  let mut value = 0;
  //println!("segments: {:?}\nvalues: {:?}", segments, input_values);
  println!("segments: {:?}", segments);
  for (e, input) in input_values.iter().enumerate() {
    println!("segment_input: {}", input);
    let index = segments
      .iter()
      .position(|r| sort_chars(r) == sort_chars(&input))
      .unwrap() as i64;
    println!("segment_value: {}", index);
    value += index * i64::pow(10, (input_values.len() - 1 - e) as u32);
  }
  println!("segment_value-value: {}", value);
  return value;
}

pub fn part1(input: &str) -> String {
  let mut unique_values = 0;
  for line in input.trim().lines() {
    unique_values += count_unique_segments(line);
  }
  return format!("{}", unique_values);
}

pub fn part2(input: &str) -> String {
  let mut sum = 0;
  for line in input.trim().lines() {
    let (mut input_values, output_values) = parse_line(line);
    let segments = parse_segments(&mut input_values);
    let value = segment_value(&segments, &output_values);
    //println!("{:?}\n{:?} -> {}", segments, output_values, value);
    sum += value;
  }
  return format!("{}", sum);
}

#[cfg(test)]
mod test {
  use super::*;

  const INPUT: &str = "\
be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
";

  #[test]
  fn test_p1() {
    assert_eq!(&part1(INPUT), "26");
  }

  #[test]
  fn test_p2() {
    assert_eq!(
      &part2(
        "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf"
      ),
      "5353"
    );
    assert_eq!(&part2(INPUT), "61229");
  }
}
