use aoc_utils_crate::graph::Graph;
use std::cmp::{Ordering, PartialEq};
use std::collections::{HashMap, VecDeque};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum AntiPodType {
    A,
    B,
    C,
    D,
}

#[derive(Debug, Copy, Clone, Hash, Eq)]
struct AntiPod {
    anti_pod_type: AntiPodType,
    pos: (usize, usize),
}

impl PartialEq<Self> for AntiPod {
    fn eq(&self, other: &Self) -> bool {
        self.anti_pod_type == other.anti_pod_type && self.pos == other.pos
    }
}

impl PartialOrd<Self> for AntiPod {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for AntiPod {
    fn cmp(&self, other: &Self) -> Ordering {
        self.get_energy().cmp(&other.get_energy())
    }
}
impl AntiPod {
    fn new_amber(pos: (usize, usize)) -> Self {
        AntiPod {
            anti_pod_type: AntiPodType::A,
            pos,
        }
    }
    fn new_bronze(pos: (usize, usize)) -> Self {
        AntiPod {
            anti_pod_type: AntiPodType::B,
            pos,
        }
    }
    fn new_copper(pos: (usize, usize)) -> Self {
        AntiPod {
            anti_pod_type: AntiPodType::C,
            pos,
        }
    }

    fn new_desert(pos: (usize, usize)) -> Self {
        AntiPod {
            anti_pod_type: AntiPodType::D,
            pos,
        }
    }

    fn get_energy(&self) -> usize {
        match self.anti_pod_type {
            AntiPodType::A => 1,
            AntiPodType::B => 10,
            AntiPodType::C => 100,
            AntiPodType::D => 1000,
        }
    }
    fn get_target_pos_x(&self) -> usize {
        match self.anti_pod_type {
            AntiPodType::A => 3,
            AntiPodType::B => 5,
            AntiPodType::C => 7,
            AntiPodType::D => 9,
        }
    }
    fn get_map_symbol(&self) -> char {
        match self.anti_pod_type {
            AntiPodType::A => 'A',
            AntiPodType::B => 'B',
            AntiPodType::C => 'C',
            AntiPodType::D => 'D',
        }
    }

    fn get_target_pos_y(&self) -> Vec<usize> {
        vec![2, 3]
    }

    fn is_in_correct_spot(&self, anti_pods: &Vec<AntiPod>) -> bool {
        if self.pos.0 != self.get_target_pos_x() {
            return false;
        }
        if self.pos.1 == 3 {
            return true;
        }
        let binding = anti_pods
            .iter()
            .filter(|a| *a != self && a.anti_pod_type == self.anti_pod_type)
            .collect::<Vec<_>>();

        let other_anti_pod = binding.first().unwrap();

        // other one is below us in 3
        if other_anti_pod.pos.0 == self.get_target_pos_x() {
            return other_anti_pod.pos.1 == 3;
        }
        false
    }

    fn is_target_spot_free(&self, anti_pods: &Vec<AntiPod>) -> bool {
        if is_free_on_map(self, anti_pods, (self.get_target_pos_x(), 3)) {
            return true;
        }

        let binding = anti_pods
            .iter()
            .filter(|a| *a != self && a.anti_pod_type == self.anti_pod_type)
            .collect::<Vec<_>>();

        let other_anti_pod = binding.first().unwrap();
        if other_anti_pod.pos.0 == self.get_target_pos_x() && other_anti_pod.pos.1 == 3 {
            return true;
        }
        false
    }

    // empty list if currently not possible
    fn get_moves_into_target_spot(&self, anti_pods: &Vec<AntiPod>) -> Vec<(usize, usize)> {
        let mut moves = vec![];

        if !self.is_target_spot_free(anti_pods) {
            return vec![];
        }

        if self.pos.0 > self.get_target_pos_x() {
            for x in (self.get_target_pos_x()..=self.pos.0 - 1).rev() {
                let pos = (x, 1);
                if is_free_on_map(&self, anti_pods, pos) {
                    moves.push(pos)
                } else {
                    return vec![];
                }
            }
        } else {
            for x in self.pos.0 + 1..=self.get_target_pos_x() {
                let pos = (x, 1);
                if is_free_on_map(&self, anti_pods, pos) {
                    moves.push(pos)
                } else {
                    return vec![];
                }
            }
        }

        if is_free_on_map(&self, anti_pods, (self.get_target_pos_x(), 2)) {
            moves.push((self.get_target_pos_x(), 2));
        } else {
            // should not happen
            return vec![];
        }
        if is_free_on_map(&self, anti_pods, (self.get_target_pos_x(), 3)) {
            moves.push((self.get_target_pos_x(), 3));
        }

        moves
    }

    fn can_leave_slot_left(&self, anti_pods: &Vec<AntiPod>) -> bool {
        is_free_on_map(self, anti_pods, (self.pos.0 - 1, 1))
    }

    fn can_leave_slot_right(&self, anti_pods: &Vec<AntiPod>) -> bool {
        is_free_on_map(self, anti_pods, (self.pos.0 + 1, 1))
    }

    fn get_possible_moves(&self, anti_pods: &Vec<AntiPod>) -> Vec<Vec<(usize, usize)>> {
        if self.is_in_correct_spot(anti_pods) {
            return vec![];
        }
        let mut all_moves = vec![];

        if self.is_on_hallway() {
            let moves = self.get_moves_into_target_spot(anti_pods);
            if !moves.is_empty() {
                all_moves.push(moves);
            }
            return all_moves;
        }

        let mut forbidden_hallway_pos = vec![(3, 1), (5, 1), (7, 1), (9, 1)];

        if self.pos.1 == 2 {
            // can only move out if left or right is free
            if self.can_leave_slot_right(anti_pods) {
                let mut moves = vec![];
                // left or right
                for x in self.pos.0..=11 {
                    let pos = (x, 1);
                    if forbidden_hallway_pos.contains(&pos) {
                        if !is_free_on_map(&self, anti_pods, (pos.0 + 1, 1)) {
                            break;
                        }
                    }
                    if is_free_on_map(&self, anti_pods, pos) {
                        moves.push(pos)
                    }
                }
                if !moves.is_empty() {
                    all_moves.push(moves);
                }
            }
            if self.can_leave_slot_left(anti_pods) {
                let mut moves = vec![];
                for x in (1..=self.pos.0).rev() {
                    let pos = (x, 1);
                    if forbidden_hallway_pos.contains(&pos) {
                        if !is_free_on_map(&self, anti_pods, (pos.0 - 1, 1)) {
                            break;
                        }
                    }
                    if is_free_on_map(&self, anti_pods, pos) {
                        moves.push(pos)
                    }
                }
                if !moves.is_empty() {
                    all_moves.push(moves);
                }
            }
        } else {
            // cant leave yet
            if !is_free_on_map(&self, anti_pods, (self.pos.0, 2)) {
                return vec![];
            } else {
                if self.can_leave_slot_right(anti_pods) {
                    let mut moves = vec![];
                    moves.push((self.pos.0, 2));
                    // left or right
                    for x in self.pos.0..=11 {
                        let pos = (x, 1);
                        if forbidden_hallway_pos.contains(&pos) {
                            if !is_free_on_map(&self, anti_pods, (pos.0 + 1, 1)) {
                                break;
                            }
                        }
                        if is_free_on_map(&self, anti_pods, pos) {
                            moves.push(pos)
                        }
                    }
                    if !moves.is_empty() {
                        all_moves.push(moves);
                    }
                }
                if self.can_leave_slot_left(anti_pods) {
                    let mut moves = vec![];
                    moves.push((self.pos.0, 2));

                    for x in (1..=self.pos.0).rev() {
                        let pos = (x, 1);
                        if forbidden_hallway_pos.contains(&pos) {
                            if !is_free_on_map(&self, anti_pods, (pos.0 - 1, 1)) {
                                break;
                            }
                        }
                        if is_free_on_map(&self, anti_pods, pos) {
                            moves.push(pos)
                        }
                    }
                    if !moves.is_empty() {
                        all_moves.push(moves);
                    }
                }
            }
        }
        all_moves
    }

    fn is_on_hallway(&self) -> bool {
        let mut possible_hallway_pos =
            vec![(1, 1), (2, 1), (4, 1), (6, 1), (8, 1), (10, 1), (11, 1)];
        possible_hallway_pos.contains(&self.pos)
    }
}

fn is_free_on_map(anti_pod: &AntiPod, anti_pods: &Vec<AntiPod>, pos: (usize, usize)) -> bool {
    anti_pods
        .iter()
        .filter(|a| *a != anti_pod && a.pos == pos)
        .count()
        == 0
}

fn print_map(anti_pods: &Vec<AntiPod>) {
    let all_hallway_pos = vec![
        (1, 1),
        (2, 1),
        (3, 1),
        (4, 1),
        (5, 1),
        (6, 1),
        (7, 1),
        (8, 1),
        (9, 1),
        (10, 1),
        (11, 1),
    ];
    let all_slot_pos = vec![
        (3, 2),
        (3, 3),
        (5, 2),
        (5, 3),
        (7, 2),
        (7, 3),
        (9, 2),
        (9, 3),
    ];
    for y in 0..=4 {
        for x in 0..=12 {
            let pos = (x, y);
            if y == 0 {
                print!("#")
            } else {
                if let Some(anti_pod) = anti_pods
                    .iter()
                    .filter(|a| a.pos == pos)
                    .collect::<Vec<_>>()
                    .first()
                {
                    print!("{}", anti_pod.get_map_symbol());
                } else if all_hallway_pos.contains(&pos) || all_slot_pos.contains(&pos) {
                    print!(".")
                } else {
                    print!("#")
                }
            }
        }
        println!();
    }
    println!();
}

fn add_graph_edges(
    graph: &mut Graph<(usize, usize)>,
    anti_pod: &AntiPod,
    anti_pods: &Vec<AntiPod>,
    moves: &Vec<(usize, usize)>,
) -> Vec<((usize, usize), (usize, usize))> {
    let mut forbidden_hallway_pos = vec![(3, 1), (5, 1), (7, 1), (9, 1)];

    let mut edges = vec![];
    let m = anti_pod.pos;
    let mut length = 0;
    for n in moves.iter() {
        let is_out_off_slot_move = m.1 == 3 && n.1 == 2;
        length += 1;

        let is_in_slot_move = n.0 == anti_pod.get_target_pos_x()
            && n.1 == 2
            && is_free_on_map(anti_pod, anti_pods, (n.0, 3));

        if !forbidden_hallway_pos.contains(n) && !is_out_off_slot_move && !is_in_slot_move {
            // println!("add_edge {:?} -> {:?} energy = {}", m, *n, (anti_pod.get_energy() * length) as i32);
            graph.add_edge(m, *n, (anti_pod.get_energy() * length) as i32);
        }
    }
    edges
}

fn build_graph(antipods: &Vec<AntiPod>) -> Graph<(usize, usize)> {
    let mut graph: Graph<(usize, usize)> = Graph {
        nodes: HashMap::new(),
    };

    for a in antipods.iter() {
        let moves = a.get_possible_moves(&antipods);
        // println!("anti_pod {:?} moves {:?}", a, moves);
        for m in moves.iter() {
            add_graph_edges(&mut graph, &a, antipods, &m);
        }
    }
    graph
}

#[derive(Clone, Hash, Eq, PartialEq)]
struct State {
    anti_pods: Vec<AntiPod>,
}

fn dijkstra(anti_pods: &Vec<AntiPod>) -> i32 {
    let mut stack: VecDeque<(State, i32)> = VecDeque::new();
    let s = State {
        anti_pods: anti_pods.clone(),
    };
    stack.push_back((s.clone(), 0));

    let mut seen: HashMap<State, i32> = HashMap::new();
    seen.insert(s.clone(), 0);

    let mut min = i32::MAX;

    while let Some(ref current) = stack.pop_front() {
        let anti_pods = &current.0.anti_pods;
        let energy = current.1;

        if is_solved(&anti_pods) {
            if energy < min {
                println!("{}", energy);
                print_map(&anti_pods);
                min = energy
            }
            continue;
        }

        let g = build_graph(&anti_pods);
        for a in anti_pods.iter() {
            if let Some(node) = g.nodes.get(&a.pos) {
                for e in node.edges.borrow().iter() {
                    let mut anti_pods_copy = anti_pods.clone();
                    for a_copy in anti_pods_copy.iter_mut() {
                        if a_copy.pos == a.pos {
                            a_copy.pos = e.0;
                            break;
                        }
                    }

                    let s_new = State {
                        anti_pods: anti_pods_copy.clone(),
                    };
                    let dist_next_pos = *seen.get(&s_new).unwrap_or(&i32::MAX);
                    if energy + e.1 < dist_next_pos {
                        seen.insert(s_new.clone(), energy + e.1);
                        stack.push_back((s_new, energy + e.1));
                    }
                }
            }
        }
    }
    min
}

fn is_solved(anti_pods: &Vec<AntiPod>) -> bool {
    anti_pods
        .iter()
        .filter(|a| a.is_in_correct_spot(anti_pods))
        .count()
        == 8
}

fn part1(lines: &[String]) -> i32 {
    // 18282
    let mut antipods = vec![];

    // #############
    // #...........#
    // ###C#C#A#B###
    //   #D#D#B#A#
    // #########
    // antipods.push(AntiPod::new_amber((7, 2)));
    // antipods.push(AntiPod::new_amber((9, 3)));
    //
    // antipods.push(AntiPod::new_bronze((9, 2)));
    // antipods.push(AntiPod::new_bronze((7, 3)));
    //
    // antipods.push(AntiPod::new_copper((3, 2)));
    // antipods.push(AntiPod::new_copper((5, 2)));
    //
    // antipods.push(AntiPod::new_desert((3, 3)));
    // antipods.push(AntiPod::new_desert((5, 3)));

    antipods.push(AntiPod::new_amber((3, 3)));
    antipods.push(AntiPod::new_amber((9, 3)));

    antipods.push(AntiPod::new_bronze((3, 2)));
    antipods.push(AntiPod::new_bronze((7, 2)));

    antipods.push(AntiPod::new_copper((5, 2)));
    antipods.push(AntiPod::new_copper((7, 3)));

    antipods.push(AntiPod::new_desert((5, 3)));
    antipods.push(AntiPod::new_desert((9, 2)));

    antipods.sort();

    print_map(&antipods);

    // let mut graph: Graph<(usize, usize)> = Graph {
    //     nodes: HashMap::new(),
    // };
    //
    // for a in antipods.iter() {
    //     let moves = a.get_possible_moves(&antipods);
    //     println!("{:?} {:?}", a, moves);
    //     for m in moves.iter() {
    //         add_graph_edges(&mut graph, &a, &m);
    //     }
    // }

    dijkstra(&antipods)
}

fn part2(lines: &[String]) -> usize {
    0
}

fn main() {
    // let lines = read_lines_as_vec("input/input_day23.txt").unwrap();

    let lines = vec![
        "#############",
        "#...........#",
        "###B#C#B#D###",
        "  #A#D#C#A#",
        "  #########",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect::<Vec<_>>();
    println!("{}", part1(&lines));
    println!("{}", part2(&lines));
}

// #[cfg(test)]
// mod tests {
//     use crate::{part1, part2};
//
//     #[test]
//     fn it_works() {
//         let lines = vec![
//             "on x=-20..26,y=-36..17,z=-47..7",
//             "on x=-20..33,y=-21..23,z=-26..28",
//             "on x=-22..28,y=-29..23,z=-38..16",
//             "on x=-46..7,y=-6..46,z=-50..-1",
//             "on x=-49..1,y=-3..46,z=-24..28",
//             "on x=2..47,y=-22..22,z=-23..27",
//             "on x=-27..23,y=-28..26,z=-21..29",
//             "on x=-39..5,y=-6..47,z=-3..44",
//             "on x=-30..21,y=-8..43,z=-13..34",
//             "on x=-22..26,y=-27..20,z=-29..19",
//             "off x=-48..-32,y=26..41,z=-47..-37",
//             "on x=-12..35,y=6..50,z=-50..-2",
//             "off x=-48..-32,y=-32..-16,z=-15..-5",
//             "on x=-18..26,y=-33..15,z=-7..46",
//             "off x=-40..-22,y=-38..-28,z=23..41",
//             "on x=-16..35,y=-41..10,z=-47..6",
//             "off x=-32..-23,y=11..30,z=-14..3",
//             "on x=-49..-5,y=-3..45,z=-29..18",
//             "off x=18..30,y=-20..-8,z=-3..13",
//             "on x=-41..9,y=-7..43,z=-33..15",
//             "on x=-54112..-39298,y=-85059..-49293,z=-27449..7877",
//             "on x=967..23432,y=45373..81175,z=27513..53682",
//         ]
//         .iter()
//         .map(|s| s.to_string())
//         .collect::<Vec<_>>();
//         assert_eq!(part1(&lines), 590784);
//         assert_eq!(part2(&lines), 39769202357779);
//     }
// }
