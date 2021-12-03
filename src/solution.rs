const BASE: i64 = 2;
fn parse_diagnostic(input: &str) -> Vec<&str> {
  return input.trim().lines().collect();
}

fn get_bitcount(list: &[&str]) -> Vec<i64> {
  let mut bitcounts = Vec::new();
  for bin in list {
    if bitcounts.len() == 0 {
      for _ in 0..bin.len() {
        bitcounts.push(0)
      }
    }
    for (idx, bit) in bin.chars().enumerate() {
      if bit == '1' {
        bitcounts[idx] += 1;
      }
    }
  }
  return bitcounts;
}

fn get_bit_counts(list: &[&str], start: &str) -> (usize, usize) {
  let mut one = String::from(start);
  one.push('1');
  let ones = list.iter().filter(|x| x.starts_with(&one)).count();

  let mut zero = String::from(start);
  zero.push('0');
  let zeros = list.iter().filter(|x| x.starts_with(&zero)).count();
  return (ones, zeros);
}

fn find_oxygen(list: &mut Vec<&str>, bin: &mut String, use_carbon: bool) {
  //println!("oxygen-start:carbon ? {} {:?}", use_carbon, list);
  //println!("oxygen-start: {:?}", bin);
  if list.len() == 0 {
    panic!("oxygen: no oxygen value, shoud not happen!");
  }
  if list.len() == 1 {
    bin.clear();
    bin.push_str(list[0]);
    return;
  }

  let (ones, zeros) = get_bit_counts(list, bin);
  //println!("oxygen: {}, {}", ones, zeros);
  if use_carbon {
    if ones < zeros {
      bin.push('1');
    } else {
      bin.push('0');
    }
  } else {
    if zeros > ones {
      bin.push('0')
    } else {
      bin.push('1')
    }
  }
  //println!("oxygen: {}", bin);
  list.retain(|x| x.starts_with(&bin.clone()));
  find_oxygen(list, bin, use_carbon);
}

pub fn part1(input: &str) -> String {
  let values = parse_diagnostic(input);
  let bitcounts = get_bitcount(&values);
  let mut gamma = 0;
  let mut epsilon = 0;
  let amount = values.len() as i64;
  //println!("{:?}", values);
  for (bit, &count) in bitcounts.iter().enumerate() {
    //println!("{}, {}, {}, {}", bit, count, amount, count > amount);
    if count > amount / 2 {
      gamma += 1 << (bitcounts.len() - 1 - bit); // BASE.pow((bitcounts.len() - 1 - bit) as u32)
    } else {
      epsilon += 1 << (bitcounts.len() - 1 - bit); // BASE.pow((bitcounts.len() - 1 - bit) as u32)
    }
  }

  return format!("{}", gamma * epsilon);
}

pub fn part2(input: &str) -> String {
  let values = parse_diagnostic(input);
  let mut oxygen = String::from("");
  find_oxygen(&mut values.clone(), &mut oxygen, false);
  let mut carbon = String::from("");
  find_oxygen(&mut values.clone(), &mut carbon, true);

  let oxygen = i64::from_str_radix(&oxygen, 2).unwrap_or(1);
  let carbon = i64::from_str_radix(&carbon, 2).unwrap_or(1);

  return format!("{}", oxygen * carbon);
}

#[cfg(test)]
mod test {
  use super::*;
  #[test]
  fn test_p1() {
    let input = "\
      00100\n\
      11110\n\
      10110\n\
      10111\n\
      10101\n\
      01111\n\
      00111\n\
      11100\n\
      10000\n\
      11001\n\
      00010\n\
      01010\n";
    assert_eq!(&part1(input), "198");
  }

  #[test]
  fn test_p2() {
    let input = "\
      00100\n\
      11110\n\
      10110\n\
      10111\n\
      10101\n\
      01111\n\
      00111\n\
      11100\n\
      10000\n\
      11001\n\
      00010\n\
      01010\n";
    assert_eq!(&part2(input), "230");
  }
}
/*
00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010



11110
01111

*/
