const LITERAL_TYPE: usize = 4;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Length {
    Bits(usize),
    Packets(usize),
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Packet {
    Literal {
        v: usize,
        t: usize,
        num: usize,
    },
    Operator {
        v: usize,
        t: usize,
        len: Length,
        packets: Vec<Packet>,
    },
}

impl Packet {
    fn version_sum(&self) -> usize {
        match self {
            Packet::Literal { v, .. } => *v,
            Packet::Operator { v, packets, .. } => {
                packets.iter().map(|p| p.version_sum()).sum::<usize>() + v
            }
        }
    }

    fn value(&self) -> usize {
        match self {
            Packet::Literal { num, .. } => *num,
            Packet::Operator { t, packets, .. } => match t {
                0..=3 => {
                    let all = packets.iter().map(Packet::value);
                    match t {
                        0 => all.sum(),
                        1 => all.product(),
                        2 => all.min().unwrap(),
                        3 => all.max().unwrap(),
                        _ => unreachable!(),
                    }
                }
                5..=7 => {
                    let left = packets[0].value();
                    let right = packets[1].value();
                    if match t {
                        5 => left > right,
                        6 => left < right,
                        7 => left == right,
                        _ => unreachable!(),
                    } {
                        1
                    } else {
                        0
                    }
                }
                _ => unreachable!(),
            },
        }
    }
}

struct Parser<Iter> {
    iter: Iter,
    bits_read: usize,
}

fn hex2bits(hex: char) -> [bool; 4] {
    let v = match hex {
        '0'..='9' => hex as u8 - b'0',
        'A'..='F' => hex as u8 - b'A' + 10,
        _ => unreachable!(),
    };
    [v & 8 > 0, v & 4 > 0, v & 2 > 0, v & 1 > 0]
}

fn parser_from_str(input: &'static str) -> Parser<impl Iterator<Item = bool>> {
    Parser::new(input.chars().flat_map(hex2bits))
}

impl<Iter> Parser<Iter>
where
    Iter: Iterator<Item = bool>,
{
    fn new(iter: Iter) -> Self {
        Self { iter, bits_read: 0 }
    }

    fn get_bit(&mut self) -> bool {
        self.bits_read += 1;
        self.iter.next().unwrap()
    }

    fn read_int(&mut self, bits: usize) -> usize {
        let mut result = 0;
        for _ in 0..bits {
            result <<= 1;
            result |= if self.get_bit() { 1 } else { 0 };
        }
        result
    }

    fn read_var_int(&mut self) -> usize {
        let mut result = 0;
        while self.get_bit() {
            result <<= 4;
            result |= self.read_int(4);
        }
        result <<= 4;
        result |= self.read_int(4);
        result
    }

    fn read_packet(&mut self) -> Packet {
        let v = self.read_int(3);
        let t = self.read_int(3);
        if t == LITERAL_TYPE {
            let num = self.read_var_int();
            Packet::Literal { v, t, num }
        } else {
            let len = if self.get_bit() {
                Length::Packets(self.read_int(11))
            } else {
                Length::Bits(self.read_int(15))
            };
            Packet::Operator {
                v,
                t,
                len,
                packets: self.read_packets(len),
            }
        }
    }

    fn read_packets(&mut self, len: Length) -> Vec<Packet> {
        let mut packets = Vec::new();
        match len {
            Length::Bits(bits) => {
                let current_bits_read = self.bits_read;
                while self.bits_read < current_bits_read + bits {
                    packets.push(self.read_packet());
                }
            }
            Length::Packets(count) => {
                for _ in 0..count {
                    packets.push(self.read_packet());
                }
            }
        }
        packets
    }
}

fn main() {
    let packet = parser_from_str(include_str!("../../inputs/day16.txt")).read_packet();
    println!("Part 1: {}", packet.version_sum());
    println!("Part 2: {}", packet.value());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_str_to_bits() {
        let mut iter = "0F".chars().flat_map(|c| hex2bits(c));
        assert_eq!(iter.next(), Some(false));
        assert_eq!(iter.next(), Some(false));
        assert_eq!(iter.next(), Some(false));
        assert_eq!(iter.next(), Some(false));
        assert_eq!(iter.next(), Some(true));
        assert_eq!(iter.next(), Some(true));
        assert_eq!(iter.next(), Some(true));
        assert_eq!(iter.next(), Some(true));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_read_int() {
        let mut parser = parser_from_str("0ABC");
        assert_eq!(parser.read_int(4), 0);
        assert_eq!(parser.read_int(4), 0xA);
        assert_eq!(parser.read_int(4), 0xB);
    }

    #[test]
    fn test_parse_sample_literal() {
        assert_eq!(
            parser_from_str("D2FE28").read_packet(),
            Packet::Literal {
                v: 6,
                t: 4,
                num: 2021,
            }
        );
    }

    #[test]
    fn test_parse_sample_operator() {
        assert_eq!(
            parser_from_str("38006F45291200").read_packet(),
            Packet::Operator {
                v: 1,
                t: 6,
                len: Length::Bits(27),
                packets: vec![
                    Packet::Literal {
                        v: 6,
                        t: 4,
                        num: 10,
                    },
                    Packet::Literal {
                        v: 2,
                        t: 4,
                        num: 20,
                    },
                ],
            }
        );
    }

    #[test]
    fn test_parse_sample_operator2() {
        assert_eq!(
            parser_from_str("EE00D40C823060").read_packet(),
            Packet::Operator {
                v: 7,
                t: 3,
                len: Length::Packets(3),
                packets: vec![
                    Packet::Literal { v: 2, t: 4, num: 1 },
                    Packet::Literal { v: 4, t: 4, num: 2 },
                    Packet::Literal { v: 1, t: 4, num: 3 },
                ],
            }
        );
    }

    #[test]
    fn test_version_sum() {
        assert_eq!(
            parser_from_str("8A004A801A8002F478")
                .read_packet()
                .version_sum(),
            16
        );
        assert_eq!(
            parser_from_str("620080001611562C8802118E34")
                .read_packet()
                .version_sum(),
            12
        );
        assert_eq!(
            parser_from_str("C0015000016115A2E0802F182340")
                .read_packet()
                .version_sum(),
            23
        );
        assert_eq!(
            parser_from_str("A0016C880162017C3686B18A3D4780")
                .read_packet()
                .version_sum(),
            31
        );
    }

    #[test]
    fn test_value() {
        assert_eq!(parser_from_str("C200B40A82").read_packet().value(), 3);
        assert_eq!(parser_from_str("04005AC33890").read_packet().value(), 54);
        assert_eq!(parser_from_str("880086C3E88112").read_packet().value(), 7);
        assert_eq!(parser_from_str("CE00C43D881120").read_packet().value(), 9);
        assert_eq!(parser_from_str("D8005AC2A8F0").read_packet().value(), 1);
        assert_eq!(parser_from_str("F600BC2D8F").read_packet().value(), 0);
        assert_eq!(parser_from_str("9C005AC2F8F0").read_packet().value(), 0);
        assert_eq!(
            parser_from_str("9C0141080250320F1802104A08")
                .read_packet()
                .value(),
            1
        );
    }
}
