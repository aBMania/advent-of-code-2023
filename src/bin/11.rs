use advent_of_code::custom_grid::{input_to_grid, CustomGrid};
use itertools::Itertools;
use std::collections::HashMap;

advent_of_code::solution!(11);

fn parse_input(input: &str, gap: usize) -> Vec<(usize, usize)> {
    let grid: CustomGrid<char> = input_to_grid(input).unwrap();

    // For each row and cols, how many empty row/col is there ahead of it.
    // It is to know how much it should shift any galaxy at this row/col
    let mut empty_rows: HashMap<usize, usize> = HashMap::new();
    let mut empty_cols: HashMap<usize, usize> = HashMap::new();

    let mut current_empty_rows = 0;

    for (i, mut row) in grid.iter_rows().enumerate() {
        if row.all(|&n| n == '.') {
            current_empty_rows += 1;
        }

        empty_rows.insert(i, current_empty_rows);
    }

    let mut current_empty_cols = 0;

    for (i, mut col) in grid.iter_cols().enumerate() {
        if !col.contains(&'#') {
            current_empty_cols += 1;
        }

        empty_cols.insert(i, current_empty_cols);
    }

    grid.indexed_iter()
        .filter_map(|((row, col), c)| match c {
            '#' => Some((
                row + (gap - 1) * empty_rows.get(&row).expect("no empty row entry in map"),
                col + (gap - 1) * empty_cols.get(&col).expect("no empty col entry in map"),
            )),
            _ => None,
        })
        .collect()
}

fn distance((row, col): (usize, usize), (other_row, other_col): (usize, usize)) -> usize {
    row.abs_diff(other_row) + col.abs_diff(other_col)
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        parse_input(input, 2)
            .into_iter()
            .combinations(2)
            .map(|chunk| {
                let (row, col) = chunk.first().unwrap();
                let (other_row, other_col) = chunk.get(1).unwrap();

                distance((*row, *col), (*other_row, *other_col)) as u32
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let positions = parse_input(input, 1_000_000);
    Some(
        positions
            .iter()
            .combinations(2)
            .map(|chunk| {
                let (row, col) = chunk.first().unwrap();
                let (other_row, other_col) = chunk.get(1).unwrap();

                distance((*row, *col), (*other_row, *other_col)) as u64
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance() {
        assert_eq!(distance((6, 1), (11, 5)), 9);
        assert_eq!(distance((0, 0), (0, 0)), 0);
        assert_eq!(distance((0, 0), (0, 1)), 1);
        assert_eq!(distance((0, 0), (1, 1)), 2);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(82000210));
    }
}
