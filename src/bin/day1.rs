use aoc_2021::read_lines_as_vec;

fn part1(lines: &[String]) -> u32 {
    // 1226
    let mut last = 0;
    let mut sum = 0u32;
    for line in lines.iter() {
        let depth: u32 = line.parse().unwrap();
        if last != 0 {
            if depth > last {
                sum += 1;
            }
        }
        last = depth
    }
    sum
}
fn part2(lines: &[String]) -> u32 {
    // 1252
    let mut last = 0;
    let mut sum = 0u32;

    let mut i = 0;
    while i < lines.len() - 2 {
        let slice = &lines[i..i + 3];
        let depth = slice.iter().map(|d| d.parse::<u32>().unwrap()).sum();
        if last != 0 {
            if depth > last {
                sum += 1;
            }
        }
        last = depth;
        i += 1;
    }
    sum
}

fn main() {
    let lines = read_lines_as_vec("input/input_day1.txt").unwrap();

    // let lines = vec![
    //     "199", "200", "208", "210", "200", "207", "240", "269", "260", "263",
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
            "199", "200", "208", "210", "200", "207", "240", "269", "260", "263",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<_>>();

        let result = part1(&lines);
        assert_eq!(result, 7);
        let result = part2(&lines);
        assert_eq!(result, 5);
    }
}
