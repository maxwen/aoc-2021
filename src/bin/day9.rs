use aoc_2021::read_lines_as_vec;
use std::collections::HashSet;

fn get_neighbours(grid: &Vec<Vec<u32>>, pos: (i32, i32)) -> Vec<(i32, i32)> {
    let grid_lines = grid.len() as i32;
    let grid_cols = grid.first().unwrap().len() as i32;

    [
        (pos.0, pos.1 + 1),
        (pos.0, pos.1 - 1),
        (pos.0 - 1, pos.1),
        (pos.0 + 1, pos.1),
    ]
    .iter()
    .filter(|pos| pos.0 >= 0 && pos.0 < grid_cols && pos.1 >= 0 && pos.1 < grid_lines)
    .map(|(r, c)| (*r, *c))
    .collect::<Vec<_>>()
}

fn get_basin_neighbours(grid: &Vec<Vec<u32>>, pos: (i32, i32)) -> Vec<(i32, i32)> {
    let grid_lines = grid.len() as i32;
    let grid_cols = grid.first().unwrap().len() as i32;

    [
        (pos.0, pos.1 + 1),
        (pos.0, pos.1 - 1),
        (pos.0 - 1, pos.1),
        (pos.0 + 1, pos.1),
    ]
    .iter()
    .filter(|pos| {
        pos.0 >= 0
            && pos.0 < grid_cols
            && pos.1 >= 0
            && pos.1 < grid_lines
            && grid[pos.1 as usize][pos.0 as usize] != 9
    })
    .map(|(r, c)| (*r, *c))
    .collect::<Vec<_>>()
}

fn collect_basin_neighbours(
    grid: &Vec<Vec<u32>>,
    pos: (i32, i32),
    basin_list: &mut HashSet<(i32, i32)>,
) {
    basin_list.insert(pos);
    let neighbours = get_basin_neighbours(&grid, pos);
    for n in neighbours {
        if !basin_list.contains(&n) {
            collect_basin_neighbours(grid, n, basin_list);
        }
    }
}

fn part1(lines: &[String]) -> u32 {
    // 462
    let mut grid: Vec<Vec<u32>> = vec![];

    for (_, line) in lines.iter().enumerate() {
        let mut l = vec![];
        for (_, c) in line.chars().enumerate() {
            let height = c.to_digit(10).unwrap();
            l.push(height);
        }
        grid.push(l);
    }

    let grid_lines = grid.len() as i32;
    let grid_cols = grid.first().unwrap().len() as i32;

    let mut low_points = vec![];
    let mut sum = 0;
    for y in 0..grid_lines {
        for x in 0..grid_cols {
            let height = grid[y as usize][x as usize];
            let neighbours = get_neighbours(&grid, (x, y));
            let mut low_point = true;
            for n in neighbours {
                if grid[n.1 as usize][n.0 as usize] <= height {
                    low_point = false;
                    break;
                }
            }
            if low_point {
                low_points.push((x, y));
                sum += 1 + height
            }
        }
    }
    sum
}

fn part2(lines: &[String]) -> u32 {
    // 1397760
    let mut grid: Vec<Vec<u32>> = vec![];

    for (_, line) in lines.iter().enumerate() {
        let mut l = vec![];
        for (_, c) in line.chars().enumerate() {
            let height = c.to_digit(10).unwrap();
            l.push(height);
        }
        grid.push(l);
    }

    let grid_lines = grid.len() as i32;
    let grid_cols = grid.first().unwrap().len() as i32;

    let mut low_points = vec![];
    for y in 0..grid_lines {
        for x in 0..grid_cols {
            let height = grid[y as usize][x as usize];
            let neighbours = get_neighbours(&grid, (x, y));
            let mut low_point = true;
            for n in neighbours {
                if grid[n.1 as usize][n.0 as usize] <= height {
                    low_point = false;
                    break;
                }
            }
            if low_point {
                low_points.push((x, y));
            }
        }
    }

    let mut nigh_score_basins = vec![];
    for l in low_points.iter() {
        let mut basin_list = HashSet::new();
        collect_basin_neighbours(&grid, *l, &mut basin_list);
        nigh_score_basins.push(basin_list.len() as u32);
    }
    nigh_score_basins.sort();
    nigh_score_basins
        .get(nigh_score_basins.len() - 3..)
        .unwrap()
        .iter()
        .product::<u32>()
}

fn main() {
    let lines = read_lines_as_vec("input/input_day9.txt").unwrap();

    // let lines = vec![
    //     "2199943210",
    //     "3987894921",
    //     "9856789892",
    //     "8767896789",
    //     "9899965678",
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
            "2199943210",
            "3987894921",
            "9856789892",
            "8767896789",
            "9899965678",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<_>>();

        let result = part1(&lines);
        assert_eq!(result, 15);
        let result = part2(&lines);
        assert_eq!(result, 1134);
    }
}
