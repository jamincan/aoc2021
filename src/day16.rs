const INPUT: &str = include_str!("./assets/day16.txt");

pub fn solve() -> String {
    let input = INPUT.lines().next().unwrap();
    let packets = parse::parse(input).unwrap();
    let part2 = part2(&packets);
    let part1 = part1(packets);
    format!("{part1}, {part2}")
}

fn part1(mut packets: Vec<Packet>) -> usize {
    let mut total = 0;
    while let Some(packet) = packets.pop() {
        match packet {
            Packet::Literal { version, value: _ } => total += version as usize,
            Packet::Operator {
                version,
                op: _,
                packets: sub_packets,
            } => {
                packets.extend(sub_packets.into_iter());
                total += version as usize;
            }
        }
    }
    total
}

fn part2(packets: &[Packet]) -> usize {
    assert_eq!(packets.len(), 1);
    packets[0].evaluate()
}

#[derive(Debug, Clone, PartialEq)]
enum Packet {
    Literal {
        version: u8,
        value: Vec<u8>,
    },
    Operator {
        version: u8,
        op: Operation,
        packets: Vec<Packet>,
    },
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Operation {
    Sum,
    Product,
    Min,
    Max,
    Greater,
    Less,
    Equal,
}

impl Packet {
    fn evaluate(&self) -> usize {
        match self {
            Packet::Literal { version: _, value } => bytes_to_usize(value),
            Packet::Operator {
                version: _,
                op,
                packets,
            } => {
                let values: Vec<_> = packets.iter().map(|packet| packet.evaluate()).collect();
                match op {
                    Operation::Sum => values.iter().sum(),
                    Operation::Product => values.iter().product(),
                    Operation::Min => *values.iter().min().unwrap(),
                    Operation::Max => *values.iter().max().unwrap(),
                    Operation::Greater => {
                        debug_assert_eq!(values.len(), 2);
                        if values[0] > values[1] {
                            1
                        } else {
                            0
                        }
                    }
                    Operation::Less => {
                        debug_assert_eq!(values.len(), 2);
                        if values[0] < values[1] {
                            1
                        } else {
                            0
                        }
                    }
                    Operation::Equal => {
                        debug_assert_eq!(values.len(), 2);
                        if values[0] == values[1] {
                            1
                        } else {
                            0
                        }
                    }
                }
            }
        }
    }
}

fn bytes_to_usize(input: &[u8]) -> usize {
    debug_assert!(input.len() <= 8);
    let mut bytes = [0; 8];
    let (first, _) = bytes.split_at_mut(input.len());
    first.copy_from_slice(input);
    usize::from_le_bytes(bytes)
}

mod parse {
    use super::{Operation, Packet};
    use crate::bits::Bits;
    use anyhow::{bail, Context, Result};
    use itertools::Itertools;

    pub(super) fn parse(input: &str) -> Result<Vec<Packet>> {
        let bytes: Vec<_> = (0..input.len())
            .step_by(2)
            .map(|index| u8::from_str_radix(&input[index..index + 2], 16).unwrap())
            .collect();
        let bits = Bits::new(&bytes);
        packets(bits)
    }

    fn packets(mut input: Bits) -> Result<Vec<Packet>> {
        let mut packets = Vec::new();
        while let Ok((temp_input, packet)) = packet(input) {
            input = temp_input;
            packets.push(packet);
        }
        Ok(packets)
    }

    fn packet(input: Bits) -> Result<(Bits, Packet)> {
        let res = literal(input)
            .or_else(|_| operator(input))
            .context("Packet not found.");
        res
    }

    fn literal(input: Bits) -> Result<(Bits, Packet)> {
        let (input, version) = version_tag(input).context("Literal not found.")?;
        let input = literal_type(input).context("Literal not found.")?;
        let (input, value) = literal_value(input).context("Literal not found.")?;
        Ok((input, Packet::Literal { version, value }))
    }

    fn operator(input: Bits) -> Result<(Bits, Packet)> {
        let (input, version) = version_tag(input).context("Operator not found.")?;
        let (input, variant) = operator_type(input).context("Operator not found.")?;
        let (input, packets) = total_length_op_data(input)
            .or_else(|_| count_op_data(input))
            .context("Operator sub-packets not found.")?;
        Ok((
            input,
            Packet::Operator {
                version,
                op: variant,
                packets,
            },
        ))
    }

    fn version_tag(mut input: Bits) -> Result<(Bits, u8)> {
        let tag = input.take(3).context("Version tag not found.")?[0];
        Ok((input, tag))
    }

    fn literal_type(mut input: Bits) -> Result<Bits> {
        match input.take(3).context("Literal tag not found.")?[0] {
            4 => Ok(input),
            tag => bail!("Literal tag not found. '{tag}' found instead."),
        }
    }

    fn operator_type(mut input: Bits) -> Result<(Bits, Operation)> {
        let tag = match input.take(3).context("Operator tag not found.")?[0] {
            4 => bail!("Operator tag not found. Literal tag found instead."),
            0 => Operation::Sum,
            1 => Operation::Product,
            2 => Operation::Min,
            3 => Operation::Max,
            5 => Operation::Greater,
            6 => Operation::Less,
            7 => Operation::Equal,
            _ => unreachable!(),
        };
        Ok((input, tag))
    }

    // Collect input in 5 bit chunks, ending when the 1st bit is no longer set.
    // Stored in little-endian order.
    fn literal_value(mut input: Bits) -> Result<(Bits, Vec<u8>)> {
        let mut partial_bytes = Vec::new();
        loop {
            let tag = input.take(1).context("Literal value not found.")?[0];
            let data = input.take(4).context("Literal value not found.")?[0];
            partial_bytes.push(data);
            if tag == 0 {
                break;
            }
        }
        let bytes: Vec<u8> = partial_bytes
            .into_iter()
            .rev()
            .chunks(2)
            .into_iter()
            .map(|mut chunk| {
                let second = chunk.next().unwrap();
                let first = chunk.next().unwrap_or_default();
                first << 4 | second
            })
            .collect();

        Ok((input, bytes))
    }

    fn total_length_op_data(mut input: Bits) -> Result<(Bits, Vec<Packet>)> {
        use std::convert::TryInto;
        let tag = input.take(1).context("Length tag not found.")?[0];
        if tag != 0 {
            bail!("Length tag not found.")
        }
        let length: [u8; 2] = input
            .take(15)
            .context("Length tag not found.")?
            .try_into()
            .expect("take(15) returns 2 bytes.");
        let length = u16::from_be_bytes(length) as usize;

        // Counts the difference in length between the two Bits in bits
        fn bit_count(start: Bits, end: Bits) -> usize {
            let (start_len, start_offset) = start.len();
            let (end_len, end_offset) = end.len();
            let total_bits_diff = (start_len - end_len) * 8;
            total_bits_diff - start_offset as usize + end_offset as usize
        }

        let mut running_input = input;
        let mut sub_packets = Vec::new();
        while let Ok((temp_input, packet)) = packet(running_input) {
            let sub_packets_bits = bit_count(input, temp_input);
            if sub_packets_bits > length {
                break;
            } else {
                running_input = temp_input;
                sub_packets.push(packet);
            }
        }

        let padding_bit_count = length - bit_count(input, running_input);
        let _ = running_input
            .take(padding_bit_count)
            .with_context(|| format!("{padding_bit_count} bit(s) expected as padding."))?;

        Ok((running_input, sub_packets))
    }

    fn count_op_data(mut input: Bits) -> Result<(Bits, Vec<Packet>)> {
        use std::convert::TryInto;
        let tag = input.take(1).context("Count tag not found.")?[0];
        if tag != 1 {
            bail!("Count tag not found.")
        }
        let count: [u8; 2] = input
            .take(11)
            .context("Count value not found.")?
            .try_into()
            .expect("take(11) returns 2 bytes.");
        let count = u16::from_be_bytes(count);
        let mut sub_packets = Vec::with_capacity(count as usize);
        while count as usize - sub_packets.len() > 0 {
            let (temp_input, packet) = packet(input).with_context(|| {
                let sub_packet_count = sub_packets.len();
                format!("{count} packet(s) expected, but {sub_packet_count} packet(s) found.")
            })?;
            sub_packets.push(packet);
            input = temp_input;
        }
        Ok((input, sub_packets))
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn test_version_tag() {
            let bits = Bits::new(&[0b11100011]);
            let (bits, tag) = version_tag(bits).unwrap();
            assert_eq!(bits.len(), (1, 3));
            assert_eq!(tag, 0b00000111);
            let (bits, tag) = version_tag(bits).unwrap();
            assert_eq!(bits.len(), (1, 6));
            assert_eq!(tag, 0b00000000);
            assert!(version_tag(bits).is_err());
        }
        #[test]
        fn test_literal_tag() {
            let bits = Bits::new(&[0b10000000]);
            let bits = literal_type(bits).unwrap();
            assert!(literal_type(bits).is_err())
        }
        #[test]
        fn test_literal_value() {
            let bits = Bits::new(&[0b11000100, 0b00000000]);
            let (_, lit) = literal_value(bits).unwrap();
            assert_eq!(lit, vec![0b00000000, 0b00001000]);
        }
    }
}
#[cfg(test)]
mod tests {
    use super::parse::parse;
    use super::*;

    macro_rules! assert_literal {
        ($expr:expr, |$version:pat, $value:pat| $test:block) => {
            if let Packet::Literal {
                version: $version,
                value: $value,
            } = $expr
            {
                $test
            } else {
                panic!("tried to match on `Packet::Literal`, but variant not found")
            }
        };
    }
    macro_rules! assert_operator {
        ($expr:expr, |$version:pat, $op:pat, $packets:pat| $test:block) => {
            if let Packet::Operator {
                version: $version,
                op: $op,
                packets: $packets,
            } = $expr
            {
                $test
            } else {
                panic!("tried to match on `Packet::Operator`, but variant not found")
            }
        };
    }

    #[test]
    fn test_example_1() {
        let actual = parse("D2FE28").unwrap();
        assert_literal!(&actual[0], |6, value| {
            assert_eq!(*value, 2021u16.to_le_bytes().to_vec());
        });
    }
    #[test]
    fn test_example_2() {
        let actual = parse("38006F45291200").unwrap();
        assert_operator!(&actual[0], |1, Operation::Less, packets| {
            assert_literal!(&packets[0], |_, value| {
                assert_eq!(*value, vec![10]);
            });
            assert_literal!(&packets[1], |_, value| {
                assert_eq!(*value, vec![20]);
            });
        })
    }
    #[test]
    fn test_example_3() {
        let actual = parse("EE00D40C823060").unwrap();
        assert_operator!(&actual[0], |7, Operation::Max, packets| {
            assert_literal!(&packets[0], |_, value| {
                assert_eq!(*value, vec![1]);
            });
            assert_literal!(&packets[1], |_, value| {
                assert_eq!(*value, vec![2]);
            });
            assert_literal!(&packets[2], |_, value| {
                assert_eq!(*value, vec![3]);
            })
        })
    }
    #[test]
    fn test_final_examples() {
        assert_eq!(part1(parse("8A004A801A8002F478").unwrap()), 16);
        assert_eq!(part1(parse("620080001611562C8802118E34").unwrap()), 12);
        assert_eq!(part1(parse("C0015000016115A2E0802F182340").unwrap()), 23);
        assert_eq!(part1(parse("A0016C880162017C3686B18A3D4780").unwrap()), 31);
        assert_eq!(part2(&parse("C200B40A82").unwrap()), 3);
        assert_eq!(part2(&parse("04005AC33890").unwrap()), 54);
        assert_eq!(part2(&parse("880086C3E88112").unwrap()), 7);
        assert_eq!(part2(&parse("CE00C43D881120").unwrap()), 9);
        assert_eq!(part2(&parse("D8005AC2A8F0").unwrap()), 1);
        assert_eq!(part2(&parse("F600BC2D8F").unwrap()), 0);
        assert_eq!(part2(&parse("9C005AC2F8F0").unwrap()), 0);
        assert_eq!(part2(&parse("9C0141080250320F1802104A08").unwrap()), 1);
    }
}
