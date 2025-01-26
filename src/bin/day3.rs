use aoc_2021::read_lines_as_vec;

fn part1(lines: &[String]) -> u32 {
    // 4160394
    let mut gamma = "".to_string();
    let mut epsilon = "0".to_string();
    let test = lines[0].to_string();
    let count = test.len();

    for i in 0..count {
        let mut count_one = 0;
        let mut count_zero = 0;
        for line in lines.iter() {
            let bit = line.chars().nth(i).unwrap();
            match bit {
                '1' => count_one += 1,
                '0' => count_zero += 1,
                _ => {}
            }
        }
        gamma += if count_zero > count_one {
            "0"
        } else {
            "1"
        };
        epsilon += if count_zero > count_one {
            "1"
        } else {
            "0"
        };
    }

    // println!("{} {}", gamma, epsilon);
    u32::from_str_radix(gamma.as_str(), 2).unwrap() * u32::from_str_radix(epsilon.as_str(), 2).unwrap()
}
fn part2(lines: &[String]) -> u32 {
    let mut sum = 0u32;

    sum
}

fn main() {
    let lines = read_lines_as_vec("input/input_day3.txt").unwrap();

    // let lines = vec![
    //     "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000", "11001",
    //     "00010", "01010",
    // ]
    // .iter()
    // .map(|s| s.to_string())
    // .collect::<Vec<_>>();
    println!("{}", part1(&lines));
    println!("{}", part2(&lines));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let lines = vec![
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<_>>();
        let result = part1(&lines);
        assert_eq!(result, 198);
        // let result = part2(&lines);
        // assert_eq!(result, 900);
    }
}
