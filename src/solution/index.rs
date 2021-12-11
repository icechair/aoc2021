use std::fmt::Debug;
pub fn index(row: usize, col: usize, width: usize) -> usize {
  row * width + col
}

pub fn row_col(index: usize, width: usize) -> (usize, usize) {
  let row = index / width;
  let col = index % width;
  return (row, col);
}

pub fn neighbours<T>(
  list: &[T],
  id: usize,
  width: usize,
  height: usize,
  with_diagonals: bool,
) -> impl Iterator<Item = usize> + '_
where
  T: Debug,
{
  let (row, col) = row_col(id, width);

  let mut nexts: Vec<usize> = Vec::new();
  if row > 0 {
    let up = index(row - 1, col, width);
    nexts.push(up);
  }
  if col > 0 {
    let left = index(row, col - 1, width);
    nexts.push(left);
  }
  if col + 1 < width {
    let right = index(row, col + 1, width);
    nexts.push(right);
  }
  if row + 1 < height {
    let down = index(row + 1, col, width);
    nexts.push(down);
  }

  if with_diagonals {
    if row > 0 {
      if col > 0 {
        let up_left = index(row - 1, col - 1, width);
        nexts.push(up_left);
      }
      if col + 1 < width {
        let up_right = index(row - 1, col + 1, width);
        nexts.push(up_right);
      }
    }
    if row + 1 < height {
      if col > 0 {
        let down_left = index(row + 1, col - 1, width);
        nexts.push(down_left);
      }
      if col + 1 < width {
        let down_right = index(row + 1, col + 1, width);
        nexts.push(down_right);
      }
    }
  }

  return nexts.into_iter();
}

pub fn print_map<T>(map: &[T], width: usize, height: usize)
where
  T: Debug,
{
  let mut out = String::new();
  for row in 0..height {
    for col in 0..width {
      let index = index(row, col, width);
      out.push_str(&format!("{:?}", map[index]))
    }
    out.push('\n');
  }
  println!("{}", out.trim_end())
}
