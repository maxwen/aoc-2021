use aoc_utils_crate::graph::Graph;
use regex::Regex;
use std::cmp::{Ordering, PartialEq};
use std::collections::{HashMap, VecDeque};
use aoc_2021::read_lines_as_vec;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum AntiPodType {
    A,
    B,
    C,
    D,
}

impl TryFrom<&str> for AntiPodType {
    type Error = ();

    fn try_from(c: &str) -> Result<Self, Self::Error> {
        match c {
            "A" => Ok(AntiPodType::A),
            "B" => Ok(AntiPodType::B),
            "C" => Ok(AntiPodType::C),
            "D" => Ok(AntiPodType::D),
            _ => Err(()),
        }
    }
}

struct AntiPodRoom {
    slot_pos: Vec<(usize, usize)>,
    hallway_pos: Vec<(usize, usize)>,
    forbidden_hallway_pos: Vec<(usize, usize)>,
    slot_start: usize,
    slot_height: usize,
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

    fn is_in_slot(&self, room: &AntiPodRoom) -> bool {
        (room.slot_start..room.slot_start + room.slot_height).contains(&self.pos.1)
    }

    fn can_leave_slot(&self, anti_pods: &Vec<AntiPod>, room: &AntiPodRoom) -> bool {
        (room.slot_start..self.pos.1)
            .filter(|p| !is_free_on_map(self, anti_pods, (self.pos.0, *p)))
            .count()
            == 0
    }

    fn get_next_free_slot_pos(
        &self,
        anti_pods: &Vec<AntiPod>,
        room: &AntiPodRoom,
    ) -> Option<(usize, usize)> {
        for y in room.slot_start..room.slot_start + room.slot_height {
            if is_free_on_map(self, anti_pods, (self.get_target_pos_x(), y)) {
                return Some((self.get_target_pos_x(), y));
            }
        }
        None
    }

    fn is_in_correct_spot(&self, anti_pods: &Vec<AntiPod>, room: &AntiPodRoom) -> bool {
        if self.pos.0 != self.get_target_pos_x() {
            return false;
        }

        for y in room.slot_start..room.slot_start + room.slot_height {
            let pos = (self.get_target_pos_x(), y);
            if !is_free_on_map(self, anti_pods, pos)
                && !is_used_by_type(self, self.anti_pod_type, anti_pods, pos) {
                return false;
            }
        }
        true
    }

    fn is_target_spot_free(&self, anti_pods: &Vec<AntiPod>, room: &AntiPodRoom) -> bool {
        let next_free_slot = self.get_next_free_slot_pos(anti_pods, room);
        if next_free_slot.is_none() {
            return false;
        }

        for y in next_free_slot.unwrap().1..room.slot_start + room.slot_height {
            let pos = (self.get_target_pos_x(), y);
            if !is_free_on_map(self, anti_pods, pos)
                && !is_used_by_type(self, self.anti_pod_type, anti_pods, pos)
            {
                return false;
            }
        }
        true
    }

    // empty list if currently not possible
    fn get_moves_into_target_spot(
        &self,
        anti_pods: &Vec<AntiPod>,
        room: &AntiPodRoom,
    ) -> Vec<(usize, usize)> {
        let mut moves = vec![];

        if !self.is_target_spot_free(anti_pods, room) {
            return vec![];
        }

        let next_free_slot = self.get_next_free_slot_pos(anti_pods, room);
        if next_free_slot.is_none() {
            // should not happen
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

        for y in room.slot_start..=next_free_slot.unwrap().1 {
            moves.push((self.get_target_pos_x(), y));
        }

        moves
    }

    fn can_leave_slot_left(&self, anti_pods: &Vec<AntiPod>) -> bool {
        is_free_on_map(self, anti_pods, (self.pos.0 - 1, 1))
    }

    fn can_leave_slot_right(&self, anti_pods: &Vec<AntiPod>) -> bool {
        is_free_on_map(self, anti_pods, (self.pos.0 + 1, 1))
    }

    fn get_possible_moves(
        &self,
        anti_pods: &Vec<AntiPod>,
        room: &AntiPodRoom,
    ) -> Vec<Vec<(usize, usize)>> {
        if self.is_in_correct_spot(anti_pods, room) {
            return vec![];
        }
        let mut all_moves = vec![];

        if self.is_on_hallway(room) {
            let moves = self.get_moves_into_target_spot(anti_pods, room);
            if !moves.is_empty() {
                all_moves.push(moves);
            }
            return all_moves;
        }

        if self.is_in_slot(room) && self.can_leave_slot(anti_pods, room) {
            // can only move out if left or right is free
            if self.can_leave_slot_right(anti_pods) {
                let mut moves = vec![];
                // add steps out of slot
                (room.slot_start..self.pos.1)
                    .rev()
                    .for_each(|p| moves.push((self.pos.0, p)));

                // left or right
                for x in self.pos.0..=11 {
                    let pos = (x, 1);
                    // can we step over forbidden slot
                    if room.forbidden_hallway_pos.contains(&pos) {
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
                // add steps out of slot
                (room.slot_start..self.pos.1)
                    .rev()
                    .for_each(|p| moves.push((self.pos.0, p)));

                for x in (1..=self.pos.0).rev() {
                    let pos = (x, 1);
                    if room.forbidden_hallway_pos.contains(&pos) {
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
        all_moves
    }

    fn is_on_hallway(&self, room: &AntiPodRoom) -> bool {
        room.hallway_pos.contains(&self.pos)
    }
}

fn is_free_on_map(anti_pod: &AntiPod, anti_pods: &Vec<AntiPod>, pos: (usize, usize)) -> bool {
    anti_pods
        .iter()
        .filter(|a| *a != anti_pod && a.pos == pos)
        .count()
        == 0
}

fn is_used_by_type(
    anti_pod: &AntiPod,
    anti_pod_type: AntiPodType,
    anti_pods: &Vec<AntiPod>,
    pos: (usize, usize),
) -> bool {
    anti_pods
        .iter()
        .filter(|a| *a != anti_pod && a.pos == pos && a.anti_pod_type == anti_pod_type)
        .count()
        == 1
}

#[allow(dead_code)]
fn print_map(anti_pods: &Vec<AntiPod>, anti_pod_room: &AntiPodRoom) {
    for y in 0..=2 + anti_pod_room.slot_height {
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
                } else if anti_pod_room.hallway_pos.contains(&pos)
                    || anti_pod_room.slot_pos.contains(&pos)
                {
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
    room: &AntiPodRoom,
) {
    let m = anti_pod.pos;
    let next_free_slot = anti_pod.get_next_free_slot_pos(anti_pods, room);
    let mut length = 0;
    for n in moves.iter() {
        length += 1;

        // dont add any edge that is not leaving the slot
        let is_out_off_slot_move = (room.slot_start..room.slot_start + room.slot_height)
            .contains(&m.1)
            && n.1 == room.slot_start;

        let mut is_in_slot_move = false;
        // dont add any edge before final position
        if n.0 == anti_pod.get_target_pos_x() {
            if let Some(pos) = next_free_slot {
                if n.1 != pos.1 {
                    is_in_slot_move = true
                }
            }
        }

        if !room.forbidden_hallway_pos.contains(n) && !is_out_off_slot_move && !is_in_slot_move {
            // println!("add_edge {:?} -> {:?} energy = {}", m, *n, (anti_pod.get_energy() * length) as i32);
            graph.add_edge(m, *n, (anti_pod.get_energy() * length) as i32);
        }
    }
}

fn build_graph(antipods: &Vec<AntiPod>, room: &AntiPodRoom) -> Graph<(usize, usize)> {
    let mut graph: Graph<(usize, usize)> = Graph {
        nodes: HashMap::new(),
    };

    for a in antipods.iter() {
        let moves = a.get_possible_moves(antipods, room);
        // println!("anti_pod {:?} moves {:?}", a, moves);
        for m in moves.iter() {
            add_graph_edges(&mut graph, &a, antipods, &m, room);
        }
    }
    graph
}

#[derive(Clone, Hash, Eq, PartialEq)]
struct State {
    anti_pods: Vec<AntiPod>,
}

fn dijkstra(anti_pods: &Vec<AntiPod>, room: &AntiPodRoom) -> i32 {
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

        if is_solved(&anti_pods, room) {
            if energy < min {
                // print_map(&anti_pods, &room);
                min = energy
            }
            continue;
        }

        let g = build_graph(&anti_pods, &room);
        for a in anti_pods.iter() {
            if let Some(node) = g.nodes.get(&a.pos) {
                for e in node.edges.borrow().iter() {
                    let mut new_anti_pods = anti_pods.clone();
                    // move one antipod
                    for a_copy in new_anti_pods.iter_mut() {
                        if a_copy.pos == a.pos {
                            a_copy.pos = e.0;
                            break;
                        }
                    }

                    let s_new = State {
                        anti_pods: new_anti_pods,
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

fn is_solved(anti_pods: &Vec<AntiPod>, room: &AntiPodRoom) -> bool {
    anti_pods
        .iter()
        .filter(|a| a.is_in_correct_spot(anti_pods, room))
        .count()
        == anti_pods.len()
}

fn part1(lines: &[String]) -> i32 {
    // 18282
    let room = AntiPodRoom {
        slot_pos: vec![
            (3, 2),
            (3, 3),
            (5, 2),
            (5, 3),
            (7, 2),
            (7, 3),
            (9, 2),
            (9, 3),
        ],
        hallway_pos: vec![
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
        ],
        forbidden_hallway_pos: vec![(3, 1), (5, 1), (7, 1), (9, 1)],
        slot_start: 2,
        slot_height: 2,
    };

    let mut antipods = vec![];

    let anti_pod_pos = Regex::new(r"[ABCD]").unwrap();
    for (y, line) in lines.iter().enumerate() {
        let n = anti_pod_pos.find_iter(line).collect::<Vec<_>>();
        if !n.is_empty() {
            for m in n.iter() {
                let c = m.as_str();
                let x = m.start();
                match AntiPodType::try_from(c) {
                    Ok(anti_pod_type) => match anti_pod_type {
                        AntiPodType::A => {
                            antipods.push(AntiPod::new_amber((x, y)));
                        }
                        AntiPodType::B => {
                            antipods.push(AntiPod::new_bronze((x, y)));
                        }
                        AntiPodType::C => {
                            antipods.push(AntiPod::new_copper((x, y)));
                        }
                        AntiPodType::D => {
                            antipods.push(AntiPod::new_desert((x, y)));
                        }
                    },
                    Err(_) => {}
                }
            }
        }
    }
    antipods.sort();

    dijkstra(&antipods, &room)
}

fn part2(lines: &Vec<String>) -> i32 {
    // 50132
    let room = AntiPodRoom {
        slot_pos: vec![
            (3, 2),
            (3, 3),
            (3, 4),
            (3, 5),
            (5, 2),
            (5, 3),
            (5, 4),
            (5, 5),
            (7, 2),
            (7, 3),
            (7, 4),
            (7, 5),
            (9, 2),
            (9, 3),
            (9, 4),
            (9, 5),
        ],
        hallway_pos: vec![
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
        ],
        forbidden_hallway_pos: vec![(3, 1), (5, 1), (7, 1), (9, 1)],
        slot_start: 2,
        slot_height: 4,
    };

    let mut antipods = vec![];

    let mut lines = lines.clone();
    lines.insert(3, "  #D#C#B#A#".to_string());
    lines.insert(4, "  #D#B#A#C#".to_string());

    let anti_pod_pos = Regex::new(r"[ABCD]").unwrap();
    for (y, line) in lines.iter().enumerate() {
        let n = anti_pod_pos.find_iter(line).collect::<Vec<_>>();
        if !n.is_empty() {
            for m in n.iter() {
                let c = m.as_str();
                let x = m.start();
                match AntiPodType::try_from(c) {
                    Ok(anti_pod_type) => match anti_pod_type {
                        AntiPodType::A => {
                            antipods.push(AntiPod::new_amber((x, y)));
                        }
                        AntiPodType::B => {
                            antipods.push(AntiPod::new_bronze((x, y)));
                        }
                        AntiPodType::C => {
                            antipods.push(AntiPod::new_copper((x, y)));
                        }
                        AntiPodType::D => {
                            antipods.push(AntiPod::new_desert((x, y)));
                        }
                    },
                    Err(_) => {}
                }
            }
        }
    }

    antipods.sort();

    // print_map(&antipods, &room);

    dijkstra(&antipods, &room)
}

fn main() {
    let lines = read_lines_as_vec("input/input_day23.txt").unwrap();

    // let lines = vec![
    //     "#############",
    //     "#...........#",
    //     "###B#C#B#D###",
    //     "  #A#D#C#A#",
    //     "  #########",
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
            "#############",
            "#...........#",
            "###B#C#B#D###",
            "  #A#D#C#A#",
            "  #########",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<_>>();
        assert_eq!(part1(&lines), 12521);
        assert_eq!(part2(&lines), 44169);
    }
}
