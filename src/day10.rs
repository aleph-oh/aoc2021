use crate::day10::DelimiterType::{Angle, Curly, Paren, Square};
use crate::utils::input;
use std::convert::TryFrom;
use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum DelimiterType {
    Paren,
    Square,
    Curly,
    Angle,
}

impl DelimiterType {
    fn syntax_score(&self) -> i64 {
        match self {
            Paren => 3,
            Square => 57,
            Curly => 1197,
            Angle => 25137,
        }
    }

    fn autocomplete_score(&self) -> i64 {
        match self {
            Paren => 1,
            Square => 2,
            Curly => 3,
            Angle => 4,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Delimiter {
    ty: DelimiterType,
    opening: bool,
}

impl Display for Delimiter {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let ch = match (self.ty, self.opening) {
            (Paren, true) => '(',
            (Paren, false) => ')',
            (Square, true) => '[',
            (Square, false) => ']',
            (Curly, true) => '{',
            (Curly, false) => '}',
            (Angle, true) => '<',
            (Angle, false) => '>',
        };
        write!(f, "{}", ch)
    }
}

#[derive(Debug)]
struct ParseDelimiterError;

impl TryFrom<char> for Delimiter {
    type Error = ParseDelimiterError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '(' => Ok(Delimiter {
                ty: Paren,
                opening: true,
            }),
            ')' => Ok(Delimiter {
                ty: Paren,
                opening: false,
            }),
            '[' => Ok(Delimiter {
                ty: Square,
                opening: true,
            }),
            ']' => Ok(Delimiter {
                ty: Square,
                opening: false,
            }),
            '{' => Ok(Delimiter {
                ty: Curly,
                opening: true,
            }),
            '}' => Ok(Delimiter {
                ty: Curly,
                opening: false,
            }),
            '<' => Ok(Delimiter {
                ty: Angle,
                opening: true,
            }),
            '>' => Ok(Delimiter {
                ty: Angle,
                opening: false,
            }),
            _ => Err(ParseDelimiterError),
        }
    }
}

fn to_delimiters(line: &str) -> Vec<Delimiter> {
    line.chars()
        .map(Delimiter::try_from)
        .collect::<Result<Vec<_>, _>>()
        .unwrap()
}

fn part_one(lines: &[&[Delimiter]]) -> i64 {
    let mut stack: Vec<Delimiter> = Vec::new();
    let mut score = 0;
    for &line in lines {
        stack.clear();
        for &delim in line {
            if let Some(&last_added) = stack.last() {
                if last_added.opening && !delim.opening {
                    if last_added.ty != delim.ty {
                        // Mismatch / corruption: last opening
                        score += delim.ty.syntax_score();
                        break;
                    } else {
                        stack.pop();
                    }
                }
            }
            if delim.opening {
                stack.push(delim);
            }
        }
    }
    score
}

fn part_two(lines: &[&[Delimiter]]) -> i64 {
    let mut stack: Vec<Delimiter> = Vec::new();
    let mut scores: Vec<i64> = Vec::new();
    for &line in lines {
        stack.clear();
        let mut broke = false;
        for &delim in line {
            if let Some(&last_added) = stack.last() {
                if last_added.opening && !delim.opening {
                    if last_added.ty != delim.ty {
                        // Mismatch / corruption: last opening doesn't match
                        broke = true;
                        break;
                    } else {
                        stack.pop();
                    }
                }
            }
            if delim.opening {
                stack.push(delim);
            }
        }
        if !broke {
            scores.push(
                stack
                    .iter()
                    .rev()
                    .map(|delim| delim.ty.autocomplete_score())
                    .fold(0, |acc, score| 5 * acc + score),
            );
        }
    }
    assert_eq!(
        scores.len() % 2,
        1,
        "Must have an odd number of scores but got {}, which is even",
        scores.len()
    );
    scores.sort_unstable();
    scores[scores.len() / 2]
}

pub(crate) fn solve() -> (i64, i64) {
    let input = input();
    let delimiters: Vec<_> = input.lines().map(to_delimiters).collect();
    let delimiter_refs: Vec<_> = delimiters.iter().map(Vec::as_slice).collect();
    (part_one(&delimiter_refs), part_two(&delimiter_refs))
}
