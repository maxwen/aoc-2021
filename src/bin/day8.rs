use aoc_2021::read_lines_as_vec;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Eq, PartialEq)]
struct Digit {
    segment_count: usize,
}

fn part1(lines: &[String]) -> u32 {
    // 321
    let digits = vec![
        Digit { segment_count: 2 },
        Digit { segment_count: 4 },
        Digit { segment_count: 3 },
        Digit { segment_count: 7 },
    ];
    let mut sum = 0;

    for line in lines.iter() {
        let parts = line.split("|").collect::<Vec<_>>();
        let pattern_part = parts.first().unwrap();
        let output_part = parts.last().unwrap();

        let signal_patterns = pattern_part.split_ascii_whitespace().collect::<Vec<_>>();
        let output_value = output_part.split_ascii_whitespace().collect::<Vec<_>>();

        for s in output_value.iter() {
            let len = s.len();
            if digits.contains(&Digit { segment_count: len }) {
                sum += 1;
            }
        }
    }
    sum
}

fn part2(lines: &[String]) -> u32 {
    // 1028926
    let mut sum = 0;

    for line in lines.iter() {
        let parts = line.split("|").collect::<Vec<_>>();
        let pattern_part = parts.first().unwrap();
        let output_part = parts.last().unwrap();

        let mut signal_patterns = pattern_part.split_ascii_whitespace().collect::<Vec<_>>();
        let output_value = output_part.split_ascii_whitespace().collect::<Vec<_>>();

        //   0
        // 1   2
        //   3
        // 4   5
        //   6
        let mut segement_map: HashMap<char, usize> = HashMap::new();
        let mut rev_segement_map: HashMap<usize, char> = HashMap::new();

        signal_patterns.sort_by(|a, b| a.len().cmp(&b.len()));

        let one_signal = signal_patterns.get(0).unwrap();
        let seven_signal = signal_patterns.get(1).unwrap();
        let four_signal = signal_patterns.get(2).unwrap();
        let eight_signal = signal_patterns.get(9).unwrap();

        // make sets so we can use set operations later
        let mut one_signal_chars = HashSet::new();
        one_signal.chars().for_each(|c| {
            one_signal_chars.insert(c);
        });

        let mut seven_signal_chars = HashSet::new();
        seven_signal.chars().for_each(|c| {
            seven_signal_chars.insert(c);
        });

        // segment 0 = 7 - 2
        let zero_char = seven_signal_chars
            .difference(&one_signal_chars)
            .nth(0)
            .unwrap();
        segement_map.insert(*zero_char, 0);
        rev_segement_map.insert(0, *zero_char);

        let mut four_signal_chars = HashSet::new();
        four_signal.chars().for_each(|c| {
            four_signal_chars.insert(c);
        });

        let mut eight_signal_chars = HashSet::new();
        eight_signal.chars().for_each(|c| {
            eight_signal_chars.insert(c);
        });

        let six_length_patterns = signal_patterns
            .iter()
            .filter(|s| s.len() == 6)
            .collect::<Vec<_>>();
        // 0,6,9
        for six_length_pattern in six_length_patterns.iter() {
            let mut six_signal_chars = HashSet::new();
            six_length_pattern.chars().for_each(|c| {
                six_signal_chars.insert(c);
            });
            let eight_chars_diff_one: HashSet<char> = eight_signal_chars
                .difference(&six_signal_chars)
                .map(|c| *c)
                .collect();
            if one_signal_chars.contains(&eight_chars_diff_one.iter().nth(0).unwrap()) {
                // -> 8 - 6 - 1 -> segment 5
                let c = one_signal_chars
                    .difference(&eight_chars_diff_one)
                    .map(|c| *c)
                    .nth(0)
                    .unwrap();
                segement_map.insert(c, 5);
                rev_segement_map.insert(5, c);

                segement_map.insert(*eight_chars_diff_one.iter().nth(0).unwrap(), 2);
                rev_segement_map.insert(2, *eight_chars_diff_one.iter().nth(0).unwrap());
            } else if !four_signal_chars.contains(&eight_chars_diff_one.iter().nth(0).unwrap()) {
                // -> 9 - 4 -> segment 4 (upper segment of 4) is handled before
                segement_map.insert(*eight_chars_diff_one.iter().nth(0).unwrap(), 4);
                rev_segement_map.insert(4, *eight_chars_diff_one.iter().nth(0).unwrap());
            } else {
                // -> 0 -> segment 3
                segement_map.insert(*eight_chars_diff_one.iter().nth(0).unwrap(), 3);
                rev_segement_map.insert(3, *eight_chars_diff_one.iter().nth(0).unwrap());
            }
        }

        // only missing segment 1 and 6
        // segment 1 is rest of 2,3,5 from 4
        let mut four_signal_chars_reminder = HashSet::new();
        four_signal_chars_reminder.insert(*rev_segement_map.get(&2).unwrap());
        four_signal_chars_reminder.insert(*rev_segement_map.get(&3).unwrap());
        four_signal_chars_reminder.insert(*rev_segement_map.get(&5).unwrap());

        let c = four_signal_chars
            .difference(&four_signal_chars_reminder)
            .map(|c| *c)
            .nth(0)
            .unwrap();
        segement_map.insert(c, 1);
        rev_segement_map.insert(1, c);

        // segment 6 is the only one remaining in diff to all segments of 8
        let segment_chars_known = segement_map.keys().map(|n| *n).collect::<HashSet<_>>();
        let c = eight_signal_chars
            .difference(&segment_chars_known)
            .map(|c| *c)
            .nth(0)
            .unwrap();
        segement_map.insert(c, 6);
        rev_segement_map.insert(6, c);

        // println!("{:?}", segement_map);

        // yes we could handle unique length digits 1,4,7,8 without map
        // map sorted chars to digit
        let mut digits_map: HashMap<Vec<char>, usize> = HashMap::new();
        for i in 0..10 {
            let mut v: Vec<char> = vec![];
            match i {
                0 => {
                    v.push(*rev_segement_map.get(&0).unwrap());
                    v.push(*rev_segement_map.get(&1).unwrap());
                    v.push(*rev_segement_map.get(&2).unwrap());
                    v.push(*rev_segement_map.get(&4).unwrap());
                    v.push(*rev_segement_map.get(&5).unwrap());
                    v.push(*rev_segement_map.get(&6).unwrap())
                }
                1 => {
                    v.push(*rev_segement_map.get(&2).unwrap());
                    v.push(*rev_segement_map.get(&5).unwrap());
                }
                2 => {
                    v.push(*rev_segement_map.get(&0).unwrap());
                    v.push(*rev_segement_map.get(&2).unwrap());
                    v.push(*rev_segement_map.get(&3).unwrap());
                    v.push(*rev_segement_map.get(&4).unwrap());
                    v.push(*rev_segement_map.get(&6).unwrap())
                }
                3 => {
                    v.push(*rev_segement_map.get(&0).unwrap());
                    v.push(*rev_segement_map.get(&2).unwrap());
                    v.push(*rev_segement_map.get(&3).unwrap());
                    v.push(*rev_segement_map.get(&5).unwrap());
                    v.push(*rev_segement_map.get(&6).unwrap())
                }
                4 => {
                    v.push(*rev_segement_map.get(&1).unwrap());
                    v.push(*rev_segement_map.get(&3).unwrap());
                    v.push(*rev_segement_map.get(&2).unwrap());
                    v.push(*rev_segement_map.get(&5).unwrap())
                }
                5 => {
                    v.push(*rev_segement_map.get(&0).unwrap());
                    v.push(*rev_segement_map.get(&1).unwrap());
                    v.push(*rev_segement_map.get(&3).unwrap());
                    v.push(*rev_segement_map.get(&5).unwrap());
                    v.push(*rev_segement_map.get(&6).unwrap())
                }
                6 => {
                    v.push(*rev_segement_map.get(&0).unwrap());
                    v.push(*rev_segement_map.get(&1).unwrap());
                    v.push(*rev_segement_map.get(&3).unwrap());
                    v.push(*rev_segement_map.get(&4).unwrap());
                    v.push(*rev_segement_map.get(&5).unwrap());
                    v.push(*rev_segement_map.get(&6).unwrap())
                }
                7 => {
                    v.push(*rev_segement_map.get(&0).unwrap());
                    v.push(*rev_segement_map.get(&2).unwrap());
                    v.push(*rev_segement_map.get(&5).unwrap());
                }
                8 => {
                    v.push(*rev_segement_map.get(&0).unwrap());
                    v.push(*rev_segement_map.get(&1).unwrap());
                    v.push(*rev_segement_map.get(&2).unwrap());
                    v.push(*rev_segement_map.get(&3).unwrap());
                    v.push(*rev_segement_map.get(&4).unwrap());
                    v.push(*rev_segement_map.get(&5).unwrap());
                    v.push(*rev_segement_map.get(&6).unwrap())
                }
                9 => {
                    v.push(*rev_segement_map.get(&0).unwrap());
                    v.push(*rev_segement_map.get(&1).unwrap());
                    v.push(*rev_segement_map.get(&2).unwrap());
                    v.push(*rev_segement_map.get(&3).unwrap());
                    v.push(*rev_segement_map.get(&5).unwrap());
                    v.push(*rev_segement_map.get(&6).unwrap())
                }
                _ => {}
            }
            v.sort();
            digits_map.insert(v, i);
        }

        // println!("{:?}", digits_map);
        let mut digits_str = "".to_string();
        for output in output_value.iter() {
            let mut chars = output.chars().collect::<Vec<_>>();
            chars.sort();
            let d = *digits_map.get(&chars).unwrap();
            digits_str += d.to_string().as_str();
        }
        let digits_value: u32 = digits_str.parse().unwrap();
        // println!("{}", digits_value);
        sum += digits_value;
    }
    sum
}

fn main() {
    let lines = read_lines_as_vec("input/input_day8.txt").unwrap();

    //     let lines = vec![
    //         "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb |
    // fdgacbe cefdb cefbgd gcbe",
    //         "edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec |
    // fcgedb cgb dgebacf gc",
    //         "fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef |
    // cg cg fdcagb cbg",
    //         "fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega |
    // efabcd cedba gadfec cb",
    //         "aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga |
    // gecf egdcabf bgf bfgea",
    //         "fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf |
    // gebdcfa ecba ca fadegcb",
    //         "dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf |
    // cefg dcbef fcge gbcadfe",
    //         "bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd |
    // ed bcgafe cdgba cbgef",
    //         "egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg |
    // gbdfcae bgc cg cgb",
    //         "gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc |
    // fgae cfgab fg bagce",
    //     ]
    //     .iter()
    //     .map(|s| s.to_string())
    //     .collect::<Vec<_>>();
    println!("{}", part1(&lines));
    println!("{}", part2(&lines));
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    fn it_works() {
        let lines = vec![
            "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb |
fdgacbe cefdb cefbgd gcbe",
            "edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec |
            fcgedb cgb dgebacf gc",
            "fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef |
            cg cg fdcagb cbg",
            "fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega |
            efabcd cedba gadfec cb",
            "aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga |
            gecf egdcabf bgf bfgea",
            "fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf |
            gebdcfa ecba ca fadegcb",
            "dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf |
            cefg dcbef fcge gbcadfe",
            "bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd |
            ed bcgafe cdgba cbgef",
            "egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg |
            gbdfcae bgc cg cgb",
            "gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc |
            fgae cfgab fg bagce",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<_>>();

        let result = part1(&lines);
        assert_eq!(result, 26);
        let result = part2(&lines);
        assert_eq!(result, 61229);
    }
}
