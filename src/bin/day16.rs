use std::collections::HashMap;
use aoc_2021::read_lines_as_vec;

fn get_hex_value(c: char) -> u32 {
    c.to_digit(16).unwrap()
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
    println!("literal = {} start_idx = {}", literal, start_idx);

    (literal, start_idx)
}

fn parse_operator(
    decoded_line: &String,
    start_idx: usize,
    parse_stats: &mut HashMap<usize, usize>,
) -> usize {
    let mut start_idx = start_idx;
    let length_id = decoded_line.chars().nth(start_idx).unwrap();
    if length_id == '0' {
        start_idx += 1;
        let length = 15;

        let sub_length = decoded_line.get(start_idx..start_idx + length).unwrap();
        let sub_length = u32::from_str_radix(&sub_length, 2).unwrap();
        println!("sub_length = {}", sub_length);
        start_idx += length;

        let sub_packets_start = start_idx;
        while start_idx - sub_packets_start < sub_length as usize {
            let packet_version =
                u8::from_str_radix(decoded_line.get(start_idx..start_idx + 3).unwrap(), 2).unwrap();
            let packet_type =
                u8::from_str_radix(decoded_line.get(start_idx + 3..start_idx + 6).unwrap(), 2)
                    .unwrap();
            start_idx += 6;

            if packet_type == 4 {
                parse_stats.entry(1).and_modify(|n| *n += packet_version as usize);

                let res = parse_literal(&decoded_line, start_idx);
                let literal = res.0;
                start_idx = res.1;
                println!("literal = {}", literal);
            } else {
                parse_stats.entry(0).and_modify(|n| *n += packet_version as usize);

                start_idx = parse_operator(&decoded_line, start_idx, parse_stats);
            }
        }
    }
    if length_id == '1' {
        start_idx += 1;
        let length = 11;

        let sub_length = decoded_line.get(start_idx..start_idx + length).unwrap();
        let sub_count = u32::from_str_radix(&sub_length, 2).unwrap();
        println!("sub_count = {}", sub_count);

        start_idx += length;

        for _ in 0..sub_count {
            let packet_version =
                u8::from_str_radix(decoded_line.get(start_idx..start_idx + 3).unwrap(), 2).unwrap();
            let packet_type =
                u8::from_str_radix(decoded_line.get(start_idx + 3..start_idx + 6).unwrap(), 2)
                    .unwrap();
            start_idx += 6;

            println!("{:?}", parse_stats);

            if packet_type == 4 {
                parse_stats.entry(1).and_modify(|n| *n += packet_version as usize);

                let res = parse_literal(&decoded_line, start_idx);
                let literal = res.0;
                start_idx = res.1;
            } else {
                parse_stats.entry(0).and_modify(|n| *n += packet_version as usize);

                start_idx = parse_operator(&decoded_line, start_idx, parse_stats);
            }
        }
    }
    start_idx
}

fn part1(line: &String) -> usize {
    // 871
    let mut decoded_line = "".to_string();
    for c in line.chars() {
        let value = get_hex_value(c);
        decoded_line += format!("{:04b}", value).as_str();
    }

    let mut parse_stats = HashMap::new();
    let packet_version = u8::from_str_radix(decoded_line.get(0..3).unwrap(), 2).unwrap();
    let packet_type = u8::from_str_radix(decoded_line.get(3..6).unwrap(), 2).unwrap();

    let start_idx = 6;

    if packet_type == 4 {
        parse_stats.insert(0, 0);
        parse_stats.insert(1, packet_version as usize);
        let (literal, pos) = parse_literal(&decoded_line, start_idx);
    } else {
        parse_stats.insert(0, packet_version as usize);
        parse_stats.insert(1, 0);
        let pos = parse_operator(&decoded_line, start_idx, &mut parse_stats);
    }

    parse_stats[&0] + parse_stats[&1]
}

fn part2(line: &String) -> u32 {
    0u32
}

fn main() {
    let lines = read_lines_as_vec("input/input_day16.txt").unwrap();

    // let lines = vec!["A0016C880162017C3686B18A3D4780"]
    //     .iter()
    //     .map(|s| s.to_string())
    //     .collect::<Vec<_>>();
    println!("{}", part1(&lines[0]));
    println!("{}", part2(&lines[0]));
}

#[cfg(test)]
mod tests {
    use crate::part1;

    #[test]
    fn it_works() {
        let lines = vec!["A0016C880162017C3686B18A3D4780"]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        let result = part1(&lines[0]);
        assert_eq!(result, 31);
        // let result = part2(&lines);
        // assert_eq!(result, 315);
    }
}
