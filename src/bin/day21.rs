use regex::Regex;
use aoc_2021::read_lines_as_vec;

fn get_next_dice(round: usize) -> Vec<usize> {
    let mut dice_values = vec![];
    for i in [round * 3 + 1, round * 3 + 2, round * 3 + 3].iter() {
        let dice_value = i % 100;
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
        let player1_dice: usize = get_next_dice(i).iter().sum();
        player1_pos = ((player1_pos + player1_dice - 1) % 10) + 1;

        player1_score += player1_pos;
        dice_rolls += 3;

        if player1_score >= 1000 {
            return player2_score * dice_rolls;
        }

        let player2_dice: usize = get_next_dice(i + 1).iter().sum();
        player2_pos = ((player2_pos + player2_dice - 1) % 10) + 1;

        player2_score += player2_pos;
        dice_rolls += 3;

        i += 2;

        if player2_score >= 1000 {
            return player1_score * dice_rolls;
        }
    }
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
