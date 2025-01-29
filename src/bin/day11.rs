use aoc_2021::read_lines_as_vec;

type Point = (usize, usize);

fn get_neighbours(grid: &Vec<Vec<u32>>, pos: &Point) -> Vec<Point> {
    let grid_lines = grid.len();
    let grid_cols = grid.first().unwrap().len();

    let n_pos = (pos.0 as i32, pos.1 as i32);
    [
        (n_pos.0, n_pos.1 + 1),
        (n_pos.0, n_pos.1 - 1),
        (n_pos.0 - 1, n_pos.1),
        (n_pos.0 + 1, n_pos.1),
        (n_pos.0 + 1, n_pos.1 + 1),
        (n_pos.0 - 1, n_pos.1 + 1),
        (n_pos.0 + 1, n_pos.1 - 1),
        (n_pos.0 - 1, n_pos.1 - 1),
    ]
    .iter()
    .filter(|pos| pos.0 >= 0 && pos.0 < grid_cols as i32 && pos.1 >= 0 && pos.1 < grid_lines as i32)
    .map(|(r, c)| (*r as usize, *c as usize))
    .collect::<Vec<_>>()
}

#[allow(dead_code)]
fn print_map(grid: &Vec<Vec<u32>>) {
    let grid_lines = grid.len();
    let grid_cols = grid.first().unwrap().len();

    for y in 0..grid_lines {
        for x in 0..grid_cols {
            let pos = (x, y);
            let energy_level = grid[pos.1][pos.0];
            print!("{}", energy_level);
        }
        println!();
    }
    println!();
}

fn flash_o_bang(
    grid: &mut Vec<Vec<u32>>,
    flash_list: &Vec<Point>,
    all_flash_list: &mut Vec<Point>,
    flashes: &mut usize,
) {
    let mut new_flash_list = vec![];
    for pos in flash_list.iter() {
        let neighbours = get_neighbours(&grid, pos);
        for n in neighbours.iter() {
            let energy_level = grid[n.1][n.0];
            if energy_level == 9 {
                // flashes this round - all flashes will be set to 0 later
                new_flash_list.push(*n);
                all_flash_list.push(*n);
            }
            grid[n.1][n.0] = energy_level + 1;
        }
    }
    *flashes += new_flash_list.len();
    if new_flash_list.len() != 0 {
        flash_o_bang(grid, &new_flash_list, all_flash_list, flashes);
    }
}

fn part1(lines: &[String]) -> usize {
    // 1713
    let mut grid: Vec<Vec<u32>> = vec![];

    for line in lines.iter() {
        let mut l = vec![];
        for c in line.chars() {
            let energy_level = c.to_digit(10).unwrap();
            l.push(energy_level);
        }
        grid.push(l);
    }

    let grid_lines = grid.len();
    let grid_cols = grid.first().unwrap().len();

    // print_map(&grid);

    let steps = 100;
    let mut flashes = 0;

    for _ in 0..steps {
        let mut flash_list = vec![];
        let mut all_flash_list = vec![];

        // 1: -> increase all by 1
        for y in 0..grid_lines {
            for x in 0..grid_cols {
                let pos = (x, y);
                let energy_level = grid[pos.1][pos.0];

                if energy_level == 9 {
                    // flashes this round - all flashes will be set to 0 later
                    flash_list.push((x, y));
                    all_flash_list.push((x, y));
                }
                grid[pos.1][pos.0] = energy_level + 1;
            }
        }
        flashes += flash_list.len();

        // 2: -> recursivly increase all neighbours
        flash_o_bang(&mut grid, &flash_list, &mut all_flash_list, &mut flashes);

        // 3: -> now set all that flashed this round to 0
        all_flash_list.iter().for_each(|pos| {
            grid[pos.1][pos.0] = 0;
        });
    }
    // print_map(&grid);

    flashes
}

fn part2(lines: &[String]) -> u32 {
    let mut grid: Vec<Vec<u32>> = vec![];

    for line in lines.iter() {
        let mut l = vec![];
        for c in line.chars() {
            let energy_level = c.to_digit(10).unwrap();
            l.push(energy_level);
        }
        grid.push(l);
    }

    let grid_lines = grid.len();
    let grid_cols = grid.first().unwrap().len();

    let mut step = 1;
    let mut flashes = 0;

    loop {
        let mut flash_list = vec![];
        let mut all_flash_list = vec![];

        // 1: -> increase all by 1
        for y in 0..grid_lines {
            for x in 0..grid_cols {
                let pos = (x, y);
                let energy_level = grid[pos.1][pos.0];

                if energy_level == 9 {
                    // flashes this round - all flashes will be set to 0 later
                    flash_list.push((x, y));
                    all_flash_list.push((x, y));
                }
                grid[pos.1][pos.0] = energy_level + 1;
            }
        }
        flashes += flash_list.len();

        // 2: -> recursivly increase all neighbours
        flash_o_bang(&mut grid, &flash_list, &mut all_flash_list, &mut flashes);

        // 3: -> now set all that flashed this round to 0
        all_flash_list.iter().for_each(|pos| {
            grid[pos.1][pos.0] = 0;
        });

        if all_flash_list.len() == grid_lines * grid_cols {
            return step;
        }
        step += 1;
    }
}

fn main() {
    let lines = read_lines_as_vec("input/input_day11.txt").unwrap();

    // let lines = vec![
    //     "5483143223",
    //     "2745854711",
    //     "5264556173",
    //     "6141336146",
    //     "6357385478",
    //     "4167524645",
    //     "2176841721",
    //     "6882881134",
    //     "4846848554",
    //     "5283751526",
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
            "5483143223",
            "2745854711",
            "5264556173",
            "6141336146",
            "6357385478",
            "4167524645",
            "2176841721",
            "6882881134",
            "4846848554",
            "5283751526",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<_>>();

        let result = part1(&lines);
        assert_eq!(result, 1656);
        let result = part2(&lines);
        assert_eq!(result, 195);
    }
}
