use priority_queue::PriorityQueue;
use std::collections::HashMap;
use aoc_2021::read_lines_as_vec;

type Point = (i16, i16);

fn get_neighbours(grid: &Vec<Vec<u16>>, pos: &Point) -> Vec<Point> {
    let grid_lines = grid.len();
    let grid_cols = grid.first().unwrap().len();

    let n_pos = (pos.0, pos.1);
    [
        (n_pos.0, n_pos.1 + 1),
        (n_pos.0, n_pos.1 - 1),
        (n_pos.0 - 1, n_pos.1),
        (n_pos.0 + 1, n_pos.1),
    ]
        .iter()
        .filter(|pos| pos.0 >= 0 && pos.0 < grid_cols as i16 && pos.1 >= 0 && pos.1 < grid_lines as i16)
        .map(|(r, c)| (*r, *c))
        .collect::<Vec<_>>()
}

fn dijkstra(grid: &Vec<Vec<u16>>, start: Point, end: Point) -> u16 {
    let mut stack = PriorityQueue::new();
    stack.push(start, 0u16);

    let mut seen: HashMap<(i16, i16), u16> = HashMap::new();

    let mut min = u16::MAX;

    while let Some((current, risk_level)) = stack.pop() {
        if current == end {
            if risk_level < min {
                min = risk_level
            }
            continue
        }

        for next_pos in get_neighbours(grid, &current) {
            let next_risk_level = grid[next_pos.1 as usize][next_pos.0 as usize];
            let dist_next_pos = seen.get(&next_pos).unwrap_or(&u16::MAX);
            if risk_level + next_risk_level < *dist_next_pos {
                seen.insert(next_pos, risk_level + next_risk_level);
                stack.push(next_pos, risk_level + next_risk_level);
            }
        }
    }
    min
}

fn part1(lines: &[String]) -> u16 {
    // 403
    let mut grid: Vec<Vec<u16>> = vec![];
    for (_, line) in lines.iter().enumerate() {
        let mut l = vec![];
        for (_, c) in line.chars().enumerate() {
            l.push(c.to_digit(10).unwrap() as u16);
        }
        grid.push(l);
    }
    let grid_lines = grid.len();
    let grid_cols = grid.first().unwrap().len();

    let start: Point = (0, 0);
    let end: Point = (grid_cols as i16 - 1, grid_lines as i16 - 1);

    dijkstra(&grid, start, end)
}

fn part2(lines: &[String]) -> usize {
    0usize
}

fn main() {
    let lines = read_lines_as_vec("input/input_day15.txt").unwrap();

    // let lines = vec![
    //     "1163751742",
    //     "1381373672",
    //     "2136511328",
    //     "3694931569",
    //     "7463417111",
    //     "1319128137",
    //     "1359912421",
    //     "3125421639",
    //     "1293138521",
    //     "2311944581",
    // ]
    //     .iter()
    //     .map(|s| s.to_string())
    //     .collect::<Vec<_>>();
    println!("{}", part1(&lines));
    println!("{}", part2(&lines));
}

#[cfg(test)]
mod tests {
    use crate::part1;

    #[test]
    fn it_works() {
        let lines = vec![
            "1163751742",
            "1381373672",
            "2136511328",
            "3694931569",
            "7463417111",
            "1319128137",
            "1359912421",
            "3125421639",
            "1293138521",
            "2311944581",
        ]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();

        let result = part1(&lines);
        assert_eq!(result, 40);
        // let result = part2(&lines);
        // assert_eq!(result, 2188189693529);
    }
}
