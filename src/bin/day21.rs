use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;
use aoc_2021::read_lines_as_vec;

fn get_next_dice(round: usize, dice_size: usize) -> Vec<usize> {
    let mut dice_values = vec![];
    for i in [round * 3 + 1, round * 3 + 2, round * 3 + 3].iter() {
        let dice_value = i % dice_size;
        dice_values.push(dice_value)
    }
    dice_values
}
fn part1(lines: &[String]) -> usize {
    // 1002474
    let mut player1_pos = 0;
    let mut player2_pos = 0;
    let digits = Regex::new(r"\d+").unwrap();

    player1_pos = digits
        .find_iter(lines[0].as_str())
        .nth(1)
        .unwrap()
        .as_str()
        .parse()
        .unwrap();
    player2_pos = digits
        .find_iter(lines[1].as_str())
        .nth(1)
        .unwrap()
        .as_str()
        .parse()
        .unwrap();

    let mut player1_score = 0;
    let mut player2_score = 0;

    let mut dice_rolls = 0;
    let mut i = 0;
    loop {
        let player1_dice: usize = get_next_dice(i, 100).iter().sum();
        player1_pos = ((player1_pos + player1_dice - 1) % 10) + 1;

        player1_score += player1_pos;
        dice_rolls += 3;

        if player1_score >= 1000 {
            return player2_score * dice_rolls;
        }

        let player2_dice: usize = get_next_dice(i + 1, 100).iter().sum();
        player2_pos = ((player2_pos + player2_dice - 1) % 10) + 1;

        player2_score += player2_pos;
        dice_rolls += 3;

        i += 2;

        if player2_score >= 1000 {
            return player1_score * dice_rolls;
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct Player {
    position: u8,
    points: u64,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct Game {
    player1: Player,
    player2: Player,
}

fn move_player(
    games: &HashMap<Game, u64>,
    frequencies_map: &HashMap<u8, u8>,
    player1: bool,
) -> (HashMap<Game, u64>, u64) {
    let mut next_games: HashMap<Game, u64> = HashMap::new();
    let mut victories = 0;

    for (game, count) in games.iter() {
        let mut player = if player1 {
            &game.player1
        } else {
            &game.player2
        };

        for (roll, frequency) in frequencies_map.iter() {
            let pos = ((player.position + roll - 1) % 10) + 1;
            let points = player.points + pos as u64;
            let new_player = Player {
                position: pos,
                points,
            };
            let new_state = if player1 {
                Game {
                    player1: new_player,
                    player2: game.player2.clone(),
                }
            } else {
                Game {
                    player1: game.player1.clone(),
                    player2: new_player,
                }
            };
            let new_count = *count * *frequency as u64;

            if new_state.player1.points >= 21 || new_state.player2.points >= 21 {
                victories += new_count;
            } else {
                next_games
                    .entry(new_state)
                    .and_modify(|c| *c += new_count)
                    .or_insert(new_count);
            }
        }
    }
    (next_games, victories)
}

fn part2(lines: &[String]) -> u64 {
    // 919758187195363
    let digits = Regex::new(r"\d+").unwrap();

    let player1_pos_start: u8 = digits
        .find_iter(lines[0].as_str())
        .nth(1)
        .unwrap()
        .as_str()
        .parse()
        .unwrap();
    let player2_pos_start: u8 = digits
        .find_iter(lines[1].as_str())
        .nth(1)
        .unwrap()
        .as_str()
        .parse()
        .unwrap();

    // https://www.ericburden.work/blog/2021/12/31/advent-of-code-2021-day-21/

    let player1 = Player {
        position: player1_pos_start,
        points: 0,
    };

    let player2 = Player {
        position: player2_pos_start,
        points: 0,
    };

    let mut frequencies_map: HashMap<u8, u8> = HashMap::new();
    frequencies_map.insert(3, 1);
    frequencies_map.insert(4, 3);
    frequencies_map.insert(5, 6);
    frequencies_map.insert(6, 7);
    frequencies_map.insert(7, 6);
    frequencies_map.insert(8, 3);
    frequencies_map.insert(9, 1);

    let game = Game { player1, player2 };

    let mut games: HashMap<Game, u64> = HashMap::new();
    games.insert(game, 1);

    let mut win_map = HashMap::new();
    win_map.insert("1", 0u64);
    win_map.insert("2", 0u64);

    let mut player1_turn = true;
    while !games.is_empty() {
        let mut victories = 0;
        (games, victories) = move_player(&games, &frequencies_map, player1_turn);
        if player1_turn {
            win_map.entry("1").and_modify(|v| *v += victories);
        } else {
            win_map.entry("2").and_modify(|v| *v += victories);
        }
        player1_turn = !player1_turn;
    }

    *win_map.values().max().unwrap()
}

fn main() {
    let lines = read_lines_as_vec("input/input_day21.txt").unwrap();
    // let lines = vec![
    //     "Player 1 starting position: 4",
    //     "Player 2 starting position: 8",
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
            "Player 1 starting position: 4",
            "Player 2 starting position: 8",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<_>>();
        assert_eq!(part1(&lines), 739785);
        assert_eq!(part1(&lines), 444356092776315);
    }
}
