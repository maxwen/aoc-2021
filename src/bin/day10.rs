use aoc_2021::read_lines_as_vec;
use std::collections::VecDeque;

fn part1(lines: &[String]) -> u32 {
    // 321237
    let mut sum = 0;
    for line in lines.iter() {
        let mut stack = VecDeque::new();
        let mut error_type: char = 'c';
        for c in line.chars() {
            match c {
                '(' => stack.push_front('('),
                ')' => {
                    if stack.pop_front().unwrap() != '(' {
                        error_type = ')';
                        break;
                    }
                }
                '[' => stack.push_front('['),
                ']' => {
                    if stack.pop_front().unwrap() != '[' {
                        error_type = ']';
                        break;
                    }
                }
                '{' => stack.push_front('{'),
                '}' => {
                    if stack.pop_front().unwrap() != '{' {
                        error_type = '}';
                        break;
                    }
                }
                '<' => stack.push_front('<'),

                '>' => {
                    if stack.pop_front().unwrap() != '<' {
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

fn part2(lines: &[String]) -> usize {
    // 2360030859
    let mut score_list: Vec<usize> = vec![];
    for line in lines.iter() {
        let mut stack = VecDeque::new();
        let mut error_type: char = '-';
        for c in line.chars() {
            match c {
                '(' => stack.push_front('('),
                ')' => {
                    if stack.pop_front().unwrap() != '(' {
                        error_type = ')';
                        break;
                    }
                }
                '[' => stack.push_front('['),
                ']' => {
                    if stack.pop_front().unwrap() != '[' {
                        error_type = ']';
                        break;
                    }
                }
                '{' => stack.push_front('{'),
                '}' => {
                    if stack.pop_front().unwrap() != '{' {
                        error_type = '}';
                        break;
                    }
                }
                '<' => stack.push_front('<'),

                '>' => {
                    if stack.pop_front().unwrap() != '<' {
                        error_type = '>';
                        break;
                    }
                }
                _ => {}
            }
        }

        if error_type == '-' {
            let mut complete_line = "".to_string();
            while let Some(c) = stack.pop_front() {
                match c {
                    '(' => complete_line += ')'.to_string().as_str(),
                    '[' => complete_line += ']'.to_string().as_str(),
                    '{' => complete_line += '}'.to_string().as_str(),
                    '<' => complete_line += '>'.to_string().as_str(),
                    _ => {}
                }
            }
            let mut score = 0;
            for c in complete_line.chars() {
                match c {
                    ')' => score = score * 5 + 1,
                    ']' => score = score * 5 + 2,
                    '}' => score = score * 5 + 3,
                    '>' => score = score * 5 + 4,
                    _ => {}
                }
            }
            score_list.push(score);
        }
    }

    score_list.sort();
    score_list[score_list.len() / 2]
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
    use crate::{part1, part2};

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
        let result = part2(&lines);
        assert_eq!(result, 288957);
    }
}
