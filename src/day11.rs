use crate::utils::input;
use std::collections::VecDeque;
use std::mem;

#[repr(packed)]
#[derive(Copy, Clone)]
struct Octopus {
    energy: u8,
    flashed: bool,
}

#[derive(Clone)]
struct Board {
    grid: Vec<Octopus>,
}

impl Board {
    fn neighbors(&self, index: usize) -> Vec<usize> {
        let (r, c) = self.to_r_c(index);
        (-1isize..=1)
            .flat_map(|r| (-1isize..=1).map(move |c| (r, c)))
            .filter(|&(r, c)| r != 0 || c != 0)
            .map(|(dr, dc)| (r as isize + dr, c as isize + dc))
            .filter(|&point| self.in_bounds(point))
            .map(|(r, c)| self.to_index((r as usize, c as usize)))
            .collect()
    }

    fn in_bounds(&self, (r, c): (isize, isize)) -> bool {
        let n = (self.grid.len() as f64).sqrt() as usize;
        0 <= r && r < n as isize && 0 <= c && c < n as isize
    }

    fn to_r_c(&self, index: usize) -> (usize, usize) {
        let n = (self.grid.len() as f64).sqrt() as usize;
        (index / n, index % n)
    }

    fn to_index(&self, (r, c): (usize, usize)) -> usize {
        let n = (self.grid.len() as f64).sqrt() as usize;
        r * n + c
    }

    /// Step the board, mutating existing state and returning the number of flashes.
    fn step_mut(&mut self) -> u64 {
        assert!(
            self.grid.iter().all(|o| !o.flashed),
            "No octopuses should have flashed at start of turn."
        );
        // First, increment the energy level of all octopuses.
        self.grid.iter_mut().for_each(|mut o| o.energy += 1);
        // Next, for any octopus with energy level greater than 9, flash.
        let mut queue: VecDeque<_> = self
            .grid
            .iter()
            .enumerate()
            .filter(|(_i, o)| o.energy > 9)
            .map(|(i, _o)| i)
            .collect();
        queue.iter().for_each(|&i| self.grid[i].flashed = true);
        // All indices are implicitly marked as being able to flash.
        // While there are still octopuses who can flash, flash and see if any flashed octopuses
        // can also flash (if so, enqueue them).
        while let Some(i) = queue.pop_back() {
            let neighbors = self.neighbors(i);
            for j in neighbors {
                let mut other_o = &mut self.grid[j];
                other_o.energy += 1;
                if !other_o.flashed && other_o.energy > 9 {
                    other_o.flashed = true;
                    queue.push_back(j);
                }
            }
        }
        // We're done when no more octopuses can flash.
        // Finally, set energy of any flashing octopus to zero and return the number of flashes.
        let flashed = self.grid.iter_mut().fold(0, |acc, mut o| {
            if o.flashed {
                o.energy = 0;
                o.flashed = false;
                acc + 1
            } else {
                acc
            }
        });
        flashed
    }
}

fn part_one(board: &mut Board, steps: u16) -> i64 {
    (0..steps).map(|_| board.step_mut()).sum::<u64>() as i64
}

fn part_two(board: &mut Board) -> i64 {
    let count = board.grid.len() as u64;
    (1..).take_while(|_| board.step_mut() != count).count() as i64 + 1
}

pub(crate) fn solve() -> (i64, i64) {
    let inputs = input();
    let cells: Vec<u8> = inputs
        .lines()
        .flat_map(|line| line.chars().map(|ch| ch.to_digit(10).unwrap() as u8))
        .collect();
    let mut board = Board {
        grid: cells
            .into_iter()
            .map(|n| Octopus {
                energy: n,
                flashed: false,
            })
            .collect(),
    };
    let mut copy = board.clone();
    (part_one(&mut board, 100), part_two(&mut copy))
}
