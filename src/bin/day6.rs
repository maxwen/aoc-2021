use aoc_2021::read_lines_as_vec;
use std::collections::HashMap;

// obviously those was a dead end as soon as it
// was known to be exponential
#[allow(dead_code)]
fn part1(lines: &[String], days: u16) -> usize {
    let line = lines[0].to_string();

    let mut fish = line
        .split(",")
        .map(|f| f.parse().unwrap())
        .collect::<Vec<u16>>();

    for _ in 0..days {
        let mut new_fish = vec![];
        let mut new_fish_num = 0;
        for f in fish.iter() {
            match f {
                0 => {
                    new_fish.push(6);
                    new_fish_num += 1;
                }
                _ => new_fish.push(f - 1),
            }
        }
        fish.clear();
        fish.append(&mut new_fish);
        (0..new_fish_num).for_each(|_| fish.push(8));
    }
    fish.len()
}

// we can just keep track of the num of fishes no need
// to store all of them in a long list
// every day the numbers flow down
fn part12(lines: &[String], days: usize) -> usize {
    // 394994
    // 1765974267455
    let line = lines[0].to_string();

    // age -> number of fish with that age
    let mut fish_map: HashMap<usize, usize> = HashMap::new();

    let init_fish = line
        .split(",")
        .map(|f| f.parse().unwrap())
        .collect::<Vec<usize>>();

    for age in init_fish.iter() {
        fish_map.entry(*age).and_modify(|n| *n += 1).or_insert(1);
    }

    for _ in 1..days + 1 {
        let new_fish_num = *fish_map.get(&0).unwrap_or(&0);
        fish_map.remove(&0);

        // move all numbers above 0 one down
        // 8 is then empty
        for age in 1..9 {
            let old_num = *fish_map.get(&age).unwrap_or(&0);

            if fish_map.contains_key(&age) {
                fish_map
                    .entry(age - 1)
                    .and_modify(|n| *n = old_num)
                    .or_insert(old_num);
                fish_map.remove(&age);
            }
        }

        // all old 0 switch to 6 and add the same number to 8
        if new_fish_num != 0 {
            // MUST use += here cause we can have fish
            // from age 7 already in here from before
            fish_map
                .entry(6)
                .and_modify(|n| *n += new_fish_num)
                .or_insert(new_fish_num);
            fish_map
                .entry(8)
                .and_modify(|n| *n = new_fish_num)
                .or_insert(new_fish_num);
        }
    }
    fish_map.values().sum()
}

fn main() {
    let lines = read_lines_as_vec("input/input_day6.txt").unwrap();

    // let lines = vec!["3,4,3,1,2"]
    //     .iter()
    //     .map(|s| s.to_string())
    //     .collect::<Vec<_>>();
    println!("{}", part12(&lines, 80));
    println!("{}", part12(&lines, 256));
}

#[cfg(test)]
mod tests {
    use crate::part12;

    #[test]
    fn it_works() {
        let lines = vec!["3,4,3,1,2"]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();

        let result = part12(&lines, 80);
        assert_eq!(result, 5934);
        let result = part12(&lines, 256);
        assert_eq!(result, 26984457539);
    }
}
