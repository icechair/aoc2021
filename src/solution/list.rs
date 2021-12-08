pub fn drain_filter<T, F>(list: &mut Vec<T>, f: F) -> Vec<T>
where
  F: Fn(&T) -> bool,
{
  let mut out = Vec::with_capacity(list.capacity());
  let mut i = 0;
  while i < list.len() {
    if f(&list[i]) {
      let val = list.remove(i);
      out.push(val);
    } else {
      i += 1;
    }
  }
  return out;
}
