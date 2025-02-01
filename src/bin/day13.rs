use aoc_2021::read_lines_as_vec;
use regex::Regex;
use std::collections::HashMap;

fn fold_horizontal(grid: &HashMap<(i16, i16), bool>, y_fold: i16) -> HashMap<(i16, i16), bool> {
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
fn fold_vertical(grid: &HashMap<(i16, i16), bool>, x_fold: i16) -> HashMap<(i16, i16), bool> {
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

fn print_grid(grid: &HashMap<(i16, i16), bool>, width: i16, height: i16) {
    for y in 0..height {
        for x in 0..width {
            if grid.contains_key(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
}
fn part12(lines: &[String], fold_once: bool) -> usize {
    // 765
    // RZKZLPGH
    let mut grid: HashMap<(i16, i16), bool> = HashMap::new();
    let mut height = 0;
    let mut width = 0;
    let mut line_idx = 0;
    for line in lines.iter() {
        line_idx += 1;
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

    let value = Regex::new(r"\d+").unwrap();
    let mut current_width = width;
    let mut current_height = height;
    let mut folded_grid = grid;
    for i in line_idx..lines.len() {
        let line = lines[i].as_str();
        if line.contains("y=") {
            let y: i16 = value.find(line).unwrap().as_str().parse().unwrap();
            folded_grid = fold_horizontal(&folded_grid, y);
            if fold_once {
                return folded_grid.len();
            }
            current_height /= 2;
        }
        if line.contains("x=") {
            let x: i16 = value.find(line).unwrap().as_str().parse().unwrap();
            folded_grid = fold_vertical(&folded_grid, x);
            if fold_once {
                return folded_grid.len();
            }
            current_width /= 2;
        }
    }
    print_grid(&folded_grid, current_width, current_height);

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
    println!("{}", part12(&lines, true));
    part12(&lines, false);
}

#[cfg(test)]
mod tests {
    use crate::part12;

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

        let result = part12(&lines, true);
        assert_eq!(result, 17);
    }
}
