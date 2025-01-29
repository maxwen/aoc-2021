use std::collections::VecDeque;
use aoc_2021::read_lines_as_vec;

fn part1(lines: &[String]) -> u32 {
    // 321237
    let mut sum = 0;
    for line in lines.iter() {
        let mut stack = VecDeque::new();
        let mut error_pos = 0;
        let mut error_type: char = 'c';
        for (x, c) in line.chars().enumerate() {
            match c {
                '(' => stack.push_front('('),
                ')' => {
                    if stack.pop_front().unwrap() != '(' {
                        error_pos = x;
                        error_type = ')';
                        break;
                    }
                }
                '[' => stack.push_front('['),
                ']' => {
                    if stack.pop_front().unwrap() != '[' {
                        error_pos = x;
                        error_type = ']';
                        break;
                    }
                }
                '{' => stack.push_front('{'),
                '}' => {
                    if stack.pop_front().unwrap() != '{' {
                        error_pos = x;
                        error_type = '}';
                        break;
                    }
                }
                '<' => stack.push_front('<'),

                '>' => {
                    if stack.pop_front().unwrap() != '<' {
                        error_pos = x;
                        error_type = '>';
                        break;
                    }
                }
                _ => {}
            }
        }

        match error_type {
            ')' => sum += 3,
            ']' => sum += 57,
            '}' => sum += 1197,
            '>' => sum += 25137,
            _ => {}
        }
    }

    sum
}

fn part2(lines: &[String]) -> u32 {
    for line in lines.iter() {}

    0u32
}

fn main() {
    let lines = read_lines_as_vec("input/input_day10.txt").unwrap();

    // let lines = vec![
    //     "[({(<(())[]>[[{[]{<()<>>",
    //     "[(()[<>])]({[<{<<[]>>(",
    //     "{([(<{}[<>[]}>{[]{[(<()>",
    //     "(((({<>}<{<{<>}{[]{[]{}",
    //     "[[<[([]))<([[{}[[()]]]",
    //     "[{[{({}]{}}([{[{{{}}([]",
    //     "{<[[]]>}<{[{[{[]{()[[[]",
    //     "[<(<(<(<{}))><([]([]()",
    //     "<{([([[(<>()){}]>(<<{{",
    //     "<{([{{}}[<[[[<>{}]]]>[]]",
    // ]
    // .iter()
    // .map(|s| s.to_string())
    // .collect::<Vec<_>>();
    println!("{}", part1(&lines));
    println!("{}", part2(&lines));
}

#[cfg(test)]
mod tests {
    use crate::part1;

    #[test]
    fn it_works() {
        let lines = vec![
            "[({(<(())[]>[[{[]{<()<>>",
            "[(()[<>])]({[<{<<[]>>(",
            "{([(<{}[<>[]}>{[]{[(<()>",
            "(((({<>}<{<{<>}{[]{[]{}",
            "[[<[([]))<([[{}[[()]]]",
            "[{[{({}]{}}([{[{{{}}([]",
            "{<[[]]>}<{[{[{[]{()[[[]",
            "[<(<(<(<{}))><([]([]()",
            "<{([([[(<>()){}]>(<<{{",
            "<{([{{}}[<[[[<>{}]]]>[]]",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<_>>();

        let result = part1(&lines);
        assert_eq!(result, 26397);
        // let result = part2(&lines);
        // assert_eq!(result, 1134);
    }
}
