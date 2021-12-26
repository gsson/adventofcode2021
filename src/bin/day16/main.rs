#![feature(array_windows)]

use adventofcode2021::*;
use std::cmp::Ordering;
use std::collections::VecDeque;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = Input::from_file("src/bin/day16/input.txt");
    let input = parse(input);
    let a = part1::solve(&input);
    eprintln!("Part 1: {}", a);
    assert_eq!(1012, a);
    let a = part2::solve(&input);
    eprintln!("Part 2: {}", a);
    assert_eq!(2223947372407, a);
    Ok(())
}

#[inline]
const fn index_offset(position: usize) -> (usize, u32) {
    let index = position >> 5;
    let offset = (position & 31) as u32;
    (index, offset)
}

#[derive(Default)]
struct BitStream {
    head: usize,
    tail: usize,
    bits: VecDeque<u32>,
}

impl BitStream {
    pub fn substream(&mut self, length: u32) -> BitStream {
        debug_assert!(self.tail - self.head >= length as usize);
        let tail = self.head + length as usize;
        let (from_index, from_offset) = index_offset(self.head);
        let (to_index, to_offset) = index_offset(tail);
        let mut bits = VecDeque::with_capacity(to_index - from_index + 1);
        bits.extend(self.bits.range(from_index..=to_index));

        let s = BitStream {
            head: from_offset as usize,
            tail: (to_index - from_index) * 32 + to_offset as usize,
            bits,
        };

        self.head += length as usize;
        s
    }

    pub fn pop<const BITS: u32>(&mut self) -> u32 {
        debug_assert!(self.tail - self.head >= BITS as usize);
        let index = self.head >> 5;
        let offset = (self.head & 31) as u32;

        let value = if offset > 32 - BITS {
            let low = self.bits[index] >> offset;
            let high_mask = u32::MAX >> (2 * u32::BITS - BITS - offset);
            let high = (self.bits[index + 1] & high_mask) << (u32::BITS - offset);
            low | high
        } else {
            self.bits[index] >> offset
        };
        let mask = (1 << BITS) - 1;

        self.head += BITS as usize;

        value & mask
    }

    pub fn len(&self) -> usize {
        self.tail - self.head
    }

    pub fn push4(&mut self, bits: u32) {
        let index = self.tail >> 5;
        let offset = self.tail & 31;
        if index == self.bits.len() {
            self.bits.push_back(0);
        }
        self.bits[index] |= bits << offset;
        self.tail += 4;
    }
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
enum PacketType {
    Sum,
    Product,
    Minimum,
    Maximum,
    Literal,
    GreaterThan,
    LessThan,
    Equal,
}

impl PacketType {
    fn from_id(id: u32) -> Self {
        match id {
            0 => PacketType::Sum,
            1 => PacketType::Product,
            2 => PacketType::Minimum,
            3 => PacketType::Maximum,
            4 => PacketType::Literal,
            5 => PacketType::GreaterThan,
            6 => PacketType::LessThan,
            7 => PacketType::Equal,
            _ => unreachable!("Invalid packet type {}", id),
        }
    }
}

#[allow(clippy::type_complexity)]
fn parse<R: std::io::BufRead>(input: Input<R>) -> Vec<u8> {
    input
        .bytes()
        .take_while(|b| *b != b'\n')
        .map(|b| match b {
            b'0'..=b'9' => b - b'0',
            b'A'..=b'F' => b - b'A' + 10,
            _ => unreachable!("{}", b),
        })
        .collect()
}

#[test]
fn test_bitstream() {
    let mut bits = BitStream::default();

    bits.push4(0xf); // 1111 tail = 4
    assert_eq!(4, bits.len());
    assert_eq!(0x7, bits.pop::<3>()); // 1
    assert_eq!(1, bits.len());
    bits.push4(0); // 00001 tail = 8
    assert_eq!(5, bits.len());
    assert_eq!(0x1, bits.pop::<3>()); // 00
    assert_eq!(2, bits.len());
    bits.push4(0xf); // 111100 tail = 12
    assert_eq!(6, bits.len());
    assert_eq!(0x4, bits.pop::<3>()); // 111
    assert_eq!(3, bits.len());
    assert_eq!(0x7, bits.pop::<3>()); // -
    assert_eq!(0, bits.len());

    bits.push4(0xf); // 1111 tail = 16
    bits.push4(0x0); // 00001111  tail = 20
    bits.push4(0xf); // 111100001111  tail = 24
    bits.push4(0x0); // 0000111100001111  tail = 28
    bits.push4(0xf); // 11110000111100001111  tail = 32
    bits.push4(0xa); // 1010.11110000111100001111 tail = 36
    assert_eq!(0x7, bits.pop::<3>()); // 1010.11110000111100001 | 111
    assert_eq!(0x1, bits.pop::<3>()); // 1010.11110000111100 | 001
    assert_eq!(0x4, bits.pop::<3>()); // 1010.11110000111 | 100
    assert_eq!(0x7, bits.pop::<3>()); // 1010.11110000 | 111
    assert_eq!(0x0, bits.pop::<3>()); // 1010.11110 | 000
    assert_eq!(0x6, bits.pop::<3>()); // 1010.11 | 110
    assert_eq!(0x3, bits.pop::<3>()); // 101 | 0.11
    assert_eq!(3, bits.len());
    assert_eq!(0x5, bits.pop::<3>()); // - | 101
    assert_eq!(0, bits.len());

    bits.push4(0xf); // 1111 tail = 4
    bits.push4(0x0); // 00001111 tail = 8
    bits.push4(0xf); // 111100001111 tail = 12
    bits.push4(0xf); // 1111111100001111 tail = 16
    bits.push4(0x0); // 00001111111100001111  tail = 20
    bits.push4(0xf); // 111100001111111100001111  tail = 24
    bits.push4(0x0); // 0000111100001111111100001111  tail = 28
    bits.push4(0xf); // 11110000111100001111111100001111  tail = 32
    bits.push4(0xa); // 1010.11110000111100001111111100001111 tail = 36
    assert_eq!(0b01111, bits.pop::<5>()); // 1010.111100001111000011111111000 | 01111
    assert_eq!(0b11000, bits.pop::<5>()); // 1010.1111000011110000111111 | 11000
    assert_eq!(0b11111, bits.pop::<5>()); // 1010.11110000111100001 | 11111
    assert_eq!(0b00001, bits.pop::<5>()); // 1010.111100001111 | 00001
    assert_eq!(0b01111, bits.pop::<5>()); // 1010.1111000 | 01111
    assert_eq!(0b11000, bits.pop::<5>()); // 1010.11 | 11000
    assert_eq!(0b01011, bits.pop::<5>()); // 1 | 010.11
    assert_eq!(1, bits.len());
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
enum PacketPayload {
    Literal(u64),
    Operator(Vec<(u32, PacketType, PacketPayload)>),
}

impl PacketPayload {
    fn literal(&self) -> u64 {
        if let PacketPayload::Literal(v) = self {
            *v
        } else {
            unreachable!()
        }
    }
    fn operator_args(&self) -> &[(u32, PacketType, PacketPayload)] {
        if let PacketPayload::Operator(v) = self {
            v
        } else {
            unreachable!()
        }
    }
}

fn parse_packets(bits: &mut BitStream) -> Vec<(u32, PacketType, PacketPayload)> {
    std::iter::from_fn(|| parse_packet(bits)).collect()
}

fn parse_packet(bits: &mut BitStream) -> Option<(u32, PacketType, PacketPayload)> {
    if bits.len() < 6 {
        None
    } else {
        let version = bits.pop::<3>().reverse_bits() >> 29;
        let type_id = bits.pop::<3>().reverse_bits() >> 29;
        let packet_type = PacketType::from_id(type_id);
        let payload = match packet_type {
            PacketType::Literal => {
                let mut v = 0u64;
                while bits.pop::<1>() == 1 {
                    v = (v << 4) | (bits.pop::<4>().reverse_bits() >> 28) as u64;
                }
                v = (v << 4) | (bits.pop::<4>().reverse_bits() >> 28) as u64;
                PacketPayload::Literal(v)
            }
            _ => {
                let fixed_length = bits.pop::<1>() == 1;
                if fixed_length {
                    let packet_count = reverse::<11>(bits.pop::<11>());
                    let packets = (0..packet_count)
                        .map(|_| parse_packet(bits).unwrap())
                        .collect::<Vec<_>>();
                    PacketPayload::Operator(packets)
                } else {
                    let length = reverse::<15>(bits.pop::<15>());
                    let mut bits = bits.substream(length);
                    PacketPayload::Operator(parse_packets(&mut bits))
                }
            }
        };
        Some((version, packet_type, payload))
    }
}

fn reverse<const BITS: u32>(bits: u32) -> u32 {
    bits.reverse_bits() >> (u32::BITS - BITS)
}

fn version_sum((version, _, payload): &(u32, PacketType, PacketPayload)) -> u32 {
    version
        + match payload {
            PacketPayload::Operator(v) => v.iter().fold(0, |s, p| s + version_sum(p)),
            _ => 0,
        }
}

fn evaluate_packets(payload: &PacketPayload) -> impl Iterator<Item = u64> + '_ {
    payload.operator_args().iter().map(execute)
}

fn evaluate_tuple(payload: &PacketPayload) -> (u64, u64) {
    if let [a, b] = payload.operator_args() {
        (execute(a), execute(b))
    } else {
        unreachable!()
    }
}

fn evaluate_compare(payload: &PacketPayload) -> Ordering {
    let (a, b) = evaluate_tuple(payload);
    a.cmp(&b)
}

fn execute((_, packet_type, payload): &(u32, PacketType, PacketPayload)) -> u64 {
    match packet_type {
        PacketType::Sum => evaluate_packets(payload).sum::<u64>(),
        PacketType::Product => evaluate_packets(payload).product::<u64>(),
        PacketType::Minimum => evaluate_packets(payload).min().unwrap() as u64,
        PacketType::Maximum => evaluate_packets(payload).max().unwrap() as u64,
        PacketType::Literal => payload.literal() as u64,
        PacketType::GreaterThan => {
            if evaluate_compare(payload).is_gt() {
                1
            } else {
                0
            }
        }
        PacketType::LessThan => {
            if evaluate_compare(payload).is_lt() {
                1
            } else {
                0
            }
        }
        PacketType::Equal => {
            if evaluate_compare(payload).is_eq() {
                1
            } else {
                0
            }
        }
    }
}

#[test]
fn packet_test_1() {
    let input = parse(Input::from_buf(b"D2FE28"));
    let mut bits = BitStream::default();
    input
        .iter()
        .for_each(|b| bits.push4(reverse::<4>(*b as u32)));

    assert_eq!(
        Some((6, PacketType::Literal, PacketPayload::Literal(2021))),
        parse_packet(&mut bits)
    );
}

#[test]
fn packet_test_2() {
    let input = parse(Input::from_buf(b"38006F45291200"));
    let mut bits = BitStream::default();
    input
        .iter()
        .for_each(|b| bits.push4(reverse::<4>(*b as u32)));

    assert_eq!(
        Some((
            1,
            PacketType::LessThan,
            PacketPayload::Operator(vec![
                (6, PacketType::Literal, PacketPayload::Literal(10)),
                (2, PacketType::Literal, PacketPayload::Literal(20))
            ])
        )),
        parse_packet(&mut bits)
    );
}

#[test]
fn packet_test_3() {
    let input = parse(Input::from_buf(b"EE00D40C823060"));
    let mut bits = BitStream::default();
    input
        .iter()
        .for_each(|b| bits.push4(reverse::<4>(*b as u32)));

    assert_eq!(
        Some((
            7,
            PacketType::Maximum,
            PacketPayload::Operator(vec![
                (2, PacketType::Literal, PacketPayload::Literal(1)),
                (4, PacketType::Literal, PacketPayload::Literal(2)),
                (1, PacketType::Literal, PacketPayload::Literal(3))
            ])
        )),
        parse_packet(&mut bits)
    );
}

mod part1 {
    use crate::*;

    pub fn solve(input: &[u8]) -> u32 {
        let mut bits = BitStream::default();
        input
            .iter()
            .for_each(|b| bits.push4(reverse::<4>(*b as u32)));
        version_sum(&parse_packet(&mut bits).unwrap())
    }

    #[test]
    fn test_1() {
        let input = parse(Input::from_buf(b"8A004A801A8002F478"));
        assert_eq!(16, solve(&input));
    }

    #[test]
    fn test_2() {
        let input = parse(Input::from_buf(b"620080001611562C8802118E34"));
        assert_eq!(12, solve(&input));
    }

    #[test]
    fn test_3() {
        let input = parse(Input::from_buf(b"C0015000016115A2E0802F182340"));
        assert_eq!(23, solve(&input));
    }

    #[test]
    fn test_4() {
        let input = parse(Input::from_buf(b"A0016C880162017C3686B18A3D4780"));
        assert_eq!(31, solve(&input));
    }
}

mod part2 {
    use crate::*;

    pub fn solve(input: &[u8]) -> u64 {
        let mut bits = BitStream::default();
        input
            .iter()
            .for_each(|b| bits.push4(reverse::<4>(*b as u32)));
        execute(&parse_packet(&mut bits).unwrap())
    }
    // 29219084151 low

    #[test]
    fn test_1() {
        let input = parse(Input::from_buf(b"C200B40A82"));
        assert_eq!(3, solve(&input));
    }

    #[test]
    fn test_2() {
        let input = parse(Input::from_buf(b"04005AC33890"));
        assert_eq!(54, solve(&input));
    }

    #[test]
    fn test_3() {
        let input = parse(Input::from_buf(b"880086C3E88112"));
        assert_eq!(7, solve(&input));
    }

    #[test]
    fn test_4() {
        let input = parse(Input::from_buf(b"CE00C43D881120"));
        assert_eq!(9, solve(&input));
    }

    #[test]
    fn test_5() {
        let input = parse(Input::from_buf(b"D8005AC2A8F0"));
        assert_eq!(1, solve(&input));
    }

    #[test]
    fn test_6() {
        let input = parse(Input::from_buf(b"F600BC2D8F"));
        assert_eq!(0, solve(&input));
    }
    #[test]
    fn test_7() {
        let input = parse(Input::from_buf(b"9C005AC2F8F0"));
        assert_eq!(0, solve(&input));
    }
    #[test]
    fn test_8() {
        let input = parse(Input::from_buf(b"9C0141080250320F1802104A08"));
        assert_eq!(1, solve(&input));
    }
}
