use glam::IVec3;
use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;
use aoc_2021::read_lines_as_vec;

fn part1(lines: &[String]) -> usize {
    let reg = Regex::new(r"-?\d+").unwrap();
    let mut core: HashMap<IVec3, bool> = HashMap::new();

    for line in lines.iter() {
        let mut action = 0;
        if line.starts_with("on") {
            action = 1;
        }
        let values = reg
            .find_iter(line)
            .map(|v| v.as_str().parse::<i32>().unwrap())
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
        // println!("{:?} {:?} {:?}", x_range, y_range, z_range);
        if action == 1 {
            for (_, x) in x_range.clone().enumerate() {
                for (_, y) in y_range.clone().enumerate(){
                    for (_, z) in z_range.clone().enumerate(){
                        let pos = IVec3::new(x, y, z);
                        core.insert(pos, true);
                    }
                }
            }
        } else {
            for (_, x) in x_range.clone().enumerate() {
                for (_, y) in y_range.clone().enumerate(){
                    for (_, z) in z_range.clone().enumerate(){
                        let pos = IVec3::new(x, y, z);
                        core.remove(&pos);
                    }
                }
            }
        }
    }
    core.len()
}

fn part2(lines: &[String]) -> usize {
    0
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
    //     .iter()
    //     .map(|s| s.to_string())
    //     .collect::<Vec<_>>();
    println!("{}", part1(&lines));
    println!("{}", part2(&lines));
}

#[cfg(test)]
mod tests {
    use crate::part1;

    #[test]
    fn it_works() {
        let lines = vec![
            "Player 1 starting position: 4",
            "Player 2 starting position: 8",
        ]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        assert_eq!(part1(&lines), 739785);
        // assert_eq!(part1(&lines, 50), 3351);
    }
}
