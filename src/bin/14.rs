use std::collections::HashMap;
use itertools::Itertools;
use advent_of_code::custom_grid::{CustomGrid, input_to_grid};
advent_of_code::solution!(14);

fn tilt_up(grid: &mut CustomGrid<char>) {
    let topmost_obstacles: HashMap<usize, usize> = HashMap::with_capacity(grid.cols());

    (0..grid.rows()).cartesian_product(0..grid.cols())
        .fold(topmost_obstacles, |mut topmost_obstacles, (row, col)| {
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
        });
}

fn damages(grid: &CustomGrid<char>) -> u32 {
    grid.indexed_iter().map(|((row, _), c)| {
        match c {
            'O' => (grid.rows() - row) as u32,
            _ => 0
        }
    }).sum()
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid: CustomGrid<char> = input_to_grid(input).unwrap();

    let topmost_obstacles: HashMap<usize, usize> = HashMap::with_capacity(grid.cols());

    let (_, damage) = grid
        .indexed_iter()
        .fold((topmost_obstacles, 0), |(mut topmost_obstacles, mut damage), ((row, col), c)| {
            let topmost_obstacle = topmost_obstacles.entry(col).or_insert(0);

            match c {
                '#' => {
                    *topmost_obstacle = row + 1;
                }
                'O' => {
                    damage += grid.rows() - *topmost_obstacle;
                    *topmost_obstacle += 1;
                }
                _ => {}
            }

            (topmost_obstacles, damage)
        });

    Some(damage as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut grid: CustomGrid<char> = input_to_grid(input).unwrap();

    let mut memo = HashMap::new();
    let mut solves = HashMap::new();
    let mut i = 0usize;

    let mut bound: Option<usize> = None;

    let solve = loop {
        i += 1;
        tilt_up(&mut grid);
        grid.rotate_right();
        tilt_up(&mut grid);
        grid.rotate_right();
        tilt_up(&mut grid);
        grid.rotate_right();
        tilt_up(&mut grid);
        grid.rotate_right();

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
