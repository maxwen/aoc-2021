use aoc_2021::read_lines_as_vec;

#[allow(dead_code)]
fn print_grid(grid: &Vec<Vec<u8>>) {
    for y in 0..grid.len() {
        for x in 0..grid.first().unwrap().len() {
            let value = grid[y][x];
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

fn get_neighbours_value(grid: &Vec<Vec<u8>>, p: (usize, usize), default_value: u8) -> u16 {
    let pos = (p.0 as i32, p.1 as i32);
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
        let mut value = default_value;
        if x >= 0 && x < grid.first().unwrap().len() as i32 && y >= 0 && y < grid.len() as i32 {
            value = grid[y as usize][x as usize];
        }
        value_str += value.to_string().as_str();
    }
    u16::from_str_radix(value_str.as_str(), 2).unwrap()
}

fn add_padding(grid: &Vec<Vec<u8>>, padding_value: u8) -> Vec<Vec<u8>> {
    let mut enhanced_grid: Vec<Vec<u8>> = vec![];

    for y in 0..grid.len() + 2 {
        let mut l = vec![];
        for x in 0..grid.first().unwrap().len() + 2 {
            l.push(padding_value);
        }
        enhanced_grid.push(l);
    }

    for y in 0..grid.len() {
        for x in 0..grid.first().unwrap().len() {
            enhanced_grid[y + 1][x + 1] = grid[y][x]
        }
    }
    enhanced_grid
}
fn part1(lines: &[String], steps: usize) -> usize {
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

    let mut grid: Vec<Vec<u8>> = vec![];
    for y in 2..lines.len() {
        let line = lines.get(y).unwrap();
        let mut l = vec![];
        for c in line.chars() {
            match c {
                '#' => l.push(1),
                '.' => l.push(0),
                _ => panic!("wrong input"),
            }
        }
        grid.push(l);
    }

    // print_grid(&grid);

    let blink = algo_values[0] == 1;

    for step in 1..=steps {
        // the currently NOT visible part of the infinite input
        // changes too so we MUST not add constant 0 padding
        // but depending on algorithm what the invisible dark pixel
        // would change too. This means its alternating between
        // 0 and 1 to use for next padding round
        let padding_value = if (step % 2 == 0) {1} else {0};
        grid = add_padding(&grid, padding_value);
        // print_grid(&grid);

        let mut enhanced_grid: Vec<Vec<u8>> = vec![];

        for y in 0..grid.len() {
            let mut l = vec![];
            for x in 0..grid.first().unwrap().len() {
                let pos = (x, y);
                let value = get_neighbours_value(&grid, pos, padding_value);
                let mapped_value = algo_values[value as usize];
                l.push(mapped_value);
            }
            enhanced_grid.push(l);
        }
        // print_grid(&enhanced_grid);

        grid = enhanced_grid;
    }

    let mut sum = 0;
    for y in 0..grid.len() {
        for x in 0..grid.first().unwrap().len() {
            if grid[y][x] == 1 {
                sum += 1;
            }
        }
    }
    sum
}

fn part2(lines: &[String]) -> usize {
    0
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
        assert_eq!(part1(&lines), 35);
        // assert_eq!(part2(&lines), 112);
    }
}
