use aoc_2021::read_lines_as_vec;
use std::collections::HashMap;

fn part12(lines: &[String], steps: usize) -> u64 {
    // 3058
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

    for _ in 0..steps {
        let mut new_polymer_map: HashMap<(char, char), u64> = HashMap::new();

        for (rules_pair, insert) in rules.iter() {
            let new_pair_0 = (rules_pair.0, *insert);
            let new_pair_1 = (*insert, rules_pair.1);

            if polymer_map.contains_key(rules_pair) {
                let num = *polymer_map.get(rules_pair).unwrap();
                char_map.entry(*insert).and_modify(|n| *n += num).or_insert(num);

                new_polymer_map
                    .entry(new_pair_0)
                    .and_modify(|e| *e += num)
                    .or_insert(num);

                new_polymer_map
                    .entry(new_pair_1)
                    .and_modify(|e| *e += num)
                    .or_insert(num);
            }
        }
        polymer_map = new_polymer_map;
    }
    let mut v = char_map.values().map(|v| *v).collect::<Vec<_>>();
    v.sort();

    v.last().unwrap() - v.first().unwrap()
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
    println!("{}", part12(&lines, 10));
    println!("{}", part12(&lines, 40));
}

#[cfg(test)]
mod tests {
    use crate::{part12};

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

        let result = part12(&lines, 10);
        assert_eq!(result, 1588);
        let result = part12(&lines, 40);
        assert_eq!(result, 2188189693529);
    }
}
