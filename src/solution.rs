//1101 0010 1111 1110 0010 1000
//VVVT TTAA AAAB BBBB CCCC C

#[derive(Debug, PartialEq, Eq)]
struct BitReader<'a> {
  data: &'a [u8],
  byte: usize,
  bit: u8,
}

fn bit_reader<'a>(data: &'a [u8]) -> BitReader<'a> {
  let byte = 0;
  let bit = 0;
  return BitReader { data, byte, bit };
}

impl<'a> Iterator for BitReader<'a> {
  type Item = u8;
  fn next(&mut self) -> Option<Self::Item> {
    if self.byte == self.data.len() {
      return None;
    };
    let next = (self.data[self.byte] >> (3 - self.bit)) & 1;
    self.bit += 1;
    if self.bit >= 4 {
      self.bit = 0;
      self.byte += 1;
    }
    return Some(next & 1);
  }
}

fn parse_bits(s: &str) -> Vec<u8> {
  (0..s.len())
    .flat_map(|i| u8::from_str_radix(&s[i..i + 1], 16))
    .collect()
}

#[derive(Debug, PartialEq, Eq)]
enum PacketLength {
  BitLength(usize),
  PacketCount(usize),
}

#[derive(Debug, PartialEq, Eq)]
enum Packet {
  Literal {
    version: u8,
    id: u8,
    value: i64,
  },
  Operator {
    version: u8,
    id: u8,
    len: PacketLength,
    children: Vec<Packet>,
  },
}

fn parse_literal_value(reader: &mut dyn Iterator<Item = u8>) -> Result<(i64, usize), String> {
  let mut len = 0;
  let mut value = 0;
  let mut nibbles = Vec::new();
  while let Some(block_type) = reader.next() {
    println!("block_type: {}", block_type);
    let mut nibble = 0;
    for bit in 0..4 {
      if let Some(part) = reader.next() {
        nibble += part << 3 - bit;
      } else {
        return Err(format!("unexpected end of literal value bit stream"));
      }
    }
    nibbles.push(nibble);
    println!("nibble: {:04b}", nibble);
    len += 5;
    if block_type == 0 {
      break;
    }
  }
  for i in 0..nibbles.len() {
    value += (nibbles[i] as i64) << 4 * (nibbles.len() - 1 - i);
  }
  println!("parse_literal_value: Ok({}, {})", value, len);
  return Ok((value, len));
}

fn parse_packet_length(reader: &mut dyn Iterator<Item = u8>) -> Result<PacketLength, String> {
  match reader.next() {
    None => Err(format!("packet_length: EOF, no len_type")),
    Some(len_type) => match len_type {
      0 => {
        let mut len = 0;
        for i in 0..15 {
          match reader.next() {
            None => return Err(format!("packet_length: EOF, no bit_length_bit {}", i)),
            Some(bit) => len += (bit as usize) << (14 - i),
          }
        }
        println!("parse_packet_len: ok(bitlen({}))", len);
        return Ok(PacketLength::BitLength(len));
      }
      1 => {
        let mut len = 0;
        for i in 0..11 {
          match reader.next() {
            None => return Err(format!("packet_length: EOF, no pack_count_bit {}", i)),
            Some(bit) => len += (bit as usize) << (10 - i),
          }
        }
        println!("parse_packet_len: Ok(count({}))", len);
        return Ok(PacketLength::PacketCount(len));
      }
      _ => unreachable!("packet len: invalid iterator"),
    },
  }
}

fn take_bits(reader: &mut dyn Iterator<Item = u8>, len: u8) -> Result<u8, String> {
  let mut value = 0;
  for i in 0..len {
    match reader.next() {
      None => return Err(format!("take_bits: eof on bit {}", i)),
      Some(bit) => value += bit << (len - 1 - i),
    }
  }
  return Ok(value);
}

fn parse_packet(reader: &mut dyn Iterator<Item = u8>) -> Result<(Packet, usize), String> {
  let version = match take_bits(reader, 3) {
    Err(e) => return Err(format!("parse_packet: error version: {}", e)),
    Ok(v) => v,
  };
  let id = match take_bits(reader, 3) {
    Err(e) => return Err(format!("parse_packet: error id: {}", e)),
    Ok(v) => v,
  };

  println!("parse_packet: version: {}, id: {}", version, id);

  match id {
    4 => match parse_literal_value(reader) {
      Err(e) => return Err(e),
      Ok((value, len)) => return Ok((Packet::Literal { version, id, value }, 6 + len)),
    },
    _ => match parse_packet_length(reader) {
      Err(e) => {
        return Err(format!(
          "parse_packet({},{}): error length: {}",
          version, id, e
        ))
      }
      Ok(x) => match x {
        PacketLength::BitLength(len) => {
          let mut children = Vec::new();
          let mut counter = 0;
          while counter < len {
            match parse_packet(reader) {
              Err(e) => {
                return Err(format!(
                  "parse_packet({},{}): err_bit_child: {}",
                  version, id, e
                ))
              }
              Ok((child, len)) => {
                children.push(child);
                counter += len;
                println!(
                  "parse_packet_child({},{}): len: {}, count:{}",
                  version, id, len, counter
                );
              }
            }
          }
          return Ok((
            Packet::Operator {
              version,
              id,
              len: x,
              children,
            },
            7 + 15 + len,
          ));
        }
        PacketLength::PacketCount(count) => {
          let mut children = Vec::new();
          let mut len = 0;
          for _ in 0..count {
            match parse_packet(reader) {
              Err(e) => return Err(format!("parse_packet: err_packet_child: {}", e)),
              Ok((child, l)) => {
                children.push(child);
                len += l
              }
            }
          }
          return Ok((
            Packet::Operator {
              version,
              id,
              len: x,
              children,
            },
            7 + 11 + len,
          ));
        }
      },
    },
  };
}

fn version_sum(packet: &Packet) -> usize {
  println!("version_sum: {:?}", packet);
  match packet {
    Packet::Literal { version, .. } => *version as usize,
    Packet::Operator {
      version, children, ..
    } => children
      .iter()
      .fold(usize::from(*version), |acc, c| acc + version_sum(c)),
  }
}

pub fn part1(input: &str) -> String {
  let data = parse_bits(input);
  for d in &data {
    print!("{:04b} ", d);
  }
  println!("");
  let mut reader = bit_reader(&data).peekable();
  let mut version = 0;
  match parse_packet(&mut reader) {
    Err(e) => eprintln!("part1 err: {}", e),
    Ok((packet, _)) => version += version_sum(&packet),
  }
  return format!("{}", version);
}

pub fn part2(_input: &str) -> String {
  return format!("0");
}

#[cfg(test)]
mod test {
  use super::*;

  const INPUT: &str = "\
";

  #[test]
  fn test_bitreader() {
    let mut reader = bit_reader(&[0x13, 0x40, 0x0F]);
    assert_eq!(reader.next(), Some(0));
    assert_eq!(reader.next(), Some(0));
    assert_eq!(reader.next(), Some(0));
    assert_eq!(reader.next(), Some(1));

    assert_eq!(reader.next(), Some(0));
    assert_eq!(reader.next(), Some(0));
    assert_eq!(reader.next(), Some(1));
    assert_eq!(reader.next(), Some(1));

    assert_eq!(reader.next(), Some(0));
    assert_eq!(reader.next(), Some(1));
    assert_eq!(reader.next(), Some(0));
    assert_eq!(reader.next(), Some(0));

    assert_eq!(reader.next(), Some(0));
    assert_eq!(reader.next(), Some(0));
    assert_eq!(reader.next(), Some(0));
    assert_eq!(reader.next(), Some(0));

    assert_eq!(reader.next(), Some(0));
    assert_eq!(reader.next(), Some(0));
    assert_eq!(reader.next(), Some(0));
    assert_eq!(reader.next(), Some(0));

    assert_eq!(reader.next(), Some(1));
    assert_eq!(reader.next(), Some(1));
    assert_eq!(reader.next(), Some(1));
    assert_eq!(reader.next(), Some(1));

    assert_eq!(reader.next(), None);
  }

  #[test]
  fn test_literal_value() {
    let data = parse_bits("D2FE28");
    let reader = bit_reader(&data);
    assert_eq!(parse_literal_value(&mut reader.skip(6)), Ok((2021, 15)));

    let data = parse_bits("38006F45291200");
    let reader = bit_reader(&data);
    assert_eq!(
      parse_packet_length(&mut reader.skip(6)),
      Ok(PacketLength::BitLength(27))
    );

    let data = parse_bits("EE00D40C823060");
    let reader = bit_reader(&data);
    assert_eq!(
      parse_packet_length(&mut reader.skip(6)),
      Ok(PacketLength::PacketCount(3))
    );
  }

  #[test]
  fn test_p1() {
    //assert_eq!(&part1("8A004A801A8002F478"), "16");
    //assert_eq!(&part1("620080001611562C8802118E34"), "12");
    assert_eq!(&part1("C0015000016115A2E0802F182340"), "23");
    assert_eq!(&part1("A0016C880162017C3686B18A3D4780"), "31");
  }

  #[test]
  fn test_p2() {
    assert_eq!(&part2(INPUT), "0");
  }
}
