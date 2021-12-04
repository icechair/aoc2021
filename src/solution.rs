use std::fmt;
use std::str::Lines;
const BOARD_SIZE: usize = 5;
fn index(row: usize, col: usize) -> usize {
  row * BOARD_SIZE + col
}
fn row_col(index: usize) -> (usize, usize) {
  let row = index / BOARD_SIZE;
  let col = index % BOARD_SIZE;
  return (row, col);
}
#[derive(Debug)]
struct Field {
  value: i64,
  marked: bool,
}

struct Bingo {
  board: Vec<Field>,
}

impl fmt::Debug for Bingo {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let mut out = String::new();
    for row in 0..BOARD_SIZE {
      for col in 0..BOARD_SIZE {
        let id = index(row, col);
        let field = &self.board[id];
        out.push_str(&match field.marked {
          true => format!("[{}]\t", field.value),
          false => format!(" {} \t", field.value),
        })
      }
      out.push('\n');
    }
    write!(f, "{}", out.trim_end())
  }
}

impl Bingo {
  fn from(lines: &mut Lines) -> Bingo {
    let mut board = Vec::with_capacity(BOARD_SIZE * BOARD_SIZE);
    for _ in 0..BOARD_SIZE {
      let line = lines.next().expect("lines shoud not be finished");
      for (_, num) in line.trim().split_ascii_whitespace().enumerate() {
        let value = num.parse::<i64>().expect("num shoud be a number");
        let marked = false;
        board.push(Field { value, marked })
      }
    }
    return Bingo { board };
  }

  fn check_bingo(&self, s_id: usize) -> bool {
    let (s_row, s_col) = row_col(s_id);
    let mut row_marked = 0;
    let mut col_marked = 0;
    for (id, field) in self.board.iter().enumerate() {
      let (row, col) = row_col(id);
      if row == s_row && field.marked {
        row_marked += 1;
      }
      if col == s_col && field.marked {
        col_marked += 1;
      }
    }
    return col_marked == BOARD_SIZE || row_marked == BOARD_SIZE;
  }

  fn mark(&mut self, value: i64) -> bool {
    for (idx, field) in self.board.iter_mut().enumerate() {
      if field.value == value {
        field.marked = true;
        return self.check_bingo(idx);
      }
    }
    return false;
  }

  fn score(&self) -> i64 {
    let mut score = 0;
    for field in &self.board {
      if field.marked == false {
        score += field.value;
      }
    }
    return score;
  }
}

pub fn part1(input: &str) -> String {
  let mut lines = input.trim().lines();
  let raffle: Vec<i64> = lines
    .next()
    .expect("raffle shoud not be empty")
    .split(",")
    .flat_map(|x| x.parse::<i64>())
    .collect();
  let mut boards: Vec<Bingo> = Vec::new();
  while let Some(_) = lines.next() {
    boards.push(Bingo::from(&mut lines));
  }

  for (rid, &value) in raffle.iter().enumerate() {
    println!("raffle-{}: {}", rid, value);
    for (bid, board) in boards.iter_mut().enumerate() {
      if board.mark(value) {
        return format!("{}", board.score() * value);
      }
      println!("board-{}:\n{:?}", bid, board);
    }
  }
  return format!("{}", 0);
}

pub fn part2(_input: &str) -> String {
  format!("{}", 0)
}

#[cfg(test)]
mod test {
  use super::*;

  const input: &str = "\
7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7
";

  #[test]
  fn test_p1() {
    assert_eq!(&part1(input), "4512");
  }

  #[test]
  fn test_p2() {}
}
