use aoc_2021::{read_lines_as_vec, Graph};
use std::collections::{HashMap, HashSet, VecDeque};

fn count_small_caves(path: &Vec<&str>) -> bool {
    let mut small_caves = HashSet::new();
    for p in path.iter() {
        let is_lowercase = p.chars().all(|c| c.is_lowercase());
        if is_lowercase {
            if small_caves.contains(p) {
                return false;
            }
            small_caves.insert(p);
        }
    }
    true
}

fn bfs(graph: &Graph<&str>, start: &str, end: &str) -> usize {
    let mut stack = VecDeque::new();
    stack.push_back(vec![start]);

    let mut seen: HashMap<&str, u32> = HashMap::new();
    seen.insert(start, 0);

    let mut path_list = vec![];
    while let Some(path) = stack.pop_front() {
        let tail = *path.last().unwrap();
        if tail == end {
            // println!("{:?}", path);
            path_list.push(path);
            continue;
        }

        let possible_steps = graph.nodes.get(&tail).unwrap().edges.borrow();
        for (move_pos, _) in possible_steps.iter() {
            if *move_pos != start {
                let mut p = path.clone();
                p.push(move_pos);

                if count_small_caves(&path) {
                    stack.push_back(p);
                }
            }
        }
    }
    path_list.len()
}

fn part1(lines: &[String]) -> usize {
    // 3510
    let mut graph: Graph<&str> = Graph {
        nodes: HashMap::new(),
    };

    for line in lines.iter() {
        let parts = line.split("-").collect::<Vec<_>>();
        let n1 = parts.first().unwrap();
        let n2 = parts.last().unwrap();

        graph.add_edge(n1, n2, 1);
        graph.add_edge(n2, n1, 1);
    }

    bfs(&graph, "start", "end")
}

fn part2(lines: &[String]) -> usize {
    0usize
}

fn main() {
    let lines = read_lines_as_vec("input/input_day12.txt").unwrap();

    // let lines = vec!["start-A", "start-b", "A-c", "A-b", "b-d", "A-end", "b-end"]
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
        let lines = vec!["start-A", "start-b", "A-c", "A-b", "b-d", "A-end", "b-end"]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();

        let result = part1(&lines);
        assert_eq!(result, 10);
        // let result = part2(&lines);
        // assert_eq!(result, 195);
    }
}
