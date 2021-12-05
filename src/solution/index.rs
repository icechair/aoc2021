pub fn index(row: usize, col: usize, width: usize) -> usize {
  row * width + col
}

pub fn row_col(index: usize, width: usize, height: usize) -> (usize, usize) {
  let row = index / height;
  let col = index % width;
  return (row, col);
}
