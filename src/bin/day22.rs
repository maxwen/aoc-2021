use glam::I64Vec3;
use regex::Regex;
use std::cmp::{max, min};
use std::collections::HashMap;
use std::ops::RangeInclusive;
use aoc_2021::read_lines_as_vec;

fn part1(lines: &[String]) -> usize {
    let reg = Regex::new(r"-?\d+").unwrap();
    let mut core: HashMap<I64Vec3, bool> = HashMap::new();

    for line in lines.iter() {
        let mut action = 0;
        if line.starts_with("on") {
            action = 1;
        }
        let values = reg
            .find_iter(line)
            .map(|v| v.as_str().parse::<i64>().unwrap())
            .collect::<Vec<_>>();
        let x_range = values[0]..=values[1];
        let y_range = values[2]..=values[3];
        let z_range = values[4]..=values[5];

        if x_range.start() < &-50
            || x_range.end() > &50
            || y_range.start() < &-50
            || y_range.end() > &50
            || z_range.start() < &-50
            || z_range.end() > &50
        {
            continue;
        }

        if action == 1 {
            for (_, x) in x_range.clone().enumerate() {
                for (_, y) in y_range.clone().enumerate() {
                    for (_, z) in z_range.clone().enumerate() {
                        let pos = I64Vec3::new(x, y, z);
                        core.insert(pos, true);
                    }
                }
            }
        } else {
            for (_, x) in x_range.clone().enumerate() {
                for (_, y) in y_range.clone().enumerate() {
                    for (_, z) in z_range.clone().enumerate() {
                        let pos = I64Vec3::new(x, y, z);
                        core.remove(&pos);
                    }
                }
            }
        }
    }
    core.len()
}

fn intersection(a: &MyCuboid, b: &MyCuboid) -> Option<MyCuboid> {
    let intersection = MyCuboid::new(
        max(a.min_x, b.min_x),
        min(a.max_x, b.max_x),
        max(a.min_y, b.min_y),
        min(a.max_y, b.max_y),
        max(a.min_z, b.min_z),
        min(a.max_z, b.max_z),
        -b.action,
    );
    if (intersection.min_x > intersection.max_x)
        || (intersection.min_y > intersection.max_y)
        || (intersection.min_z > intersection.max_z)
    {
        None
    } else {
        Some(intersection)
    }
}

#[derive(Debug, Clone)]
struct MyCuboid {
    min_x: i64,
    max_x: i64,
    min_y: i64,
    max_y: i64,
    min_z: i64,
    max_z: i64,
    action: i8,
}

impl MyCuboid {
    fn new_from_range(
        x_range: RangeInclusive<i64>,
        y_range: RangeInclusive<i64>,
        z_range: RangeInclusive<i64>,
        action: i8,
    ) -> Self {
        MyCuboid {
            min_x: *x_range.start(),
            max_x: *x_range.end(),
            min_y: *y_range.start(),
            max_y: *y_range.end(),
            min_z: *z_range.start(),
            max_z: *z_range.end(),
            action,
        }
    }
    fn new(
        min_x: i64,
        max_x: i64,
        min_y: i64,
        max_y: i64,
        min_z: i64,
        max_z: i64,
        action: i8,
    ) -> Self {
        MyCuboid {
            min_x,
            max_x,
            min_y,
            max_y,
            min_z,
            max_z,
            action,
        }
    }
    fn volume(&self) -> i64 {
        let base_volume = (self.max_x - self.min_x + 1)
            * (self.max_y - self.min_y + 1)
            * (self.max_z - self.min_z + 1);

        base_volume * self.action as i64
    }
}

impl PartialEq for MyCuboid {
    fn eq(&self, other: &Self) -> bool {
        self.min_x == other.min_x
            && self.max_x == other.max_x
            && self.min_y == other.min_y
            && self.max_y == other.max_y
            && self.min_z == other.min_z
            && self.max_z == other.max_z
            && self.action == other.action
    }
}

// https://github.com/nilanshu96/Advent-Of-Code/blob/main/2021/22/Part2.java
// https://en.wikipedia.org/wiki/Inclusion%E2%80%93exclusion_principle
fn part2(lines: &[String]) -> i64 {
    let reg = Regex::new(r"-?\d+").unwrap();

    let mut cubes = vec![];

    for line in lines.iter() {
        let mut action = 1;
        if line.starts_with("off") {
            action = -1;
        }
        let values = reg
            .find_iter(line)
            .map(|v| v.as_str().parse::<i64>().unwrap())
            .collect::<Vec<_>>();
        let x_range = values[0]..=values[1];
        let y_range = values[2]..=values[3];
        let z_range = values[4]..=values[5];

        let next_cube = MyCuboid::new_from_range(x_range, y_range, z_range, action);
        if cubes.is_empty() {
            cubes.push(next_cube);
            continue;
        } else {
            let mut new_cubes = cubes.clone();
            for cube in cubes.iter() {
                // intersection will get action of -cube.action
                // so parts that have been marked as off below
                // can be changed to on again in layers above
                if let Some(intersection) = intersection(&next_cube, &cube) {
                    new_cubes.push(intersection);
                }
            }
            if action == 1 {
                new_cubes.push(next_cube);
            }
            cubes.clear();
            cubes.append(&mut new_cubes);
        }
    }
    let v1 = cubes.iter().map(|c| c.volume()).sum();
    v1
}

fn main() {
    let lines = read_lines_as_vec("input/input_day22.txt").unwrap();

    // let lines = vec![
    //     "on x=-20..26,y=-36..17,z=-47..7",
    //     "on x=-20..33,y=-21..23,z=-26..28",
    //     "on x=-22..28,y=-29..23,z=-38..16",
    //     "on x=-46..7,y=-6..46,z=-50..-1",
    //     "on x=-49..1,y=-3..46,z=-24..28",
    //     "on x=2..47,y=-22..22,z=-23..27",
    //     "on x=-27..23,y=-28..26,z=-21..29",
    //     "on x=-39..5,y=-6..47,z=-3..44",
    //     "on x=-30..21,y=-8..43,z=-13..34",
    //     "on x=-22..26,y=-27..20,z=-29..19",
    //     "off x=-48..-32,y=26..41,z=-47..-37",
    //     "on x=-12..35,y=6..50,z=-50..-2",
    //     "off x=-48..-32,y=-32..-16,z=-15..-5",
    //     "on x=-18..26,y=-33..15,z=-7..46",
    //     "off x=-40..-22,y=-38..-28,z=23..41",
    //     "on x=-16..35,y=-41..10,z=-47..6",
    //     "off x=-32..-23,y=11..30,z=-14..3",
    //     "on x=-49..-5,y=-3..45,z=-29..18",
    //     "off x=18..30,y=-20..-8,z=-3..13",
    //     "on x=-41..9,y=-7..43,z=-33..15",
    //     "on x=-54112..-39298,y=-85059..-49293,z=-27449..7877",
    //     "on x=967..23432,y=45373..81175,z=27513..53682",
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
            "on x=-20..26,y=-36..17,z=-47..7",
            "on x=-20..33,y=-21..23,z=-26..28",
            "on x=-22..28,y=-29..23,z=-38..16",
            "on x=-46..7,y=-6..46,z=-50..-1",
            "on x=-49..1,y=-3..46,z=-24..28",
            "on x=2..47,y=-22..22,z=-23..27",
            "on x=-27..23,y=-28..26,z=-21..29",
            "on x=-39..5,y=-6..47,z=-3..44",
            "on x=-30..21,y=-8..43,z=-13..34",
            "on x=-22..26,y=-27..20,z=-29..19",
            "off x=-48..-32,y=26..41,z=-47..-37",
            "on x=-12..35,y=6..50,z=-50..-2",
            "off x=-48..-32,y=-32..-16,z=-15..-5",
            "on x=-18..26,y=-33..15,z=-7..46",
            "off x=-40..-22,y=-38..-28,z=23..41",
            "on x=-16..35,y=-41..10,z=-47..6",
            "off x=-32..-23,y=11..30,z=-14..3",
            "on x=-49..-5,y=-3..45,z=-29..18",
            "off x=18..30,y=-20..-8,z=-3..13",
            "on x=-41..9,y=-7..43,z=-33..15",
            "on x=-54112..-39298,y=-85059..-49293,z=-27449..7877",
            "on x=967..23432,y=45373..81175,z=27513..53682",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<_>>();
        assert_eq!(part1(&lines), 590784);
        assert_eq!(part2(&lines), 39769202357779);
    }
}
