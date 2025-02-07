use aoc_2021::read_lines_as_vec;
use std::cmp::{max, min};
use std::collections::HashMap;

#[allow(dead_code)]
fn print_grid(grid: &HashMap<(i32, i32), u8>, min_x: i32, max_x: i32, min_y: i32, max_y: i32) {
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let pos = (x, y);
            let value = grid.get(&pos).unwrap();
            match value {
                0 => print!("."),
                1 => print!("#"),
                _ => {}
            }
        }
        println!();
    }
    println!();
}

fn get_neighbours_value(
    grid: &HashMap<(i32, i32), u8>,
    pos: (i32, i32),
    default_value: u8,
) -> u16 {
    let mut value_str = "".to_string();
    for (x, y) in [
        (pos.0 - 1, pos.1 - 1),
        (pos.0, pos.1 - 1),
        (pos.0 + 1, pos.1 - 1),
        (pos.0 - 1, pos.1),
        (pos.0, pos.1),
        (pos.0 + 1, pos.1),
        (pos.0 - 1, pos.1 + 1),
        (pos.0, pos.1 + 1),
        (pos.0 + 1, pos.1 + 1),
    ]
    .iter()
    .map(|(r, c)| (*r, *c))
    {
        let value = *grid.get(&(x, y)).unwrap_or(&default_value);
        value_str += value.to_string().as_str();
    }
    u16::from_str_radix(value_str.as_str(), 2).unwrap()
}

fn part1(lines: &[String], steps: usize) -> usize {
    // 5203
    // 18806
    let algo = lines.first().unwrap().trim();
    let algo_values = algo
        .chars()
        .filter(|c| *c != '\n')
        .map(|c| match c {
            '#' => 1,
            '.' => 0,
            _ => panic!("wrong input"),
        })
        .collect::<Vec<u8>>();

    let mut grid: HashMap<(i32, i32), u8> = HashMap::new();

    let mut min_x = i32::MAX;
    let mut max_x = 0;
    let mut min_y = i32::MAX;
    let mut max_y = 0;

    for y in 2..lines.len() {
        let line = lines.get(y).unwrap();
        for (x, c) in line.chars().enumerate() {
            let pos = (x as i32, y as i32);
            match c {
                '#' => grid.insert(pos, 1),
                '.' => grid.insert(pos, 0),
                _ => panic!("wrong input"),
            };
            min_x = min(min_x, x as i32);
            min_y = min(min_y, y as i32);
            max_x = max(max_x, x as i32);
            max_y = max(max_y, y as i32);
        }
    }

    let init_enhance_padding_value = algo_values[0];
    for step in 1..=steps {
        // https://github.com/PhenixFine/advent-of-code-kotlin-2021/blob/main/src/Day20.kt
        // the currently NOT visible part of the infinite input
        // changes too so we MUST not add constant 0 padding
        // but depending on algorithm what the invisible dark pixel
        // would change too. This means its alternating between
        // 0 and 1 to use for next padding round
        // steps 1 - invisible pixel are 0
        // enhance -> algo_values[0] == 1 so all that was 0 will become 1
        // step 2 - invisible pixel are 1
        // enhance -> algo_values[1] == 0 so all that was 1 will become 0
        // step 3 - invisible pixel are 0

        let padding_value = if init_enhance_padding_value == 0 {
            0
        } else if step % 2 == 0 {
            1
        } else {
            0
        };

        let mut enhanced_grid: HashMap<(i32, i32), u8> = HashMap::new();

        min_x -= 1;
        min_y -= 1;
        max_x += 1;
        max_y += 1;

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let pos = (x, y);
                let value = get_neighbours_value(&grid, pos, padding_value);
                let mapped_value = algo_values[value as usize];
                enhanced_grid.insert(pos, mapped_value);
            }
        }

        grid = enhanced_grid;
    }

    grid.values().filter(|v| *v == &1).count()
}

fn main() {
    let lines = read_lines_as_vec("input/input_day20.txt").unwrap();
    //     let lines = vec![
    //         "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..##
    // #..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###
    // .######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#.
    // .#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#.....
    // .#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#..
    // ...####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.....
    // ..##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#",
    //         "",
    //         "#..#.",
    //         "#....",
    //         "##..#",
    //         "..#..",
    //         "..###",
    //     ]
    //     .iter()
    //     .map(|s| s.to_string())
    //     .collect::<Vec<_>>();
    println!("{}", part1(&lines, 2));
    println!("{}", part1(&lines, 50));
}

#[cfg(test)]
mod tests {
    use crate::part1;

    #[test]
    fn it_works() {
        let lines = vec![
            "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..##
#..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###
.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#.
.#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#.....
.#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#..
...####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.....
..##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#",
            "",
            "#..#.",
            "#....",
            "##..#",
            "..#..",
            "..###",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<_>>();
        assert_eq!(part1(&lines, 2), 35);
        assert_eq!(part1(&lines, 50), 3351);
    }
}
