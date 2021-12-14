use std::collections::HashMap;

type Polymer = Vec<u8>;
type Pair = (u8, u8);
type Rule = (Pair, u8);

fn parse_rule(line: &str) -> Rule {
  let (from, to) = line.trim().split_once(" -> ").unwrap();
  let from = from.as_bytes();
  let from = (from[0], from[1]);
  let to = to.as_bytes();
  let to = to[0];
  return (from, to);
}

fn parse_template(input: &str) -> (Polymer, HashMap<Pair, u8>) {
  let mut lines = input.trim().lines();
  let template = lines.next().unwrap().as_bytes();
  let mut rules = HashMap::new();

  while let Some(line) = lines.next() {
    if line.len() > 0 {
      let (key, val) = parse_rule(line);
      rules.insert(key, val);
    }
  }

  for win in template.windows(2) {}

  return (template.to_vec(), rules);
}

fn create_counts(template: &Polymer) -> (HashMap<Pair, usize>, HashMap<u8, usize>) {
  let mut pair_counts = HashMap::new();
  let mut elem_counts = HashMap::new();
  for win in template.windows(2) {
    pair_counts
      .entry((win[0], win[1]))
      .and_modify(|c| *c += 1)
      .or_insert(1);
    elem_counts
      .entry(win[0])
      .and_modify(|c| *c += 1)
      .or_insert(0);
  }
  elem_counts
    .entry(*template.last().unwrap())
    .and_modify(|c| *c += 1)
    .or_insert(1);

  return (pair_counts, elem_counts);
}

fn expand(
  pair_counts: &mut HashMap<Pair, usize>,
  elem_counts: &mut HashMap<u8, usize>,
  rules: &HashMap<Pair, u8>,
) {
  let mut next_pair_counts = pair_counts.clone();
  for (pair, elem) in rules {
    if let Some(pc) = pair_counts.get(pair) {
      next_pair_counts.entry(*pair).and_modify(|c| *c -= *pc);
      elem_counts
        .entry(*elem)
        .and_modify(|c| *c += *pc)
        .or_insert(*pc);
      next_pair_counts
        .entry((pair.0, *elem))
        .and_modify(|c| *c += *pc)
        .or_insert(*pc);
      next_pair_counts
        .entry((*elem, pair.1))
        .and_modify(|c| *c += *pc)
        .or_insert(*pc);
    };
  }
  *pair_counts = next_pair_counts;
}

pub fn part1(input: &str) -> String {
  let (template, rules) = parse_template(input);
  let (mut pair_counts, mut elem_counts) = create_counts(&template);
  for turn in 0..10 {
    expand(&mut pair_counts, &mut elem_counts, &rules);
    println!("turn {} -> len: {}", turn, template.len());
  }
  let max = elem_counts.values().max().unwrap();
  let min = elem_counts.values().min().unwrap();
  return format!("{}", max - min);
}

pub fn part2(input: &str) -> String {
  let (template, rules) = parse_template(input);
  let (mut pair_counts, mut elem_counts) = create_counts(&template);
  for turn in 0..40 {
    expand(&mut pair_counts, &mut elem_counts, &rules);
    println!("turn {} -> len: {}", turn, template.len());
  }
  let max = elem_counts.values().max().unwrap();
  let min = elem_counts.values().min().unwrap();
  return format!("{}", max - min);
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
    assert_eq!(&part2(INPUT), "2188189693529");
  }
}
