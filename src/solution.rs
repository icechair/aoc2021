use std::collections::{HashMap, HashSet};
struct Graph {
  list: HashMap<String, Vec<String>>,
}

impl Graph {
  fn new() -> Self {
    let list = HashMap::new();
    return Self { list };
  }

  fn from(input: &str) -> Self {
    let mut graph = Graph::new();
    for line in input.trim().lines() {
      let (from, to) = line.trim().split_once('-').unwrap();
      graph.add(from, to);
    }

    return graph;
  }

  fn add_edge(&mut self, from: &str, to: &str) {
    self
      .list
      .entry(from.to_string())
      .or_insert_with(Vec::new)
      .push(to.to_string());
  }

  pub fn add(&mut self, a: &str, b: &str) {
    self.add_edge(a, b);
    self.add_edge(b, a);
  }

  pub fn neighbours(&self, node: &str) -> impl Iterator<Item = &String> {
    self.list[node].iter()
  }
}

fn walk_graph<'a>(graph: &'a Graph, node: &'a str, seen: &mut HashSet<&'a str>) -> usize {
  if seen.contains(&node) {
    return 0;
  }
  if node == "end" {
    return 1;
  }

  if node == node.to_lowercase() {
    seen.insert(node);
  }

  return graph
    .neighbours(node)
    .map(|node| walk_graph(graph, node, &mut seen.clone()))
    .sum();
}

fn walk_graph2<'a>(
  graph: &'a Graph,
  node: &'a str,
  seen: &mut HashSet<&'a str>,
  double: &mut bool,
) -> usize {
  if seen.contains(&node) {
    if *double {
      return 0;
    } else {
      *double = true;
    }
  }
  if node == "end" {
    return 1;
  }

  if node.chars().all(char::is_lowercase) {
    seen.insert(node);
  }

  return graph
    .neighbours(node)
    .filter(|&node| node != "start")
    .map(|node| walk_graph2(graph, node, &mut seen.clone(), &mut double.clone()))
    .sum();
}

pub fn part1(input: &str) -> String {
  let graph = Graph::from(input);
  let result = walk_graph(&graph, "start", &mut HashSet::new());
  return format!("{}", result);
}

pub fn part2(input: &str) -> String {
  let graph = Graph::from(input);
  let result = walk_graph2(&graph, "start", &mut HashSet::new(), &mut false);
  return format!("{}", result);
}

#[cfg(test)]
mod test {
  use super::*;

  const INPUT: &str = "\
start-A
start-b
A-c
A-b
b-d
A-end
b-end
";

  const INPUT_B: &str = "\
dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc
";

  const INPUT_C: &str = "\
fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW
  ";

  #[test]
  fn test_p1() {
    assert_eq!(&part1(INPUT), "10");
    assert_eq!(&part1(INPUT_B), "19");
    assert_eq!(&part1(INPUT_C), "226");
  }

  #[test]
  fn test_p2() {
    assert_eq!(&part2(INPUT), "36");
    assert_eq!(&part2(INPUT_B), "103");
    assert_eq!(&part2(INPUT_C), "3509");
  }
}
