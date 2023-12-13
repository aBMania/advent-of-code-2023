use std::iter::zip;
use itertools::Itertools;
use rayon::prelude::*;
use advent_of_code::custom_grid::{CustomGrid, input_to_grid};

advent_of_code::solution!(13);


fn parse_input(input: &str) -> Vec<CustomGrid<char>> {
    input
        .split("\n\n")
        .map(|grid| input_to_grid(grid).unwrap())
        .collect()
}

fn solve_grid_ignore(grid: &mut CustomGrid<char>, ignored_row: Option<usize>, ignored_col: Option<usize>) -> (Option<usize>, Option<usize>) {
    let vertical: Option<usize> = (1..grid.rows()).map(|row| {
        if let Some(ignored_row) = ignored_row {
            if row == ignored_row {
                return 0;
            }
        }
        let symetrical = (0..row).all(|checked_row| {
            {
                let gap = row.checked_sub(checked_row);
                return if let Some(gap) = gap {
                    let checked_against_row = checked_row + 2 * gap - 1;
                    if checked_against_row > grid.rows() - 1 {
                        return true
                    }
                    zip(
                        grid.iter_row(checked_row),
                        grid.iter_row(checked_against_row)
                    ).all(|(c1, c2)| c1.eq(c2))
                } else {
                    false
                }

            }
        });

        if symetrical {
            row
        } else {
            0
        }
    }).find(|&val| val != 0);

    let horizontal: Option<usize> = (1..grid.cols()).map(|col| {
        if let Some(ignored_col) = ignored_col {
            if col == ignored_col {
                return 0;
            }
        }
        let symetrical = (0..col).all(|checked_col| {
            {
                let gap = col.checked_sub(checked_col);
                return if let Some(gap) = gap {
                    let checked_against_col = checked_col + 2 * gap - 1;
                    if checked_against_col > grid.cols() - 1 {
                        return true
                    }

                    zip(
                        grid.iter_col(checked_col),
                        grid.iter_col(checked_against_col)
                    ).all(|(c1, c2)| c1.eq(c2))
                } else {
                    false
                }

            }
        });

        if symetrical {
            col
        } else {
            0
        }
    }).find(|&val| val != 0);

    (horizontal, vertical)
}

fn solve_grid(grid: &mut CustomGrid<char>) -> (Option<usize>, Option<usize>) {
    solve_grid_ignore(grid, None, None)
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        parse_input(input)
            .iter_mut()// parallelize here slows down the solve
            .map(solve_grid)
            .map(|(h,v )| (h.unwrap_or(0) + 100 * v.unwrap_or(0)) as u32)
            .sum()
    )
}

fn solve_part_2(grid: &mut CustomGrid<char>) -> (Option<usize>, Option<usize>) {
    let (h, v) = solve_grid(grid);

    let (smudge_h, smudge_v) = (0..grid.rows()).cartesian_product(0..grid.cols())
        .map(|(row, col)| {
            // Flip
            match grid.get_mut(row, col).unwrap() {
                c if c == &'.' => *c = '#',
                c if c == &'#' => *c = '.',
                _ => panic!()
            }
            let (h, v) = solve_grid_ignore(grid, v, h);

            // Unflip
            match grid.get_mut(row, col).unwrap() {
                c if c == &'.' => *c = '#',
                c if c == &'#' => *c = '.',
                _ => panic!()
            }

            (h, v)
        })
        .map(|(smudge_h, smudge_v)| {
            let smudge_h = match smudge_h.eq(&h) {
                true => None,
                false => smudge_h
            };
            let smudge_v = match smudge_v.eq(&v) {
                true => None,
                false => smudge_v
            };
            (smudge_h, smudge_v)
        })
        .find(|(smudge_h, smudge_v)| smudge_h.is_some() || smudge_v.is_some())
        .unwrap_or((None, None))
        ;

    (smudge_h, smudge_v)
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        parse_input(input)
            .par_iter_mut()
            .map(solve_part_2)
            .map(|(h,v )| (h.unwrap_or(0) + 100 * v.unwrap_or(0)) as u32)
            .sum()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(405));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(400));
    }
}
