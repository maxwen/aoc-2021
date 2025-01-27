use aoc_2021::read_lines_as_vec;
use std::cell::RefCell;
use std::collections::HashSet;

#[derive(Debug)]
struct BingoBoard {
    id: usize,
    numbers: Vec<Vec<u16>>,
    marked: Vec<u16>,
}

impl BingoBoard {
    fn add_number(&mut self, number: u16) -> bool {
        self.marked.push(number);
        self.is_winner()
    }

    fn is_winner_lines(&self) -> bool {
        for y in 0..self.numbers.len() {
            let line = self.get_line(y);
            let marked_num = line.iter().filter(|n| self.marked.contains(n)).count();
            if marked_num == 5 {
                return true;
            }
        }
        false
    }

    fn is_winner_columns(&self) -> bool {
        for x in 0..self.numbers[0].len() {
            let col = self.get_column(x);
            let marked_num = col.iter().filter(|n| self.marked.contains(n)).count();
            if marked_num == 5 {
                return true;
            }
        }
        false
    }

    fn is_winner(&self) -> bool {
        self.is_winner_columns() || self.is_winner_lines()
    }

    fn sum_unmarked(&self) -> u16 {
        let mut sum = 0u16;
        for y in 0..self.numbers.len() {
            for x in 0..self.numbers[y].len() {
                let number = self.numbers[y][x];
                if !self.marked.contains(&number) {
                    sum += number
                }
            }
        }
        sum
    }
    fn get_line(&self, y: usize) -> Vec<u16> {
        self.numbers.get(y).unwrap().clone()
    }

    fn get_column(&self, x: usize) -> Vec<u16> {
        let mut col = vec![];
        for y in 0..self.numbers.len() {
            col.push(*self.numbers.get(y).unwrap().get(x).unwrap())
        }
        col
    }
}

fn parse_boards(lines: &[String]) -> Vec<RefCell<BingoBoard>> {
    let mut bingo_boards: Vec<RefCell<BingoBoard>> = vec![];

    let mut i = 2;
    let mut board_id = 0;
    while i < lines.len() {
        let board_numbers = &lines[i..i + 5];
        let mut bingo_board = BingoBoard {
            id: board_id,
            numbers: vec![],
            marked: vec![],
        };
        board_id += 1;
        for board_line in board_numbers.iter() {
            let board_line_numbers = board_line
                .split_ascii_whitespace()
                .map(|n| n.parse().unwrap())
                .collect::<Vec<u16>>();

            bingo_board.numbers.push(board_line_numbers);
        }
        // println!("{:?}", bingo_board);
        bingo_boards.push(RefCell::new(bingo_board));

        i += 6;
    }
    bingo_boards
}
fn part1(lines: &[String]) -> u16 {
    // 25023
    let numbers = lines[0]
        .split(",")
        .map(|n| n.parse().unwrap())
        .collect::<Vec<u16>>();

    let bingo_boards = parse_boards(lines);

    for number in numbers.iter() {
        for board in bingo_boards.iter() {
            let is_winner = board.borrow_mut().add_number(*number);
            if is_winner {
                return number * board.borrow().sum_unmarked();
            }
        }
    }
    0u16
}

fn part2(lines: &[String]) -> u16 {
    // 2634
    let numbers = lines[0]
        .split(",")
        .map(|n| n.parse().unwrap())
        .collect::<Vec<u16>>();

    let bingo_boards = parse_boards(lines);
    let mut boards_not_won = HashSet::new();

    (0..bingo_boards.len()).for_each(|n| { boards_not_won.insert(n); });

    for number in numbers.iter() {
        for board in bingo_boards.iter() {
            let is_winner = board.borrow_mut().add_number(*number);
            if is_winner {
                boards_not_won.remove(&board.borrow().id);

                if boards_not_won.is_empty() {
                    return number * board.borrow().sum_unmarked();
                }
            }
        }
    }
    0u16
}

fn main() {
    let lines = read_lines_as_vec("input/input_day4.txt").unwrap();

    // let lines = vec![
    //     "7,4,9,5,11,17,23,2,0,14,21,19,20,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1",
    //     "",
    //     "22 13 17 11  0",
    //     " 8  2 23  4 24",
    //     "21  9 14 16  7",
    //     " 6 10  3 18  5",
    //     " 1 12 20 15 19",
    //     "",
    //     " 3 15  0  2 22",
    //     " 9 18 13 17  5",
    //     "19  8  7 25 23",
    //     "20 11 10 24  4",
    //     "14 21 16 12  6",
    //     "",
    //     "14 21 17 24  4",
    //     "10 16 15  9 19",
    //     "18  8 23 26 20",
    //     "22 11 13  6  5",
    //     " 2  0 12  3  7",
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
            "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1",
            "",
            "22 13 17 11  0",
            " 8  2 23  4 24",
            "21  9 14 16  7",
            " 6 10  3 18  5",
            " 1 12 20 15 19",
            "",
            " 3 15  0  2 22",
            " 9 18 13 17  5",
            "19  8  7 25 23",
            "20 11 10 24  4",
            "14 21 16 12  6",
            "",
            "14 21 17 24  4",
            "10 16 15  9 19",
            "18  8 23 26 20",
            "22 11 13  6  5",
            " 2  0 12  3  7",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<_>>();
        let result = part1(&lines);
        assert_eq!(result, 4512);
        let result = part2(&lines);
        assert_eq!(result, 1924);
    }
}
