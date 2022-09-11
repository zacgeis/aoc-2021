use std::fs;

#[derive(Debug)]
enum Op {
    Sum,
    Product,
    Minimum,
    Maximum,
    GreaterThan,
    LessThan,
    EqualTo,
}

impl Op {
    fn from_u32(v: u32) -> Self {
        match v {
            0 => Op::Sum,
            1 => Op::Product,
            2 => Op::Minimum,
            3 => Op::Maximum,
            5 => Op::GreaterThan,
            6 => Op::LessThan,
            7 => Op::EqualTo,
            _ => panic!("unexpected op id"),
        }
    }
}

#[derive(Debug)]
enum Packet {
    Literal(u32, u64),
    Operator(u32, Op, Vec<Packet>),
}

fn get_input() -> Vec<u8> {
    let input = fs::read_to_string("inputs/16.txt").unwrap();
    let mut message = vec![];
    for c in input.chars() {
        if let Some(v) = c.to_digit(16) {
            for i in (0..4).rev() {
                message.push((v & (1 << i) > 0) as u8);
            }
        }
    }
    message
}

fn main() {
    part1();
    part2();
}

fn parse_int(message: &[u8]) -> u32 {
    let mut result = 0;
    for v in message {
        result <<= 1;
        result |= *v as u32;
    }
    result
}

fn parse(message: &[u8], mut pos: usize) -> (Packet, usize) {
    let version = parse_int(&message[pos..pos + 3]);
    pos += 3;
    let type_id = parse_int(&message[pos..pos + 3]);
    pos += 3;
    match type_id {
        4 => {
            let mut value = 0;
            loop {
                value <<= 4;
                let prefix = message[pos];
                pos += 1;
                let partial_value = parse_int(&message[pos..pos + 4]);
                pos += 4;
                value |= partial_value as u64;
                if prefix == 0 {
                    break;
                }
            }
            (Packet::Literal(version, value), pos)
        }
        _ => {
            let length_type_id = message[pos];
            pos += 1;
            match length_type_id {
                0 => {
                    let total_length = parse_int(&message[pos..pos + 15]);
                    pos += 15;
                    let end_pos = pos + total_length as usize;
                    let mut packets = vec![];
                    loop {
                        let (packet, new_pos) = parse(message, pos);
                        pos = new_pos;
                        packets.push(packet);
                        if pos == end_pos {
                            break;
                        }
                    }
                    (
                        Packet::Operator(version, Op::from_u32(type_id), packets),
                        pos,
                    )
                }
                1 => {
                    let sub_packet_count = parse_int(&message[pos..pos + 11]);
                    pos += 11;
                    let mut packets = vec![];
                    for _ in 0..sub_packet_count {
                        let (packet, new_pos) = parse(message, pos);
                        pos = new_pos;
                        packets.push(packet);
                    }
                    (
                        Packet::Operator(version, Op::from_u32(type_id), packets),
                        pos,
                    )
                }
                _ => panic!("unexpected length_type_id: {}", length_type_id),
            }
        }
    }
}

fn sum_version_numbers(packet: Packet) -> u64 {
    match packet {
        Packet::Literal(version, _) => version as u64,
        Packet::Operator(version, _, packets) => {
            let mut sum = 0;
            for packet in packets {
                sum += sum_version_numbers(packet);
            }
            sum + version as u64
        }
    }
}

fn part1() {
    let message = get_input();
    let (packet, _) = parse(&message, 0);

    println!("part1: {}", sum_version_numbers(packet));
}

fn run_ops(packet: &Packet) -> u64 {
    match packet {
        Packet::Literal(_, value) => *value,
        Packet::Operator(_, op, packets) => match op {
            Op::Sum => packets.iter().map(|packet| run_ops(packet)).sum(),
            Op::Product => packets.iter().map(|packet| run_ops(packet)).product(),
            Op::Minimum => packets.iter().map(|packet| run_ops(packet)).min().unwrap(),
            Op::Maximum => packets.iter().map(|packet| run_ops(packet)).max().unwrap(),
            Op::GreaterThan => {
                let left = run_ops(&packets[0]);
                let right = run_ops(&packets[1]);
                if left > right {
                    1
                } else {
                    0
                }
            }
            Op::LessThan => {
                let left = run_ops(&packets[0]);
                let right = run_ops(&packets[1]);
                if left < right {
                    1
                } else {
                    0
                }
            }
            Op::EqualTo => {
                let left = run_ops(&packets[0]);
                let right = run_ops(&packets[1]);
                if left == right {
                    1
                } else {
                    0
                }
            }
        },
    }
}

fn part2() {
    let message = get_input();
    let (packet, _) = parse(&message, 0);

    println!("part2: {}", run_ops(&packet));
}
