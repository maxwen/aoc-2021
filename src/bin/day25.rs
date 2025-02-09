use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use aoc_2021::read_lines_as_vec;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum CucumberType {
    East,
    South,
}
impl TryFrom<char> for CucumberType {
    type Error = ();

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '>' => Ok(CucumberType::East),
            'v' => Ok(CucumberType::South),
            _ => Err(()),
        }
    }
}

impl Display for CucumberType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CucumberType::East => write!(f, ">"),
            CucumberType::South => write!(f, "v"),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Cucumber {
    cucumber_type: CucumberType,
    pos: (usize, usize),
}

impl Cucumber {
    fn new(cucumber_type: CucumberType, pos: (usize, usize)) -> Self {
        Cucumber { cucumber_type, pos }
    }

    fn move_one(
        &mut self,
        map: &HashMap<(usize, usize), CucumberType>,
        grid_cols: usize,
        grid_lines: usize,
    ) {
        let next_pos = match self.cucumber_type {
            CucumberType::East => ((self.pos.0 + 1) % grid_cols, self.pos.1),
            CucumberType::South => (self.pos.0, (self.pos.1 + 1) % grid_lines),
        };
        if !map.contains_key(&next_pos) {
            self.pos = next_pos
        }
    }
}

fn update_map(
    map: &mut HashMap<(usize, usize), CucumberType>,
    east_cucumbers: &Vec<Cucumber>,
    south_cucumber: &Vec<Cucumber>,
) {
    map.clear();
    east_cucumbers.iter().for_each(|c| {
        map.insert(c.pos, c.cucumber_type);
    });
    south_cucumber.iter().for_each(|c| {
        map.insert(c.pos, c.cucumber_type);
    });
}

#[allow(dead_code)]
fn print_map(map: &HashMap<(usize, usize), CucumberType>, grid_cols: usize, grid_lines: usize) {
    for y in 0..grid_lines {
        for x in 0..grid_cols {
            let pos = (x, y);
            if map.contains_key(&pos) {
                print!("{}", map[&pos]);
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
}

fn part1(lines: &[String]) -> usize {
    // 486
    let mut east_cucumbers = vec![];
    let mut south_cucumber = vec![];

    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let pos = (x, y);
            let cucumber_type = CucumberType::try_from(c);
            match cucumber_type {
                Ok(cucumber_type) => match cucumber_type {
                    CucumberType::East => east_cucumbers.push(Cucumber::new(cucumber_type, pos)),
                    CucumberType::South => south_cucumber.push(Cucumber::new(cucumber_type, pos)),
                },
                Err(_) => {}
            }
        }
    }

    let grid_lines = lines.len();
    let grid_cols = lines.get(0).unwrap().len();
    let mut map = HashMap::new();
    update_map(&mut map, &east_cucumbers, &south_cucumber);
    // print_map(&map, grid_cols, grid_lines);

    let mut last_map = map.clone();

    let mut steps = 1;

    loop {
        east_cucumbers
            .iter_mut()
            .for_each(|c| c.move_one(&map, grid_cols, grid_lines));
        update_map(&mut map, &east_cucumbers, &south_cucumber);
        // print_map(&map, grid_cols, grid_lines);

        south_cucumber
            .iter_mut()
            .for_each(|c| c.move_one(&map, grid_cols, grid_lines));

        update_map(&mut map, &east_cucumbers, &south_cucumber);
        // print_map(&map, grid_cols, grid_lines);

        if map == last_map {
            break;
        }
        last_map = map.clone();
        steps += 1;
    }
    // print_map(&map, grid_cols, grid_lines);
    // println!("{}", steps);

    steps
}

fn main() {
    let lines = read_lines_as_vec("input/input_day25.txt").unwrap();

    // let lines = vec![
    //     "v...>>.vv>",
    //     ".vv>>.vv..",
    //     ">>.>v>...v",
    //     ">>v>>.>.v.",
    //     "v>v.vv.v..",
    //     ">.>>..v...",
    //     ".vv..>.>v.",
    //     "v.v..>>v.v",
    //     "....v..v.>",
    // ]
    // .iter()
    // .map(|s| s.to_string())
    // .collect::<Vec<_>>();
    println!("{}", part1(&lines));
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    fn it_works() {
        let lines = vec![
            "v...>>.vv>",
            ".vv>>.vv..",
            ">>.>v>...v",
            ">>v>>.>.v.",
            "v>v.vv.v..",
            ">.>>..v...",
            ".vv..>.>v.",
            "v.v..>>v.v",
            "....v..v.>",
        ]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        assert_eq!(part1(&lines), 58);
    }
}
