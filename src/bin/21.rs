use std::collections::{BinaryHeap};
use fxhash::{FxHashMap, FxHashSet};
use advent_of_code::custom_grid::CustomGrid;
advent_of_code::solution!(21);

pub fn part_one(input: &str) -> Option<u32> {
    let grid: CustomGrid<char> = input.parse().unwrap();

    let ((start_row, start_col), _) = grid.indexed_iter().find(|(_, &c)| c == 'S').unwrap();

    let mut visited = FxHashSet::default();
    visited.insert((start_row, start_col));

    let mut to_visit = BinaryHeap::from([(start_row, start_col)]);

    for _ in 0..=64 {
        let mut to_visit_next = BinaryHeap::new();
        while let Some((row, col)) = to_visit.pop() {
            for ((neighbor_row, neighbor_col), _) in grid.iter_neighbors(row, col).filter(|(_, &c)| c == '.') {
                if !visited.contains(&(neighbor_row, neighbor_col)) {
                    visited.insert((neighbor_row, neighbor_col));
                    to_visit_next.push((neighbor_row, neighbor_col));
                }
            }
        }
        to_visit = to_visit_next;
    }
    let count = visited.iter().filter(|(row, col)| (row.abs_diff(start_row) + col.abs_diff(start_col)) % 2 == 0).count();
    Some(count as u32)
}

pub fn part_two(input: &str) -> Option<u64> {
    let grid: CustomGrid<char> = input.parse().unwrap();

    let ((start_row, start_col), _) = grid.indexed_iter().find(|(_, &c)| c == 'S').unwrap();

    let mut visited_at = FxHashMap::default();
    let mut visited = FxHashSet::default();
    visited.insert((start_row, start_col));
    let mut to_visit = BinaryHeap::from([(start_row, start_col)]);

    for i in 0..=130 {
        let mut to_visit_next = BinaryHeap::new();
        while let Some((row, col)) = to_visit.pop() {
            if grid.is_border(row, col) {
                // println!("{row}, {col} visited at {i}");
            }
            visited_at.insert((row, col), i);
            for ((neighbor_row, neighbor_col), _) in grid.iter_neighbors(row, col).filter(|(_, &c)| c == '.') {
                if !visited.contains(&(neighbor_row, neighbor_col)) {
                    visited.insert((neighbor_row, neighbor_col));
                    to_visit_next.push((neighbor_row, neighbor_col));
                }
            }
        }

        let mut grid_clone = grid.clone();
        for (row, col) in visited.iter() {
            *grid_clone.get_mut(*row, *col).unwrap() = '0';
        }
        for (row, col) in to_visit.iter() {
            *grid_clone.get_mut(*row, *col).unwrap() = 'X';
        }

        to_visit = to_visit_next;
    }

    let even_squares = visited_at.values().filter(|val| **val % 2 == 0).count() as u64;
    let odd_squares = visited_at.values().filter(|val| **val % 2 == 1).count() as u64;

    let even_corners = visited_at.values().filter(|val| **val % 2 == 0 && **val > 65).count() as u64;
    let odd_corners = visited_at.values().filter(|val| **val % 2 == 1 && **val > 65).count() as u64;

    let n: u64 = (26501365 - 65) / 131;

    Some((n + 1).pow(2) * odd_squares + n.pow(2) * even_squares - (n + 1) * odd_corners + n * even_corners)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(366));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(29834683280163));
    }
}
