#![allow(unused_mut)]

use advent_of_code::custom_grid::{input_to_grid, CustomGrid};
use itertools::Itertools;
use std::collections::HashMap;
use memoize::memoize;
advent_of_code::solution!(14);

#[memoize]
fn cycle(mut grid: CustomGrid<char>) -> CustomGrid<char> {
    tilt_up(&mut grid);
    tilt_left(&mut grid);
    tilt_down(&mut grid);
    tilt_right(&mut grid);
    grid
}

fn tilt_up(grid: &mut CustomGrid<char>) {
    (0..grid.rows()).cartesian_product(0..grid.cols()).fold(
        HashMap::with_capacity(grid.cols()),
        |mut topmost_obstacles, (row, col)| {
            let topmost_obstacle = topmost_obstacles.entry(col).or_insert(0);
            let c = grid.get_mut(row, col).unwrap();

            match c {
                '#' => {
                    *topmost_obstacle = row + 1;
                }
                'O' => {
                    *c = '.';
                    *grid.get_mut(*topmost_obstacle, col).unwrap() = 'O';
                    *topmost_obstacle += 1;
                }
                _ => {}
            }

            topmost_obstacles
        },
    );
}

fn tilt_down(grid: &mut CustomGrid<char>) {
    (0..grid.rows()).rev().cartesian_product(0..grid.cols()).fold(
        HashMap::with_capacity(grid.cols()),
        |mut bottommost_obstacles, (row, col)| {
            let bottommost_obstacle = bottommost_obstacles.entry(col).or_insert(grid.rows() - 1);
            let c = grid.get_mut(row, col).unwrap();

            match c {
                '#' => {
                    if row != 0 {
                        *bottommost_obstacle = row - 1;
                    }
                }
                'O' => {
                    *c = '.';
                    *grid.get_mut(*bottommost_obstacle, col).unwrap() = 'O';
                    if *bottommost_obstacle != 0 {
                        *bottommost_obstacle -= 1;
                    }
                }
                _ => {}
            }

            bottommost_obstacles
        },
    );
}

fn tilt_left(grid: &mut CustomGrid<char>) {
    (0..grid.cols()).cartesian_product(0..grid.rows()).fold(
        HashMap::with_capacity(grid.rows()),
        |mut rightmost_obstacles, (col, row)| {
            let rightmost_obstacle = rightmost_obstacles.entry(row).or_insert(0);
            let c = grid.get_mut(row, col).unwrap();

            match c {
                '#' => {
                    *rightmost_obstacle = col + 1;
                }
                'O' => {
                    *c = '.';
                    *grid.get_mut(row, *rightmost_obstacle).unwrap() = 'O';
                    *rightmost_obstacle += 1;
                }
                _ => {}
            }

            rightmost_obstacles
        },
    );
}

fn tilt_right(grid: &mut CustomGrid<char>) {
    (0..grid.cols()).rev().cartesian_product(0..grid.rows()).fold(
        HashMap::with_capacity(grid.rows()),
        |mut rightmost_obstacles, (col, row)| {
            let rightmost_obstacle = rightmost_obstacles.entry(row).or_insert(grid.cols() - 1);
            let c = grid.get_mut(row, col).unwrap();

            match c {
                '#' => {
                    if col != 0 {
                        *rightmost_obstacle = col - 1;
                    }
                }
                'O' => {
                    *c = '.';
                    *grid.get_mut(row, *rightmost_obstacle).unwrap() = 'O';
                    if *rightmost_obstacle != 0 {
                        *rightmost_obstacle -= 1;
                    }
                }
                _ => {}
            }

            rightmost_obstacles
        },
    );
}

fn damages(grid: &CustomGrid<char>) -> u32 {
    grid.indexed_iter()
        .map(|((row, _), c)| match c {
            'O' => (grid.rows() - row) as u32,
            _ => 0,
        })
        .sum()
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut grid: CustomGrid<char> = input_to_grid(input).unwrap();

    tilt_up(&mut grid);

    Some(damages(&grid))
}

pub fn part_two(input: &str) -> Option<u32> {
    let input = input.to_string().leak();
    let mut grid: CustomGrid<char> = input_to_grid(input).unwrap();


    let mut memo = HashMap::new();
    let mut solves = HashMap::new();
    let mut i = 0usize;

    let mut bound: Option<usize> = None;

    let solve = loop {
        i += 1;

        grid = cycle(grid);

        solves.insert(i, damages(&grid));

        let v: String = grid.iter().collect();
        let memo_entry = memo.entry(v).or_insert(vec![]);

        if bound.is_none() {
            if let Some(previous_i) = memo_entry.pop() {
                let cycle = i - previous_i;
                let offset = previous_i;
                let mut tmp = 1_000_000_000 % (i - previous_i);
                while tmp < offset {
                    tmp += cycle
                }
                bound = Some(tmp);
            }
        }

        if let Some(bound) = bound {
            if let Some(&solve) = solves.get(&bound) {
                break solve;
            }
        }

        memo_entry.push(i);
    };

    Some(solve)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(136));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(64));
    }
}
