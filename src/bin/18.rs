use advent_of_code::custom_grid::Direction;
use advent_of_code::custom_grid::Direction::{Down, Left, Right, Up};

advent_of_code::solution!(18);

struct Line<T> {
    pub direction: Direction,
    pub n: T,
}

fn parse_input(input: &str) -> Vec<Line<u64>> {
    input
        .trim()
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();

            let direction = Direction::from_rdlu(parts.next().unwrap().chars().next().unwrap()).unwrap();
            let n = parts.next().unwrap().parse().unwrap();

            Line {
                direction,
                n,
            }
        })
        .collect()
}

fn parse_input_part_2(input: &str) -> Vec<Line<u64>> {
    input
        .trim()
        .lines()
        .map(|line| {
            let parts = line.split_whitespace();
            let real_part = parts.last().unwrap();
            let n = u64::from_str_radix(&real_part[2..7], 16).unwrap();
            let direction = match &real_part[7..8] {
                "0" => Right,
                "1" => Down,
                "2" => Left,
                "3" => Up,
                _ => unimplemented!()
            };

            Line {
                direction,
                n,
            }
        })
        .collect()
}

fn solve(lines: Vec<Line<u64>>) -> u64 {
    let (_, mut corners) =
        lines
            .iter()
            .fold(((0isize, 0isize), vec![]), |((mut row, mut col), mut points), Line { n, direction }| {
                points.push((row, col));
                match direction {
                    Up => row -= *n as isize,
                    Down => row += *n as isize,
                    Right => col += *n as isize,
                    Left => col -= *n as isize
                }

                ((row, col), points)
            });

    corners.push((0, 0));
    let (inside_points, border_points) = corners.windows(2).fold((0i64, 0i64), |(acc_ip, acc_bp), window| {
        if let [(row, col), (next_row, next_col)] = window {
            (
                acc_ip + (col + next_col) as i64 * (row - next_row) as i64,
                acc_bp + row.abs_diff(*next_row) as i64 + col.abs_diff(*next_col) as i64
            )

        } else {
            unreachable!()
        }
    });

    ((inside_points.abs() + border_points) /2 + 1) as u64
}

pub fn part_one(input: &str) -> Option<u64> {
    let lines = parse_input(input);

    Some(solve(lines))
}

pub fn part_two(input: &str) -> Option<u64> {
    let lines = parse_input_part_2(input);
    Some(solve(lines))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(62));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(952408144115));
    }
}
