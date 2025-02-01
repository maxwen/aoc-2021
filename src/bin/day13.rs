use aoc_2021::read_lines_as_vec;
use regex::Regex;
use std::collections::HashMap;

fn fold_horizontal(
    grid: &HashMap<(i16, i16), bool>,
    y_fold: i16,
    height: i16,
) -> HashMap<(i16, i16), bool> {
    let mut folded_grid: HashMap<(i16, i16), bool> = HashMap::new();
    for (x, y) in grid.keys() {
        if *y > y_fold {
            folded_grid.insert((*x, y_fold - (y - y_fold)), true);
        } else {
            folded_grid.insert((*x, *y), true);
        }
    }
    folded_grid
}
fn fold_vertical(
    grid: &HashMap<(i16, i16), bool>,
    x_fold: i16,
    width: i16,
) -> HashMap<(i16, i16), bool> {
    let mut folded_grid: HashMap<(i16, i16), bool> = HashMap::new();
    for (x, y) in grid.keys() {
        if *x > x_fold {
            folded_grid.insert((x_fold - (x - x_fold), *y), true);
        } else {
            folded_grid.insert((*x, *y), true);
        }
    }
    folded_grid
}

fn part1(lines: &[String]) -> usize {
    // 765
    let mut grid: HashMap<(i16, i16), bool> = HashMap::new();
    let mut height = 0;
    let mut width = 0;
    for line in lines.iter() {
        if line.len() == 0 {
            break;
        }
        let coords = line.split(",").collect::<Vec<_>>();
        let x: i16 = coords.first().unwrap().parse().unwrap();
        let y: i16 = coords.last().unwrap().parse().unwrap();
        grid.insert((x, y), true);

        if x > width {
            width = x;
        }
        if y > height {
            height = y;
        }
    }
    height += 1;
    width += 1;

    lines.iter().next();

    let value = Regex::new(r"\d+").unwrap();
    for line in lines.iter() {
        if line.contains("y=") {
            let y: i16 = value.find(line).unwrap().as_str().parse().unwrap();
            let folded_grid = fold_horizontal(&grid, y, height);
            return folded_grid.len();
        }
        if line.contains("x=") {
            let x: i16 = value.find(line).unwrap().as_str().parse().unwrap();
            let folded_grid = fold_vertical(&grid, x, width);
            return folded_grid.len();
        }
    }
    0usize
}

fn part2(lines: &[String]) -> usize {
    0usize
}

fn main() {
    let lines = read_lines_as_vec("input/input_day13.txt").unwrap();

    // let lines = vec![
    //     "6,10",
    //     "0,14",
    //     "9,10",
    //     "0,3",
    //     "10,4",
    //     "4,11",
    //     "6,0",
    //     "6,12",
    //     "4,1",
    //     "0,13",
    //     "10,12",
    //     "3,4",
    //     "3,0",
    //     "8,4",
    //     "1,10",
    //     "2,14",
    //     "8,10",
    //     "9,0",
    //     "",
    //     "fold along y=7",
    //     "fold along x=5",
    // ]
    // .iter()
    // .map(|s| s.to_string())
    // .collect::<Vec<_>>();
    println!("{}", part1(&lines));
    println!("{}", part2(&lines));
}

#[cfg(test)]
mod tests {
    use crate::part1;

    #[test]
    fn it_works() {
        let lines = vec![
            "6,10",
            "0,14",
            "9,10",
            "0,3",
            "10,4",
            "4,11",
            "6,0",
            "6,12",
            "4,1",
            "0,13",
            "10,12",
            "3,4",
            "3,0",
            "8,4",
            "1,10",
            "2,14",
            "8,10",
            "9,0",
            "",
            "fold along y=7",
            "fold along x=5",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<_>>();

        let result = part1(&lines);
        assert_eq!(result, 17);
        // let result = part2(&lines);
        // assert_eq!(result, 36);
    }
}
