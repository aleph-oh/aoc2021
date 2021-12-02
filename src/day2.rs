use crate::utils::{input, parse_input};
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Copy, Clone)]
enum Direction {
    Forward,
    Down,
    Up,
}

#[derive(Debug)]
struct ParseDirectionError;

impl Display for ParseDirectionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to parse direction")
    }
}

impl Error for ParseDirectionError {}

impl FromStr for Direction {
    type Err = ParseDirectionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "forward" => Ok(Self::Forward),
            "down" => Ok(Self::Down),
            "up" => Ok(Self::Up),
            _ => Err(ParseDirectionError),
        }
    }
}

#[derive(Copy, Clone)]
struct Move {
    dir: Direction,
    length: u64,
}

#[derive(Debug)]
struct ParseMoveError;

impl Display for ParseMoveError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to parse move")
    }
}

impl Error for ParseMoveError {}

impl FromStr for Move {
    type Err = ParseMoveError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split_ascii_whitespace().collect();
        if parts.len() != 2 {
            return Err(ParseMoveError);
        }
        Ok(Move {
            dir: parts[0].parse().map_err(|_| ParseMoveError)?,
            length: parts[1].parse().map_err(|_| ParseMoveError)?,
        })
    }
}

impl Move {
    fn dx(&self) -> i64 {
        (match self.dir {
            Direction::Forward => self.length,
            _ => 0,
        }) as i64
    }

    fn dy(&self) -> i64 {
        match self.dir {
            Direction::Up => -(self.length as i64),
            Direction::Down => self.length as i64,
            _ => 0,
        }
    }
}

struct Position {
    x: i64,
    y: i64,
}

struct AimPosition {
    x: i64,
    y: i64,
    aim: i64,
}

impl AimPosition {
    fn new() -> AimPosition {
        AimPosition { x: 0, y: 0, aim: 0 }
    }
}

impl From<AimPosition> for Position {
    fn from(ap: AimPosition) -> Self {
        Self { x: ap.x, y: ap.y }
    }
}

fn make_moves_no_aim(moves: &[Move]) -> Position {
    let x = moves.iter().map(Move::dx).sum();
    let y = moves.iter().map(Move::dy).sum();
    Position { x, y }
}

fn make_moves_aim(moves: &[Move]) -> Position {
    let aim_pos = moves
        .iter()
        .fold(AimPosition::new(), |acc, mv| match mv.dir {
            Direction::Forward => AimPosition {
                x: acc.x + mv.dx(),
                y: acc.y + acc.aim * mv.length as i64,
                aim: acc.aim,
            },
            _ => AimPosition {
                x: acc.x,
                y: acc.y,
                aim: acc.aim + mv.dy(),
            },
        });
    Position::from(aim_pos)
}

pub(crate) fn solve() -> (i64, i64) {
    let v = parse_input(&input());
    let pos = make_moves_no_aim(&v);
    let aim_pos = make_moves_aim(&v);
    (pos.x * pos.y, aim_pos.x * aim_pos.y)
}
