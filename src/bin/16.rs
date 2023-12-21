use itertools::Itertools;
use bit_set::BitSet;
use rayon::prelude::*;
use advent_of_code::custom_grid::{CustomGrid, input_to_grid};
use crate::Direction::{Left, Right, Up, Down};
advent_of_code::solution!(16);

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct PosDir {
    position: (usize, usize),
    direction: Direction,
}

impl Into<usize> for PosDir {
    fn into(self) -> usize {
        (self.position.0 << 16) + (self.position.1 << 2) + self.direction as usize
    }
}

fn solve(grid: &CustomGrid<char>, start: PosDir) -> u32 {
    let mut energized_beams: BitSet = BitSet::new();

    let mut next_stack: Vec<_> = vec![start];

    while let Some(current) = next_stack.pop() {
        if energized_beams.contains(current.into()) {
            continue;
        }

        let (row, col) = current.position;
        let direction = current.direction;

        match grid.get(row, col) {
            None => continue,
            Some(c) => {
                energized_beams.insert(current.into());
                let mut next_directions = vec![];
                match (c, direction) {
                    ('.', _) => next_directions.push(direction),
                    ('/', Right) => next_directions.push(Up),
                    ('/', Up) => next_directions.push(Right),
                    ('/', Left) => next_directions.push(Down),
                    ('/', Down) => next_directions.push(Left),
                    ('\\', Left) => next_directions.push(Up),
                    ('\\', Down) => next_directions.push(Right),
                    ('\\', Right) => next_directions.push(Down),
                    ('\\', Up) => next_directions.push(Left),
                    ('|', Up | Down) => next_directions.push(direction),
                    ('-', Left | Right) => next_directions.push(direction),
                    ('|', Left | Right) => {
                        next_directions.push(Up);
                        next_directions.push(Down);
                    }
                    ('-', Up | Down) => {
                        next_directions.push(Left);
                        next_directions.push(Right);
                    }
                    _ => unimplemented!()
                };

                for next_direction in next_directions.iter() {
                    match next_direction {
                        Up => {
                            if row != 0 {
                                next_stack.push(PosDir {
                                    position: (row - 1, col),
                                    direction: *next_direction,
                                });
                            }
                        }
                        Down => {
                            if row != grid.rows() - 1 {
                                next_stack.push(PosDir {
                                    position: (row + 1, col),
                                    direction: *next_direction,
                                });
                            }
                        }
                        Right => {
                            if col != grid.cols() - 1 {
                                next_stack.push(PosDir {
                                    position: (row, col + 1),
                                    direction: *next_direction,
                                });
                            }
                        }
                        Left => {
                            if col != 0 {
                                next_stack.push(PosDir {
                                    position: (row, col - 1),
                                    direction: *next_direction,
                                });
                            }
                        }
                    }
                }
            }
        }
    }

    energized_beams.iter().unique_by(|pos| pos >> 2).count() as u32
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid: CustomGrid<char> = input_to_grid(input).unwrap();

    Some(solve(&grid, PosDir{
        direction: Right,
        position: (0, 0)
    }))
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid: CustomGrid<char> = input_to_grid(input).unwrap();

    let mut possible_starts = vec![];

    for i in 0..grid.rows() {
        possible_starts.push(PosDir {
            position: (i, 0),
            direction: Right
        });
        possible_starts.push(PosDir {
            position: (i, grid.cols() - 1),
            direction: Left
        });
    }

    for i in 0..grid.cols() {
        possible_starts.push(PosDir {
            position: (0, i),
            direction: Down
        });
        possible_starts.push(PosDir {
            position: (grid.rows() - 1, i),
            direction: Up
        });
    }

    Some(
        possible_starts
            .into_par_iter()
            .map(|start| solve(&grid, start))
            .max()
            .unwrap()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(51));
    }
}
