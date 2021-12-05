use crate::utils::input;
use std::collections::HashMap;
use std::convert::{identity, TryInto};

const ROW_SIZE: usize = 5;

#[derive(Clone)]
struct Board {
    cells: [u64; ROW_SIZE * ROW_SIZE],
    marked: [bool; ROW_SIZE * ROW_SIZE],
    value_to_cell: HashMap<u64, usize>,
    // indices into cells corresponding to all values present in the board
}

impl Board {
    fn from_str_rows(rows: &[&str]) -> Self {
        assert_eq!(rows.len(), 5);
        let cells: Vec<u64> = rows
            .iter()
            .flat_map(|r| {
                r.split_whitespace()
                    .map(|s| s.parse::<u64>().expect("Failed to parse a cell as u64"))
            })
            .collect();
        let value_to_cell = cells.iter().enumerate().map(|(i, &n)| (n, i)).collect();
        let marked = [false; ROW_SIZE * ROW_SIZE];
        Self {
            cells: cells.try_into().expect("Cells had wrong length"),
            value_to_cell,
            marked,
        }
    }

    fn is_win(&self) -> bool {
        // Check for bingo in a row
        if (0..ROW_SIZE).any(|r| {
            self.marked
                .iter()
                .copied()
                .skip(r * ROW_SIZE) // Skip the first r rows
                .take(ROW_SIZE) // Row-major order: go over the row
                .all(identity)
        }) {
            return true;
        }
        // Check for bingo in a column
        if (0..ROW_SIZE).any(|c| {
            self.marked
                .iter()
                .copied()
                .skip(c) // Skip the first max(c - 1, 0) columns
                .step_by(ROW_SIZE) // Next cell is ROW_SIZE down
                .take(ROW_SIZE)
                .all(identity)
        }) {
            return true;
        }
        false
    }

    fn draw_number(&mut self, n: u64) {
        if let Some(&i) = self.value_to_cell.get(&n) {
            self.marked[i] = true;
        }
    }

    fn score(&self, last_n: u64) -> i64 {
        last_n as i64
            * self
                .cells
                .iter()
                .zip(self.marked.iter())
                .fold(
                    0i64,
                    |acc, (&v, marked)| {
                        if !marked {
                            acc + v as i64
                        } else {
                            acc
                        }
                    },
                )
    }
}

fn parse_input(lines: &[&str]) -> (Vec<u64>, Vec<Board>) {
    let numbers: Vec<u64> = lines
        .get(0)
        .expect("Should have at least 1 line")
        .split(',')
        .map(|s| s.parse().expect("Failed to parse first row as all numbers"))
        .collect();
    // Skip the next line: it's blank as per the spec
    let boards: Vec<Board> = lines[1..]
        .chunks(6)
        .filter(|chunk| chunk.len() != 1)
        .map(|rows| Board::from_str_rows(&rows[1..]))
        .collect();
    (numbers, boards)
}

fn part_one(numbers: &[u64], mut boards: Vec<Board>) -> i64 {
    let mut i = 0;
    while !boards.iter().any(Board::is_win) {
        boards.iter_mut().for_each(|b| b.draw_number(numbers[i]));
        i += 1;
    }
    // We must have won on at least one board, otherwise we wouldn't have exited the loop..
    let last_i = i - 1;
    let first_winning_board = boards.iter().find(|&b| b.is_win()).unwrap();
    first_winning_board.score(numbers[last_i])
}

fn part_two(numbers: &[u64], mut boards: Vec<Board>) -> i64 {
    let mut i = 0;
    while !boards.iter().all(Board::is_win) {
        boards.retain(|b| !b.is_win()); // Reduce only to boards that haven't been won
        boards.iter_mut().for_each(|b| b.draw_number(numbers[i]));
        i += 1;
    }
    assert_eq!(boards.len(), 1);
    let last_i = i - 1;
    let last_winning_board = &boards[0];
    last_winning_board.score(numbers[last_i])
}

pub(crate) fn solve() -> (i64, i64) {
    let input = input();
    let lines: Vec<&str> = input.lines().collect();
    let (numbers, boards) = parse_input(&lines);
    (
        part_one(&numbers, boards.clone()),
        part_two(&numbers, boards),
    )
}
