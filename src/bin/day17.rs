use std::collections::HashSet;
use regex::Regex;
use aoc_2021::read_lines_as_vec;

fn test_speed(
    x_speed: i32,
    y_speed: i32,
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
) -> (bool, i32) {
    let mut x = 0;
    let mut y = 0;
    let mut x_speed = x_speed;
    let mut y_speed = y_speed;
    let mut y_max = y;

    loop {
        x += x_speed;
        y += y_speed;
        x_speed = if x_speed > 0 {
            x_speed - 1
        } else if x_speed < 0 {
            x_speed + 1
        } else {
            x_speed
        };
        y_speed -= 1;

        if y > y_max {
            y_max = y
        }

        if x >= min_x && x <= max_x && y <= max_y && y >= min_y {
            return (true, y_max);
        }
        if x > max_x || y < min_y {
            return (false, y_max);
        }
    }
}

// how far do we get in t steps when we start with speed_x_start
fn get_x_in_t(t: i32, speed_x_start: i32) -> i32 {
    let mut x = speed_x_start;

    for t_now in (1..t).rev() {
        x += speed_x_start - t_now;
    }
    x
}

// collect all possible start x_speed values that can
// get us inside min_x - max_y in t steps
fn get_x_speed_list(min_x: i32, max_x: i32) -> HashSet<i32> {
    let mut x_speed_list = HashSet::new();
    for x_speed in (0..=max_x).rev() {
        // we can assume it will never take longer then max_x
        for t in 1..=max_x {
            let x = get_x_in_t(t, x_speed);
            if x >= min_x && x <= max_x {
                x_speed_list.insert(x_speed);
            }
        }
    }
    x_speed_list
}

fn part1(line: &String) -> ((i32, i32), i32) {
    // 4851
    let reg = Regex::new(r"-?\d+").unwrap();
    let values = reg
        .find_iter(line)
        .map(|v| v.as_str().parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let min_x = values[0];
    let max_x = values[1];
    let min_y = values[2];
    let max_y = values[3];

    let x_speed_list = get_x_speed_list(min_x, max_x);

    let mut height_max_all = 0;
    let mut height_max_speed = (0, 0);

    for x_speed in x_speed_list {
        let mut y_speed = 1;
        loop {
            let (hit, height_max) = test_speed(x_speed, y_speed, min_x, max_x, min_y, max_y);
            if hit {
                if height_max > height_max_all {
                    height_max_all = height_max;
                    height_max_speed = (x_speed, y_speed);
                }
            }
            y_speed += 1;

            // just assumption
            if y_speed > min_y.abs() + 1 {
                break;
            }
        }
    }

    (height_max_speed, height_max_all)
}

fn part2(line: &String) -> usize {
    // 1739
    let reg = Regex::new(r"-?\d+").unwrap();
    let values = reg
        .find_iter(line)
        .map(|v| v.as_str().parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let min_x = values[0];
    let max_x = values[1];
    let min_y = values[2];
    let max_y = values[3];

    // from part1 we know the speed_y for max height so we can assume
    // we dont need to test anything out of that range
    let max_y_speed = part1(line).0.1 + 1;

    let x_speed_list = get_x_speed_list(min_x, max_x);
    let mut hit_list = HashSet::new();

    for x_speed in x_speed_list {
        for y_speed in -max_y_speed..=max_y_speed {
            let (hit, _) = test_speed(x_speed, y_speed, min_x, max_x, min_y, max_y);
            if hit {
                hit_list.insert((x_speed, y_speed));
            }
        }
    }
    hit_list.len()
}

fn main() {
    let lines = read_lines_as_vec("input/input_day17.txt").unwrap();
    // let lines = vec!["target area: x=20..30, y=-10..-5"]
    //     .iter()
    //     .map(|s| s.to_string())
    //     .collect::<Vec<_>>();
    println!("{}", part1(&lines[0]).1);
    println!("{}", part2(&lines[0]));
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    fn it_works() {
        let lines = vec!["target area: x=20..30, y=-10..-5"]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        assert_eq!(part1(&lines[0]).1, 45);
        assert_eq!(part2(&lines[0]), 112);

    }
}
