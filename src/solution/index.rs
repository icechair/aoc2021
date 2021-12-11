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
) -> impl Iterator<Item = (usize, &T)> + '_
where
  T: Debug,
{
  let (row, col) = row_col(id, width);

  let mut nexts: Vec<(usize, &T)> = Vec::new();
  if row > 0 {
    let up = index(row - 1, col, width);
    nexts.push((up, &list[up]));
  }
  if col > 0 {
    let left = index(row, col - 1, width);
    nexts.push((left, &list[left]));
  }
  if col + 1 < width {
    let right = index(row, col + 1, width);
    nexts.push((right, &list[right]));
  }
  if row + 1 < height {
    let down = index(row + 1, col, width);
    nexts.push((down, &list[down]));
  }
  return nexts.into_iter();
}
