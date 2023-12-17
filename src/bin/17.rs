use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

use advent_of_code::custom_grid::{CustomGrid, input_to_grid};

use crate::Direction::{Down, Left, Right, Up};

advent_of_code::solution!(17);

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Pos {
    row: usize,
    col: usize,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct SearchCase {
    direction: Direction,
    position: Pos,
    n_steps: u8,
}

struct SmallestCostHolder {
    search_case: SearchCase,
    cost: u32,
}


impl PartialEq for SmallestCostHolder {
    fn eq(&self, other: &Self) -> bool {
        self.cost.eq(&other.cost)
    }
}

impl Eq for SmallestCostHolder {}

impl PartialOrd for SmallestCostHolder {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for SmallestCostHolder {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

fn solve(grid: CustomGrid<u8>, min_steps: u8, max_steps: u8) -> u32 {
    let mut to_see = BinaryHeap::new();
    let mut bests: HashMap<SearchCase, u32> = HashMap::new();


    to_see.push(SmallestCostHolder {
        search_case: SearchCase {
            position: Pos { row: 0, col: 0 },
            direction: Right,
            n_steps: 0,
        },
        cost: 0,
    });

    to_see.push(SmallestCostHolder {
        search_case: SearchCase {
            position: Pos { row: 0, col: 0 },
            direction: Down,
            n_steps: 0,
        },
        cost: 0,
    });


    while let Some(SmallestCostHolder { cost, search_case, .. }) = to_see.pop() {
        let current_best = bests.get(&search_case).unwrap_or(&u32::MAX);
        if *current_best <= cost {
            continue;
        }

        bests.insert(search_case, cost);

        if search_case.position.row == grid.rows() - 1 && search_case.position.col == grid.cols() - 1 && search_case.n_steps + 1 >= min_steps {
            return cost;
        }

        if search_case.n_steps + 1 < max_steps {
            let next = match search_case.direction {
                Down => grid.down_indexed(search_case.position.row, search_case.position.col),
                Up => grid.up_indexed(search_case.position.row, search_case.position.col),
                Right => grid.right_indexed(search_case.position.row, search_case.position.col),
                Left => grid.left_indexed(search_case.position.row, search_case.position.col),
            };
            if let Some(((row, col), &heat)) = next {
                to_see.push(SmallestCostHolder {
                    search_case: SearchCase {
                        direction: search_case.direction,
                        n_steps: search_case.n_steps + 1,
                        position: Pos {row, col}
                    },
                    cost: cost + heat as u32,
                })
            }
        }

        let next_directions = match search_case.direction {
            Up | Down => [Right, Left],
            Right | Left => [Up, Down],
        };

        if search_case.n_steps + 1 < min_steps {
            continue;
        }

        for next_direction in next_directions
        {
            let next = match next_direction {
                Down => grid.down_indexed(search_case.position.row, search_case.position.col),
                Up => grid.up_indexed(search_case.position.row, search_case.position.col),
                Right => grid.right_indexed(search_case.position.row, search_case.position.col),
                Left => grid.left_indexed(search_case.position.row, search_case.position.col),
            };

            if let Some(((row, col), &heat)) = next {
                to_see.push(SmallestCostHolder {
                    search_case: SearchCase {
                        direction: next_direction,
                        n_steps: 0,
                        position: Pos {row, col}
                    },
                    cost: cost + heat as u32,
                })
            }
        }
    }

    0
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid: CustomGrid<u8> = input_to_grid(input).unwrap();

    Some(solve(grid, 1, 3))
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid: CustomGrid<u8> = input_to_grid(input).unwrap();

    Some(solve(grid, 4, 10))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(102));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(94));
    }
}
