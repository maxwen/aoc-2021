use std::collections::{HashMap, HashSet};
use aoc_2021::read_lines_as_vec;

type Point = (i16, i16);

fn get_neighbours(grid_cols: usize, grid_lines: usize, pos: &Point) -> Vec<Point> {
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

// https://www.blog.findlayian.com/posts/aoc2021-day15
fn dijkstra_mod(
    grid: &Vec<Vec<u16>>,
    grid_cols: usize,
    grid_lines: usize,
    start: Point,
    end: Point,
) -> u32 {
    let mut stack: HashMap<Point, u32> = HashMap::new();
    stack.insert(start, 0);

    let mut current_pos = start;
    let mut seen: HashSet<Point> = HashSet::new();

    while !stack.is_empty() {
        let current_risk = *stack.get(&current_pos).unwrap_or(&0);
        for next_pos in get_neighbours(grid_cols, grid_lines, &current_pos) {
            if seen.contains(&next_pos) {
                continue;
            }
            // we where here before or not
            let next_pos_risk = stack.get(&next_pos).unwrap_or(&0);
            // going there again would create this risk
            let next_risk_level = current_risk + get_mapped_value(grid, next_pos) as u32;

            if next_pos_risk == &0 || next_risk_level < current_risk {
                // only go there if never been or this time its less risk
                stack.insert(next_pos, next_risk_level);
            }
        }
        // never again
        seen.insert(current_pos);
        stack.remove(&current_pos);

        // just find the next pos with lowest risk to use next
        let mut min_risk = u32::MAX;
        let mut min_pos = (0, 0);
        for pos in stack.iter() {
            if *pos.1 < min_risk {
                min_risk = *pos.1;
                min_pos = *pos.0;
            }
        }
        current_pos = min_pos;
        if current_pos == end {
            return min_risk;
        }
    }
    u32::MAX
}

fn part1(lines: &[String]) -> u32 {
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

    dijkstra_mod(&grid, grid_cols, grid_lines, start, end)
}

#[allow(dead_code)]
fn print_grid(grid: &Vec<Vec<u16>>, grid_cols: usize, grid_lines: usize) {
    let gl = grid.len();
    let gc = grid.first().unwrap().len();
    for y in 0..grid_lines {
        for x in 0..grid_cols {
            if x != 0 && x % gc == 0 {
                print!("|");
            }
            print!("{}", get_mapped_value(grid, (x as i16, y as i16)))
        }
        if y != 0 && y % gl == 0 {
            println!();
            for _ in 0..grid_cols {
                print!("-");
            }
        }
        println!();
    }
    println!();
}

fn get_mapped_value(grid: &Vec<Vec<u16>>, pos: Point) -> u16 {
    let grid_lines = grid.len();
    let grid_cols = grid.first().unwrap().len();

    let x_mapped = pos.0 as usize % grid_cols;
    let y_mapped = pos.1 as usize % grid_lines;

    let mul_x = pos.0 / grid_cols as i16;
    let mul_y = pos.1 / grid_lines as i16;

    let value = grid[y_mapped][x_mapped];

    let mut mapped_value = value + mul_x as u16 + mul_y as u16;
    if mapped_value > 9 {
        mapped_value = mapped_value - 9;
    }
    mapped_value
}

fn part2(lines: &[String]) -> u32 {
    // 2840
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

    let grid_lines_big = grid_lines * 5;
    let grid_cols_big = grid_cols * 5;

    let start: Point = (0, 0);
    let end: Point = ((grid_cols_big - 1) as i16, (grid_lines_big - 1) as i16);

    dijkstra_mod(&grid, grid_cols_big, grid_lines_big, start, end)
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
        let result = part2(&lines);
        assert_eq!(result, 315);
    }
}
