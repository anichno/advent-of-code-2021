#[derive(Debug)]
enum Packet {
    Literal(LiteralPacket),
    Operator(OperatorPacket),
}

#[derive(Debug)]
struct LiteralPacket {
    version: u8,
    value: u64,
}

#[derive(Debug)]
struct OperatorPacket {
    version: u8,
    type_id: u8,
    packets: Vec<Packet>,
}

struct Cursor<'a> {
    bit_pos: usize,
    data: &'a [u8],
}

impl<'a> Cursor<'a> {
    fn new(data: &'a [u8]) -> Cursor {
        Self { bit_pos: 0, data }
    }

    fn read(&mut self, num_bits: usize) -> u64 {
        let mut retval = 0;
        let mut bits_to_get = num_bits;
        let mut byte_pos = self.bit_pos / 8;

        while bits_to_get > 0 {
            let remaining_bits_in_byte = 8 - self.bit_pos % 8;
            let bits_grabbed = remaining_bits_in_byte.min(bits_to_get);
            let mask = 0xff >> (8 - bits_grabbed) << (remaining_bits_in_byte - bits_grabbed);

            bits_to_get -= bits_grabbed;
            retval |= ((self.data[byte_pos] & mask) as u64)
                >> (8 - bits_grabbed - self.bit_pos % 8)
                << bits_to_get;
            self.bit_pos += bits_grabbed;
            if self.bit_pos % 8 == 0 {
                byte_pos += 1;
            }
        }

        retval
    }
}

fn parse(cursor: &mut Cursor) -> Packet {
    let version = cursor.read(3) as u8;
    let type_id = cursor.read(3) as u8;

    match type_id {
        4 => {
            let mut value = 0;
            loop {
                let chunk = cursor.read(5);
                let more_chunks = chunk & 0b10000 > 0;
                value = (value << 4) | (chunk & 0xf);
                if !more_chunks {
                    break;
                }
            }
            Packet::Literal(LiteralPacket { version, value })
        }
        _ => {
            let mut sub_packets = Vec::new();
            let length_type_id = cursor.read(1) > 0;
            if length_type_id {
                let total_packets = cursor.read(11);
                for _ in 0..total_packets {
                    sub_packets.push(parse(cursor));
                }
            } else {
                let total_len = cursor.read(15) as usize;
                let target_pos = cursor.bit_pos + total_len;
                while cursor.bit_pos < target_pos {
                    sub_packets.push(parse(cursor));
                }
            }

            Packet::Operator(OperatorPacket {
                version,
                type_id,
                packets: sub_packets,
            })
        }
    }
}

fn solve1(packet: &Packet) -> u64 {
    let mut sum = 0;

    match packet {
        Packet::Literal(packet) => sum += packet.version as u64,
        Packet::Operator(packet) => {
            sum += packet.version as u64;
            for packet in &packet.packets {
                sum += solve1(packet);
            }
        }
    }

    sum
}

fn solve2(packet: &Packet) -> u64 {
    match packet {
        Packet::Literal(packet) => packet.value,
        Packet::Operator(packet) => {
            match packet.type_id {
                //sum
                0 => {
                    let mut sum = 0;
                    for packet in &packet.packets {
                        sum += solve2(packet);
                    }

                    sum
                }
                // product
                1 => {
                    let mut product = 1;
                    for packet in &packet.packets {
                        product *= solve2(packet);
                    }

                    product
                }
                // minimum
                2 => {
                    let mut min = u64::MAX;
                    for packet in &packet.packets {
                        min = min.min(solve2(packet));
                    }

                    min
                }
                //maximum
                3 => {
                    let mut max = u64::MIN;
                    for packet in &packet.packets {
                        max = max.max(solve2(packet));
                    }

                    max
                }
                //greater than
                5 => {
                    if solve2(&packet.packets[0]) > solve2(&packet.packets[1]) {
                        1
                    } else {
                        0
                    }
                }
                //less than
                6 => {
                    if solve2(&packet.packets[0]) < solve2(&packet.packets[1]) {
                        1
                    } else {
                        0
                    }
                }
                // equal to
                7 => {
                    if solve2(&packet.packets[0]) == solve2(&packet.packets[1]) {
                        1
                    } else {
                        0
                    }
                }
                _ => panic!("invalid type id: {}", packet.type_id),
            }
        }
    }
}

fn main() {
    let input = include_str!("input.txt").trim();
    let input_bytes: Vec<u8> = (0..input.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&input[i..i + 2], 16).unwrap())
        .collect();

    let mut cursor = Cursor::new(&input_bytes);
    let parsed = parse(&mut cursor);

    println!("part 1: {}", solve1(&parsed));
    println!("part 2: {}", solve2(&parsed));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &[u8] = &[
        0xA0, 0x01, 0x6C, 0x88, 0x01, 0x62, 0x01, 0x7C, 0x36, 0x86, 0xB1, 0x8A, 0x3D, 0x47, 0x80,
    ];
    const INPUT2: &[u8] = &[0x8A, 0x00, 0x4A, 0x80, 0x1A, 0x80, 0x02, 0xF4, 0x78];

    #[test]
    fn test_cursor() {
        let mut cursor = Cursor::new(&[0xD2, 0xFE, 0x28]);
        let version = cursor.read(3);
        assert_eq!(version, 6);
        assert_eq!(cursor.bit_pos, 3);
        let type_id = cursor.read(3);
        assert_eq!(cursor.bit_pos, 6);
        assert_eq!(type_id, 4);
        assert_eq!(cursor.read(5), 0b10111);
        assert_eq!(cursor.read(5), 0b11110);
        assert_eq!(cursor.read(5), 0b00101);
    }

    #[test]
    fn test_parse() {
        let mut cursor = Cursor::new(&[0xD2, 0xFE, 0x28]);
        let parsed = parse(&mut cursor);
        if let Packet::Literal(packet) = parsed {
            assert_eq!(packet.version, 6);
            assert_eq!(packet.value, 2021);
        } else {
            panic!()
        }
    }

    #[test]
    fn test_solve1() {
        let mut cursor = Cursor::new(&INPUT1);
        let parsed = parse(&mut cursor);
        assert_eq!(solve1(&parsed), 31);

        let mut cursor = Cursor::new(&INPUT2);
        let parsed = parse(&mut cursor);
        assert_eq!(solve1(&parsed), 16);
    }

    #[test]
    fn test_solve2() {
        let mut cursor = Cursor::new(&[
            0x9C, 0x01, 0x41, 0x08, 0x02, 0x50, 0x32, 0x0F, 0x18, 0x02, 0x10, 0x4A, 0x08,
        ]);
        let parsed = parse(&mut cursor);
        assert_eq!(solve2(&parsed), 1);
    }
}
