use std::collections::HashMap;

type Polymer = Vec<u8>;
type Rule = (u16, u8);
type Rules = HashMap<u16, u8>;

fn u88_16(hi: u8, lo: u8) -> u16 {
  ((hi as u16) << 8) | lo as u16
}

fn parse_rule(line: &str) -> Rule {
  let (from, to) = line.trim().split_once(" -> ").unwrap();
  let from = from.as_bytes();
  let from = u88_16(from[0], from[1]);
  let to = to.as_bytes();
  let to = to[0];
  return (from, to);
}

fn parse_template(input: &str) -> (Polymer, Rules) {
  let mut lines = input.trim().lines();
  let template = lines.next().unwrap().as_bytes();
  let mut rules = Rules::new();

  while let Some(line) = lines.next() {
    if line.len() > 0 {
      let (key, val) = parse_rule(line);
      rules.insert(key, val);
    }
  }
  return (template.to_vec(), rules);
}

fn find_middle(pair: [u8; 2], rules: &Rules) -> u8 {
  let key = u88_16(pair[0], pair[1]);
  let middle = rules.get(&key).unwrap();
  return *middle;
}

fn expand(list: &mut Polymer, rules: &Rules) {
  let mut i = 1;
  while i < list.len() {
    let middle = find_middle([list[i - 1], list[i]], rules);
    list.insert(i, middle);
    i += 2;
  }
}

fn occurences(list: Polymer) -> HashMap<u8, usize> {
  let mut counts = HashMap::new();
  for b in list {
    let c = counts.entry(b).or_insert(0);
    *c += 1;
  }
  return counts;
}

pub fn part1(input: &str) -> String {
  let (mut template, rules) = parse_template(input);
  println!("{:?}", String::from_utf8(template.clone()));
  for _ in 0..10 {
    expand(&mut template, &rules);
  }
  println!("{:?}", String::from_utf8(template.clone()));
  let counts = occurences(template);
  let max = counts.values().max().unwrap();
  let min = counts.values().min().unwrap();
  return format!("{}", max - min);
}

pub fn part2(_input: &str) -> String {
  return format!("0");
}

#[cfg(test)]
mod test {
  use super::*;

  const INPUT: &str = "\
NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C
";

  #[test]
  fn test_p1() {
    assert_eq!(&part1(INPUT), "1588");
  }

  #[test]
  fn test_p2() {
    assert_eq!(&part2(INPUT), "0");
  }
}
