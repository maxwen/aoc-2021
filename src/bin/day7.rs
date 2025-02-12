use aoc_2021::read_lines_as_vec;

fn part1(lines: &[String]) -> u32 {
    // 337488
    let line = lines[0].to_string();

    let crabs_pos = line
        .split(",")
        .map(|f| f.parse().unwrap())
        .collect::<Vec<u32>>();

    let min_pos = crabs_pos.iter().min().unwrap();
    let max_pos = crabs_pos.iter().max().unwrap();

    let mut min_cost = u32::MAX;

    for i in *min_pos..max_pos + 1 {
        let mut sum = 0;
        for c in crabs_pos.iter() {
            let cost = i.abs_diff(*c);
            sum += cost;
        }
        if sum < min_cost {
            min_cost = sum;
        }
    }

    min_cost
}

fn part2(lines: &[String]) -> u32 {
    // 89647695
    let line = lines[0].to_string();

    let crabs_pos = line
        .split(",")
        .map(|f| f.parse().unwrap())
        .collect::<Vec<u32>>();

    let min_pos = crabs_pos.iter().min().unwrap();
    let max_pos = crabs_pos.iter().max().unwrap();

    let mut min_cost = u32::MAX;

    for i in *min_pos..max_pos + 1 {
        let mut sum = 0;
        for c in crabs_pos.iter() {
            let mut cost = i.abs_diff(*c);
            // https://en.wikipedia.org/wiki/1_%2B_2_%2B_3_%2B_4_%2B_%E2%8B%AF
            cost = cost * (cost + 1) / 2;
            sum += cost;
        }
        if sum < min_cost {
            min_cost = sum;
        }
    }

    min_cost
}

fn main() {
    let lines = read_lines_as_vec("input/input_day7.txt").unwrap();

    // let lines = vec!["16,1,2,0,4,2,7,1,2,14"]
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
        let lines = vec!["16,1,2,0,4,2,7,1,2,14"]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();

        let result = part1(&lines);
        assert_eq!(result, 37);
        let result = part2(&lines);
        assert_eq!(result, 168);
    }
}
