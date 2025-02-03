use aoc_2021::read_lines_as_vec;
use std::cmp::{max, min};
use std::collections::HashMap;

fn get_hex_value(c: char) -> u32 {
    c.to_digit(16).unwrap()
}

fn calc_operator(operator_type: usize, lhs: u64, rhs: u64) -> u64 {
    let res = match operator_type {
        0 => lhs + rhs,
        1 => lhs * rhs,
        2 => min(lhs, rhs),
        3 => max(lhs, rhs),
        5 => {
            if lhs > rhs {
                1
            } else {
                0
            }
        }
        6 => {
            if lhs < rhs {
                1
            } else {
                0
            }
        }
        7 => {
            if lhs == rhs {
                1
            } else {
                0
            }
        }
        _ => lhs,
    };

    res
}

fn parse_literal(decoded_line: &String, start_idx: usize) -> (u64, usize) {
    let mut data_value = "".to_string();
    let mut start_idx = start_idx;

    loop {
        let chunk = decoded_line.get(start_idx..start_idx + 5).unwrap();
        data_value += chunk.get(1..).unwrap();
        start_idx = start_idx + 5;
        if chunk.starts_with('0') {
            break;
        }
    }
    let literal = u64::from_str_radix(&*data_value, 2).unwrap();
    (literal, start_idx)
}

fn parse_operator(
    level: u8,
    op_packet_type: usize,
    decoded_line: &String,
    start_idx: usize,
    parse_stats: &mut HashMap<usize, usize>,
) -> (u64, usize) {
    let mut start_idx = start_idx;
    let mut lhs = None;
    let length_id = decoded_line.chars().nth(start_idx).unwrap();

    if length_id == '0' {
        start_idx += 1;
        let length = 15;

        let sub_length = decoded_line.get(start_idx..start_idx + length).unwrap();
        let sub_length = usize::from_str_radix(&sub_length, 2).unwrap();
        start_idx += length;

        let sub_packets_start = start_idx;
        while start_idx - sub_packets_start < sub_length {
            let packet_version =
                usize::from_str_radix(decoded_line.get(start_idx..start_idx + 3).unwrap(), 2).unwrap();
            let packet_type =
                usize::from_str_radix(decoded_line.get(start_idx + 3..start_idx + 6).unwrap(), 2)
                    .unwrap();
            start_idx += 6;

            if packet_type == 4 {
                parse_stats
                    .entry(1)
                    .and_modify(|n| *n += packet_version);

                let res = parse_literal(&decoded_line, start_idx);
                let literal_value = res.0;
                if lhs.is_none() {
                    lhs = Some(literal_value)
                } else {
                    lhs = Some(calc_operator(op_packet_type, lhs.unwrap(), literal_value));
                }
                start_idx = res.1;
            } else {
                parse_stats
                    .entry(0)
                    .and_modify(|n| *n += packet_version);

                let res = parse_operator(level + 1, packet_type, &decoded_line, start_idx, parse_stats);
                let operator_value = res.0;
                if lhs.is_none() {
                    lhs = Some(operator_value)
                } else {
                    lhs = Some(calc_operator(op_packet_type, lhs.unwrap(), operator_value));
                }
                start_idx = res.1;
            }
        }
    }
    if length_id == '1' {
        start_idx += 1;
        let length = 11;

        let sub_length = decoded_line.get(start_idx..start_idx + length).unwrap();
        let sub_count = u32::from_str_radix(&sub_length, 2).unwrap();

        start_idx += length;

        for _ in 0..sub_count {
            let packet_version =
                usize::from_str_radix(decoded_line.get(start_idx..start_idx + 3).unwrap(), 2).unwrap();
            let packet_type =
                usize::from_str_radix(decoded_line.get(start_idx + 3..start_idx + 6).unwrap(), 2)
                    .unwrap();
            start_idx += 6;

            if packet_type == 4 {
                parse_stats
                    .entry(1)
                    .and_modify(|n| *n += packet_version);
                let res = parse_literal(&decoded_line, start_idx);
                let literal_value = res.0;
                if lhs.is_none() {
                    lhs = Some(literal_value);
                } else {
                    lhs = Some(calc_operator(op_packet_type, lhs.unwrap(), literal_value));
                }
                start_idx = res.1;
            } else {
                parse_stats
                    .entry(0)
                    .and_modify(|n| *n += packet_version);

                let res = parse_operator(level + 1, packet_type, &decoded_line, start_idx, parse_stats);
                let operator_value = res.0;
                if lhs.is_none() {
                    lhs = Some(operator_value)
                } else {
                    lhs = Some(calc_operator(op_packet_type, lhs.unwrap(), operator_value));
                }
                start_idx = res.1;
            }
        }
    }
    (lhs.unwrap(), start_idx)
}

fn part1(line: &String) -> usize {
    // 871
    let mut decoded_line = "".to_string();
    for c in line.chars() {
        let value = get_hex_value(c);
        decoded_line += format!("{:04b}", value).as_str();
    }

    let mut parse_stats = HashMap::new();
    let packet_version = usize::from_str_radix(decoded_line.get(0..3).unwrap(), 2).unwrap();
    let packet_type = usize::from_str_radix(decoded_line.get(3..6).unwrap(), 2).unwrap();

    let start_idx = 6;

    if packet_type == 4 {
        parse_stats.insert(0, 0);
        parse_stats.insert(1, packet_version);
        let (_, _) = parse_literal(&decoded_line, start_idx);
    } else {
        parse_stats.insert(0, packet_version);
        parse_stats.insert(1, 0);
        let (_, _) = parse_operator(0, packet_type, &decoded_line, start_idx, &mut parse_stats);
    }

    parse_stats[&0] + parse_stats[&1]
}

fn part2(line: &String) -> u64 {
    // 68703010504
    let mut decoded_line = "".to_string();
    for c in line.chars() {
        let value = get_hex_value(c);
        decoded_line += format!("{:04b}", value).as_str();
    }

    let mut parse_stats = HashMap::new();
    let packet_version = usize::from_str_radix(decoded_line.get(0..3).unwrap(), 2).unwrap();
    let packet_type = usize::from_str_radix(decoded_line.get(3..6).unwrap(), 2).unwrap();

    let start_idx = 6;

    if packet_type == 4 {
        parse_stats.insert(0, 0);
        parse_stats.insert(1, packet_version);
        let (_, _) = parse_literal(&decoded_line, start_idx);
    } else {
        parse_stats.insert(0, packet_version);
        parse_stats.insert(1, 0);
        let (value, _) = parse_operator(0, packet_type, &decoded_line, start_idx, &mut parse_stats);
        return value
    }
    0u64
}

fn main() {
    let lines = read_lines_as_vec("input/input_day16.txt").unwrap();
    println!("{}", part1(&lines[0]));
    println!("{}", part2(&lines[0]));
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    fn it_works() {
        let lines = vec!["A0016C880162017C3686B18A3D4780"]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        assert_eq!(part1(&lines[0]), 31);

        let lines = vec!["C0015000016115A2E0802F182340"]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        assert_eq!(part1(&lines[0]), 23);

        let lines = vec!["C200B40A82"]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        assert_eq!(part2(&lines[0]), 3);
        let lines = vec!["04005AC33890"]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        assert_eq!(part2(&lines[0]), 54);
        let lines = vec!["880086C3E88112"]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        assert_eq!(part2(&lines[0]), 7);
        let lines = vec!["CE00C43D881120"]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        assert_eq!(part2(&lines[0]), 9);
        let lines = vec!["D8005AC2A8F0"]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        assert_eq!(part2(&lines[0]), 1);
        let lines = vec!["F600BC2D8F"]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        assert_eq!(part2(&lines[0]), 0);
        let lines = vec!["9C005AC2F8F0"]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        assert_eq!(part2(&lines[0]), 0);
        let lines = vec!["9C0141080250320F1802104A08"]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        assert_eq!(part2(&lines[0]), 1);
    }
}
