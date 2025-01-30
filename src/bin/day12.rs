use aoc_utils_crate::graph::Graph;
use std::collections::{HashMap, HashSet, VecDeque};
use aoc_utils_crate::file::read_lines_as_vec;

// every small only once
fn test_small_caves_once(path: &Vec<&str>) -> bool {
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

fn has_any_small_cave_twice<'a>(small_caves: &'a Vec<&'a str>) -> Option<&'a str> {
    for small_cave in small_caves.iter() {
        let count = small_caves.iter().filter(|c| ***c == **small_cave).count();
        if count == 2 {
            return Some(small_cave);
        }
    }
    None
}
// one small twice - rest only once
fn test_small_caves_twice(path: &Vec<&str>) -> bool {
    let mut small_caves: Vec<&str> = vec![];
    for p in path.iter() {
        let is_lowercase = p.chars().all(|c| c.is_lowercase());
        if is_lowercase {
            if let Some(twice_small_cave) = has_any_small_cave_twice(&small_caves) {
                // would be the third time
                if twice_small_cave == *p {
                    return false;
                }
                // another cave is already in twice so we only allow this once
                let count = small_caves.iter().filter(|c| ***c == **p).count();
                if count == 0 {
                    small_caves.push(p);
                } else {
                    return false;
                }
            } else {
                small_caves.push(p);
            }
        }
    }
    true
}

fn bfs(
    graph: &Graph<&str>,
    start: &str,
    end: &str,
    small_cave_test: fn(path: &Vec<&str>) -> bool,
) -> usize {
    let mut stack = VecDeque::new();
    stack.push_back(vec![start]);

    let mut seen: HashMap<&str, u32> = HashMap::new();
    seen.insert(start, 0);

    let mut path_count = 0;
    while let Some(path) = stack.pop_front() {
        let tail = *path.last().unwrap();
        if tail == end {
            // println!("{:?}", path);
            path_count += 1;
            continue;
        }

        let possible_steps = graph.nodes.get(&tail).unwrap().edges.borrow();
        for (move_pos, _) in possible_steps.iter() {
            if *move_pos != start {
                let mut p = path.clone();
                p.push(move_pos);

                if small_cave_test(&path) {
                    stack.push_back(p);
                }
            }
        }
    }
    path_count
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

    bfs(&graph, "start", "end", test_small_caves_once)
}

fn part2(lines: &[String]) -> usize {
    // 122880
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

    bfs(&graph, "start", "end", test_small_caves_twice)
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
        let result = part2(&lines);
        assert_eq!(result, 36);
    }
}
