use std::collections::HashMap;
use std::ops::Range;
use aoc_2021::read_lines_as_vec;

#[derive(Debug)]
struct Segment {
    x_range: Range<u16>,
    y_range: Range<u16>,
}

impl Segment {
    fn get_max_x(&self) -> u16 {
        if self.x_range.start > self.x_range.end - 1 {
            return self.x_range.start
        }
        self.x_range.end - 1
    }

    fn get_max_y(&self) -> u16 {
        if self.y_range.start > self.y_range.end - 1 {
            return self.y_range.start
        }
        self.y_range.end - 1
    }

    fn get_min_x(&self) -> u16 {
        if self.x_range.start > self.x_range.end - 1 {
            return self.x_range.end - 1
        }
        self.x_range.start
    }

    fn get_min_y(&self) -> u16 {
        if self.y_range.start > self.y_range.end - 1 {
            return self.y_range.end - 1
        }
        self.y_range.start
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

fn count_overlaps(grid: &HashMap<(u16, u16), usize>) -> usize{
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

        if  start.0 == end.0 || start.1 == end.1 {
            let s = Segment {
                x_range: if start.0 < end.0 {
                    start.0..end.0 + 1
                } else {
                    end.0..start.0 + 1
                },
                y_range: if start.1 < end.1 {
                    start.1..end.1 + 1
                } else {
                    end.1..start.1 + 1
                },
            };
            segments.push(s);
        }
    }

    let max_x = segments.iter().map(|s| s.get_max_x()).max().unwrap();
    let min_x = segments.iter().map(|s| s.get_min_x()).min().unwrap();
    let max_y = segments.iter().map(|s| s.get_max_y()).max().unwrap();
    let min_y = segments.iter().map(|s| s.get_min_y()).min().unwrap();

    let mut grid: HashMap<(u16, u16), usize> = HashMap::new();

    for s in segments.iter() {
        for y in s.y_range.clone() {
            for x in s.x_range.clone() {
                let pos = (x, y);
                grid.entry(pos).and_modify(|n| *n += 1).or_insert(1);
            }
        }
    }
    // print_grid(&grid, min_x, min_y, max_x, max_y);
    count_overlaps(&grid)
}

fn part2(lines: &[String]) -> u16 {
    0u16
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
    use super::*;

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
        // let result = part2(&lines);
        // assert_eq!(result, 1924);
    }
}
