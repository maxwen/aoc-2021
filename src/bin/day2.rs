use aoc_2021::read_lines_as_vec;

fn part1(lines: &[String]) -> u32 {
    // 1990000
    let mut x_pos = 0;
    let mut depth = 0;

    for line in lines.iter() {
        let parts = line.split(" ").collect::<Vec<_>>();
        let cmd = parts[0];
        let value: u32 = parts[1].parse().unwrap();

        match cmd {
            "forward" => x_pos += value,
            "up" => depth -= value,
            "down" => depth += value,
            _ => {}
        }
    }
    x_pos * depth
}
fn part2(lines: &[String]) -> u32 {
    // 1975421260
    let mut x_pos = 0;
    let mut depth = 0;
    let mut aim = 0;

    for line in lines.iter() {
        let parts = line.split(" ").collect::<Vec<_>>();
        let cmd = parts[0];
        let value: u32 = parts[1].parse().unwrap();

        match cmd {
            "forward" => {
                x_pos += value;
                depth += aim * value
            },
            "up" => aim -= value,
            "down" => aim += value,
            _ => {}
        }
    }
    x_pos * depth
}

fn main() {
    let lines = read_lines_as_vec("input/input_day2.txt").unwrap();

    // let lines = vec![
    //     "forward 5",
    //     "down 5",
    //     "forward 8",
    //     "up 3",
    //     "down 8",
    //     "forward 2",
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
            "forward 5",
            "down 5",
            "forward 8",
            "up 3",
            "down 8",
            "forward 2",
        ]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();

        let result = part1(&lines);
        assert_eq!(result, 150);
        let result = part2(&lines);
        assert_eq!(result, 900);
    }
}
