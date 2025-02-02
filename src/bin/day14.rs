use aoc_2021::read_lines_as_vec;
use std::collections::HashMap;

fn part1(lines: &[String]) -> usize {
    // 3058
    let init_polymer = lines[0].chars().collect::<Vec<_>>();
    let mut rules: HashMap<(char, char), char> = HashMap::new();

    for i in 2..lines.len() {
        let line = lines[i].to_string();

        let rules_pair = line.split(" -> ").collect::<Vec<_>>();
        let pair = rules_pair.first().unwrap().chars().collect::<Vec<_>>();
        let insert = rules_pair.last().unwrap();

        rules.insert(
            (*pair.first().unwrap(), *pair.last().unwrap()),
            insert.chars().nth(0).unwrap(),
        );
    }

    let mut polymer_list = vec![];
    let mut polymer_map: HashMap<(char, char), usize> = HashMap::new();
    for i in 0..init_polymer.len() - 1 {
        let pair = (init_polymer[i], init_polymer[i + 1]);
        polymer_map
            .entry((init_polymer[i], init_polymer[i + 1]))
            .and_modify(|e| *e += 1)
            .or_insert(1);
        polymer_list.push(pair);
    }

    for _ in 0..10 {
        let mut new_polymer_list = vec![];

        for pair in polymer_list.iter() {
            for (rules_pair, insert) in rules.iter() {
                let new_pair_0 = (rules_pair.0, *insert);
                let new_pair_1 = (*insert, rules_pair.1);

                if rules_pair == pair {
                    new_polymer_list.push(new_pair_0);
                    new_polymer_list.push(new_pair_1);
                }
            }
        }
        polymer_list = new_polymer_list;
    }
    let mut char_map: HashMap<char, usize> = HashMap::new();
    for pair in polymer_list.iter() {
        char_map.entry(pair.0).and_modify(|n| *n += 1).or_insert(1);
    }
    let last_pair = polymer_list.last().unwrap();
    char_map
        .entry(last_pair.1)
        .and_modify(|n| *n += 1)
        .or_insert(1);
    let mut v = char_map.values().collect::<Vec<_>>();
    v.sort();

    **v.last().unwrap() - **v.first().unwrap()
}

fn part2(lines: &[String]) -> u64 {
    // 3447389044530
    let init_polymer = lines[0].chars().collect::<Vec<_>>();
    let mut rules: HashMap<(char, char), char> = HashMap::new();

    for i in 2..lines.len() {
        let line = lines[i].to_string();

        let rules_pair = line.split(" -> ").collect::<Vec<_>>();
        let pair = rules_pair.first().unwrap().chars().collect::<Vec<_>>();
        let insert = rules_pair.last().unwrap();

        rules.insert(
            (*pair.first().unwrap(), *pair.last().unwrap()),
            insert.chars().nth(0).unwrap(),
        );
    }

    let mut char_map: HashMap<char, u64> = HashMap::new();
    let mut polymer_map: HashMap<(char, char), u64> = HashMap::new();
    for i in 0..init_polymer.len() - 1 {
        let pair = (init_polymer[i], init_polymer[i + 1]);
        polymer_map
            .entry((init_polymer[i], init_polymer[i + 1]))
            .and_modify(|e| *e += 1)
            .or_insert(1);
        char_map.entry(pair.0).and_modify(|n| *n += 1).or_insert(1);
    }
    char_map
        .entry(init_polymer[init_polymer.len() - 1])
        .and_modify(|n| *n += 1)
        .or_insert(1);

    for _ in 0..40 {
        let mut new_polymer_map: HashMap<(char, char), u64> = HashMap::new();

        for (rules_pair, insert) in rules.iter() {
            let new_pair_0 = (rules_pair.0, *insert);
            let new_pair_1 = (*insert, rules_pair.1);

            if polymer_map.contains_key(rules_pair) {
                let num = polymer_map.get(rules_pair).unwrap();
                char_map.entry(*insert).and_modify(|n| *n += num).or_insert(*num);

                new_polymer_map
                    .entry(new_pair_0)
                    .and_modify(|e| *e += num)
                    .or_insert(*num);

                new_polymer_map
                    .entry(new_pair_1)
                    .and_modify(|e| *e += num)
                    .or_insert(*num);
            }
        }
        polymer_map = new_polymer_map;
    }
    let mut v = char_map.values().collect::<Vec<_>>();
    v.sort();

    **v.last().unwrap() - **v.first().unwrap()
}

fn main() {
    let lines = read_lines_as_vec("input/input_day14.txt").unwrap();

    // let lines = vec![
    //     "NNCB", "", "CH -> B", "HH -> N", "CB -> H", "NH -> C", "HB -> C", "HC -> B", "HN -> C",
    //     "NN -> C", "BH -> H", "NC -> B", "NB -> B", "BN -> B", "BB -> N", "BC -> B", "CC -> N",
    //     "CN -> C",
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
            "NNCB", "", "CH -> B", "HH -> N", "CB -> H", "NH -> C", "HB -> C", "HC -> B",
            "HN -> C", "NN -> C", "BH -> H", "NC -> B", "NB -> B", "BN -> B", "BB -> N", "BC -> B",
            "CC -> N", "CN -> C",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<_>>();

        let result = part1(&lines);
        assert_eq!(result, 1588);
        let result = part2(&lines);
        assert_eq!(result, 2188189693529);
    }
}
