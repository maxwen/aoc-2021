use aoc_2021::read_lines_as_vec;

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
    0u32
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
    use crate::part1;

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
        // let result = part2(&lines);
        // assert_eq!(result, 168);
    }
}
