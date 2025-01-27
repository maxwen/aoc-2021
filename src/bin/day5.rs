use aoc_2021::read_lines_as_vec;
use std::cmp::{max, min};
use std::collections::HashMap;

#[derive(Debug)]
struct Segment {
    x_start: u16,
    y_start: u16,
    x_end: u16,
    y_end: u16,
    diagonal: bool,
}

impl Segment {
    #[allow(dead_code)]
    fn get_max_x(&self) -> u16 {
        max(self.x_start, self.x_end)
    }
    #[allow(dead_code)]
    fn get_min_x(&self) -> u16 {
        min(self.x_start, self.x_end)
    }

    #[allow(dead_code)]
    fn get_max_y(&self) -> u16 {
        max(self.y_start, self.y_end)
    }
    #[allow(dead_code)]
    fn get_min_y(&self) -> u16 {
        min(self.y_start, self.y_end)
    }

    fn insert_into_grid(&self, grid: &mut HashMap<(u16, u16), usize>) {
        if !self.diagonal {
            if self.y_start == self.y_end {
                // line
                if self.x_start < self.x_end {
                    for x in self.x_start..self.x_end + 1 {
                        let pos = (x, self.y_start);
                        grid.entry(pos).and_modify(|n| *n += 1).or_insert(1);
                    }
                } else {
                    for x in (self.x_end..self.x_start + 1).rev() {
                        let pos = (x, self.y_start);
                        grid.entry(pos).and_modify(|n| *n += 1).or_insert(1);
                    }
                }
            }
            if self.x_start == self.x_end {
                // col
                if self.y_start < self.y_end {
                    for y in self.y_start..self.y_end + 1 {
                        let pos = (self.x_start, y);
                        grid.entry(pos).and_modify(|n| *n += 1).or_insert(1);
                    }
                } else {
                    for y in (self.y_end..self.y_start + 1).rev() {
                        let pos = (self.x_start, y);
                        grid.entry(pos).and_modify(|n| *n += 1).or_insert(1);
                    }
                }
            }
        } else {
            if self.y_start < self.y_end {
                for y in self.y_start..self.y_end + 1 {
                    let diff = y - self.y_start;
                    if self.x_start < self.x_end {
                        let pos = (self.x_start + diff, y);
                        grid.entry(pos).and_modify(|n| *n += 1).or_insert(1);
                    } else {
                        let pos = (self.x_start - diff, y);
                        grid.entry(pos).and_modify(|n| *n += 1).or_insert(1);
                    }
                }
            } else {
                for y in (self.y_end..self.y_start + 1).rev() {
                    let diff = self.y_start - y;
                    if self.x_start < self.x_end {
                        let pos = (self.x_start + diff, y);
                        grid.entry(pos).and_modify(|n| *n += 1).or_insert(1);
                    } else {
                        let pos = (self.x_start - diff, y);
                        grid.entry(pos).and_modify(|n| *n += 1).or_insert(1);
                    }
                }
            }
        }
    }
}

#[allow(dead_code)]
fn print_grid(grid: &HashMap<(u16, u16), usize>, min_x: u16, min_y: u16, max_x: u16, max_y: u16) {
    for y in min_y..max_y + 1 {
        for x in min_x..max_x + 1 {
            let pos = (x, y);
            if grid.contains_key(&pos) {
                print!("{}", grid.get(&pos).unwrap())
            } else {
                print!(".")
            }
        }
        println!();
    }
    println!()
}

fn count_overlaps(grid: &HashMap<(u16, u16), usize>) -> usize {
    grid.iter().filter(|e| e.1 > &1).count()
}

fn part1(lines: &[String]) -> usize {
    // 7269
    let mut segments = vec![];
    for line in lines.iter() {
        let points = line.split(" -> ").collect::<Vec<_>>();
        let start_coords = points.first().unwrap().split(",").collect::<Vec<_>>();
        let start: (u16, u16) = (
            start_coords.first().unwrap().parse().unwrap(),
            start_coords.last().unwrap().parse().unwrap(),
        );
        let end_coords = points.last().unwrap().split(",").collect::<Vec<_>>();
        let end: (u16, u16) = (
            end_coords.first().unwrap().parse().unwrap(),
            end_coords.last().unwrap().parse().unwrap(),
        );

        if start.0 == end.0 || start.1 == end.1 {
            let s = Segment {
                x_start: start.0,
                y_start: start.1,
                x_end: end.0,
                y_end: end.1,
                diagonal: false,
            };
            segments.push(s);
        }
    }

    // let max_x = segments.iter().map(|s| s.get_max_x()).max().unwrap();
    // let min_x = segments.iter().map(|s| s.get_min_x()).min().unwrap();
    // let max_y = segments.iter().map(|s| s.get_max_y()).max().unwrap();
    // let min_y = segments.iter().map(|s| s.get_min_y()).min().unwrap();

    let mut grid: HashMap<(u16, u16), usize> = HashMap::new();
    for s in segments.iter() {
        s.insert_into_grid(&mut grid);
    }
    // print_grid(&grid, min_x, min_y, max_x, max_y);
    count_overlaps(&grid)
}

fn part2(lines: &[String]) -> usize {
    // 21140
    let mut segments = vec![];
    for line in lines.iter() {
        let points = line.split(" -> ").collect::<Vec<_>>();
        let start_coords = points.first().unwrap().split(",").collect::<Vec<_>>();
        let start: (u16, u16) = (
            start_coords.first().unwrap().parse().unwrap(),
            start_coords.last().unwrap().parse().unwrap(),
        );
        let end_coords = points.last().unwrap().split(",").collect::<Vec<_>>();
        let end: (u16, u16) = (
            end_coords.first().unwrap().parse().unwrap(),
            end_coords.last().unwrap().parse().unwrap(),
        );

        if start.0 == end.0 || start.1 == end.1 {
            let s = Segment {
                x_start: start.0,
                y_start: start.1,
                x_end: end.0,
                y_end: end.1,
                diagonal: false,
            };
            segments.push(s);
        } else {
            let s = Segment {
                x_start: start.0,
                y_start: start.1,
                x_end: end.0,
                y_end: end.1,
                diagonal: true,
            };
            segments.push(s);
        }
    }

    // let max_x = segments.iter().map(|s| s.get_max_x()).max().unwrap();
    // let min_x = segments.iter().map(|s| s.get_min_x()).min().unwrap();
    // let max_y = segments.iter().map(|s| s.get_max_y()).max().unwrap();
    // let min_y = segments.iter().map(|s| s.get_min_y()).min().unwrap();

    let mut grid: HashMap<(u16, u16), usize> = HashMap::new();

    for s in segments.iter() {
        s.insert_into_grid(&mut grid);
    }
    // print_grid(&grid, min_x, min_y, max_x, max_y);
    count_overlaps(&grid)
}

fn main() {
    let lines = read_lines_as_vec("input/input_day5.txt").unwrap();

    // let lines = vec![
    //     "0,9 -> 5,9",
    //     "8,0 -> 0,8",
    //     "9,4 -> 3,4",
    //     "2,2 -> 2,1",
    //     "7,0 -> 7,4",
    //     "6,4 -> 2,0",
    //     "0,9 -> 2,9",
    //     "3,4 -> 1,4",
    //     "0,0 -> 8,8",
    //     "5,5 -> 8,2",
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
            "0,9 -> 5,9",
            "8,0 -> 0,8",
            "9,4 -> 3,4",
            "2,2 -> 2,1",
            "7,0 -> 7,4",
            "6,4 -> 2,0",
            "0,9 -> 2,9",
            "3,4 -> 1,4",
            "0,0 -> 8,8",
            "5,5 -> 8,2",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<_>>();
        let result = part1(&lines);
        assert_eq!(result, 5);
        let result = part2(&lines);
        assert_eq!(result, 12);
    }
}
