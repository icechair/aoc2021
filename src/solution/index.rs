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

  let mut nexts: Vec<usize> = Vec::new();
  if row > 0 {
    //up
    nexts.push(index(row - 1, col, width));
  }
  if col > 0 {
    //left
    nexts.push(index(row, col - 1, width));
  }
  let right = index(row, col + 1, width);
  if right < list.len() {
    nexts.push(right);
  }
  let down = index(row + 1, col, width);
  if down < list.len() {
    nexts.push(down);
  }
  return list.iter().enumerate().filter(move |(i, _)| {
    return *i != id && nexts.contains(i);
  });
}
