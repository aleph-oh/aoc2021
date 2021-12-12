use crate::utils::input;
use std::collections::{HashSet, VecDeque};

type Point = (usize, usize);

fn in_bounds((r, c): Point, n: usize, m: usize) -> bool {
    0 <= r as i64 && r < n && 0 <= c as i64 && c < m
}

fn neighbors((r, c): Point, n: usize, m: usize) -> Vec<Point> {
    let possible = [(r - 1, c), (r + 1, c), (r, c - 1), (r, c + 1)];
    possible
        .iter()
        .copied()
        .filter(|&(rn, cn)| in_bounds((rn, cn), n, m))
        .collect()
}

fn is_min(grid: &[&[u8]], n: usize, m: usize, (r, c): Point) -> bool {
    let v = grid[r][c];
    neighbors((r, c), n, m)
        .iter()
        .all(|&(rn, cn)| v < grid[rn][cn])
}

fn low_points(grid: &[&[u8]]) -> Vec<Point> {
    let n = grid.len();
    let m = if n == 0 { 0 } else { grid[0].len() };
    let mut pts = Vec::new();
    for r in 0..n {
        for c in 0..m {
            if is_min(grid, n, m, (r, c)) {
                pts.push((r, c));
            }
        }
    }
    pts
}

fn part_one(grid: &[&[u8]]) -> i64 {
    low_points(grid)
        .iter()
        .map(|&(r, c)| 1 + grid[r][c] as i64)
        .sum()
}

fn part_two(grid: &[&[u8]]) -> i64 {
    let n = grid.len();
    let m = if n == 0 { 0 } else { grid[0].len() };
    let mut unmarked: HashSet<Point> = (0..n)
        .flat_map(|r| (0..m).map(move |c| (r, c)))
        .filter(|&(r, c)| grid[r][c] != 9)
        .collect();
    // Do a BFS from each low point and build basins until no further points can be added.
    let starts = low_points(grid);
    let mut basins: Vec<HashSet<Point>> = starts
        .iter()
        .map(|pt| Some(*pt).into_iter().collect())
        .collect();
    let mut q = VecDeque::new();
    for (i, &start) in starts.iter().enumerate() {
        q.clear();
        q.extend(
            neighbors(start, n, m)
                .into_iter()
                .filter(|&(r, c)| grid[r][c] != 9),
        );
        while let Some(pt) = q.pop_back() {
            if unmarked.contains(&pt) {
                basins[i].insert(pt);
                q.extend(
                    neighbors(pt, n, m)
                        .into_iter()
                        .filter(|&(r, c)| grid[r][c] != 9),
                );
                unmarked.remove(&pt);
            }
        }
    }
    let basins = basins;
    let mut sizes: Vec<_> = basins.into_iter().map(|basin| (&basin).len()).collect();
    sizes.sort_unstable();
    sizes.reverse();
    sizes[0..3].iter().product::<usize>() as i64
}

pub(crate) fn solve() -> (i64, i64) {
    let grid = input()
        .lines()
        .map(|s| s.bytes().map(|b| b - '0' as u8).collect::<Vec<u8>>())
        .collect::<Vec<_>>();
    let grid_refs: Vec<_> = grid.iter().map(Vec::as_slice).collect();
    (part_one(&grid_refs), part_two(&grid_refs))
}
