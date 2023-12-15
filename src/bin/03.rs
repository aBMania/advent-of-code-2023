use advent_of_code::custom_grid::{input_to_grid, CustomGrid};
use itertools::Itertools;
use std::collections::BTreeMap;
use std::iter::once;
advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let grid: CustomGrid<char> = input_to_grid::<char>(input).unwrap();

    let mut iterator = grid.indexed_iter().peekable();
    let mut sum = 0;

    while let Some(((row, col), value)) = iterator.next() {
        match *value {
            _ if value.is_ascii_digit() => {}
            _ => continue,
        }

        let group: Vec<_> = once(((row, col), value))
            .chain(iterator.peeking_take_while(|(_, value)| value.is_ascii_digit()))
            .collect();

        let symbol = group
            .iter()
            .filter_map(|&((row, col), _)| {
                grid.iter_diagonal_neighbors(row, col)
                    .filter(|(_, &neighbor_value)| {
                        neighbor_value != '.' && !neighbor_value.is_ascii_digit()
                    })
                    .map(|(pos, _)| pos)
                    .next()
            })
            .next();

        if symbol.is_some() {
            let group_sum: u32 = group
                .into_iter()
                .map(|(_, value)| value)
                .collect::<String>()
                .parse()
                .unwrap();

            sum += group_sum;
        }
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid: CustomGrid<char> = input_to_grid::<char>(input).unwrap();

    let mut iterator = grid.indexed_iter().peekable();
    let mut gears: BTreeMap<(usize, usize), Vec<u32>> = BTreeMap::new();

    while let Some(((row, col), value)) = iterator.next() {
        match *value {
            _ if value.is_ascii_digit() => {}
            _ => continue,
        }

        let group: Vec<_> = once(((row, col), value))
            .chain(iterator.peeking_take_while(|(_, value)| value.is_ascii_digit()))
            .collect();

        let gear = group
            .iter()
            .filter_map(|&((row, col), _)| {
                grid.iter_diagonal_neighbors(row, col)
                    .filter(|(_, &neighbor_value)| neighbor_value == '*')
                    .map(|(pos, _)| pos)
                    .next()
            })
            .next();

        if let Some(gear) = gear {
            let group_sum: u32 = group
                .into_iter()
                .map(|(_, value)| value)
                .collect::<String>()
                .parse()
                .unwrap();

            gears.entry(gear).or_default().push(group_sum);
        }
    }

    Some(
        gears
            .iter()
            .filter_map(|(_, sums)| match sums.len() {
                2 => Some(sums.first().unwrap() * sums.get(1).unwrap()),
                _ => None,
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
