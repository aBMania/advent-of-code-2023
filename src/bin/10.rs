use std::collections::HashMap;
use advent_of_code::custom_grid::{CustomGrid, input_to_grid};
advent_of_code::solution!(10);

#[derive(Clone, Copy, PartialEq)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

fn step(
    grid: &CustomGrid<char>,
    next_row: usize,
    next_col: usize,
    direction: Direction,
) -> (usize, usize, Direction) {
    let direction = match (grid.get(next_row, next_col), direction) {
        (Some('│'), Direction::Down) => Direction::Down,
        (Some('│'), Direction::Up) => Direction::Up,
        (Some('─'), Direction::Right) => Direction::Right,
        (Some('─'), Direction::Left) => Direction::Left,
        (Some('└'), Direction::Down) => Direction::Right,
        (Some('└'), Direction::Left) => Direction::Up,
        (Some('┘'), Direction::Down) => Direction::Left,
        (Some('┘'), Direction::Right) => Direction::Up,
        (Some('┐'), Direction::Right) => Direction::Down,
        (Some('┐'), Direction::Up) => Direction::Left,
        (Some('┌'), Direction::Left) => Direction::Down,
        (Some('┌'), Direction::Up) => Direction::Right,
        _ => unreachable!("should not come from this direction/char"),
    };

    let ((next_row, next_col), _) = match direction {
        Direction::Up => grid.up_indexed(next_row, next_col).unwrap(),
        Direction::Down => grid.down_indexed(next_row, next_col).unwrap(),
        Direction::Right => grid.right_indexed(next_row, next_col).unwrap(),
        Direction::Left => grid.left_indexed(next_row, next_col).unwrap()
    };

    (next_row, next_col, direction)
}

fn start_point(grid: &CustomGrid<char>) -> (usize, usize) {
    grid
        .indexed_iter()
        .find(|(_, &c)| c == 'S')
        .map(|((start_row, start_col), _)| (start_row, start_col))
        .expect("no S in grid")
}

fn replace_with_box_char(grid: &mut CustomGrid<char>) {
    for c in grid.iter_mut() {
        match *c {
            'F' => *c = '┌',
            '7' => *c = '┐',
            '-' => *c = '─',
            '|' => *c = '│',
            'J' => *c = '┘',
            'L' => *c = '└',
            '.' => *c = '•',
            'S' => {}
            _ => panic!("invalid char")
        }
    }
}

fn replace_starting_point(grid: &mut CustomGrid<char>, start_row: usize, start_col: usize) -> Direction {
    // Replace starting point
    match (
        grid.left(start_row, start_col),
        grid.up(start_row, start_col),
        grid.right(start_row, start_col),
        grid.down(start_row, start_col),
    ) {
        (Some('─' | '┌' | '└'), Some('┌' | '│' | '┐'), _, _) => {
            *grid.get_mut(start_row, start_col).unwrap() = '┘';
            Direction::Down
        }
        (Some('─' | '┌' | '└'), _, Some('─' | '┘' | '┐'), _) => {
            *grid.get_mut(start_row, start_col).unwrap() = '─';
            Direction::Right
        }
        (Some('─' | '┌' | '└'), _, _, Some('│' | '┘' | '└')) => {
            *grid.get_mut(start_row, start_col).unwrap() = '┐';
            Direction::Up
        }
        (_, Some('┌' | '│' | '┐'), Some('─' | '┘' | '┐'), _) => {
            *grid.get_mut(start_row, start_col).unwrap() = '└';
            Direction::Down
        }
        (_, Some('┌' | '│' | '┐'), _, Some('│' | '┘' | '└')) => {
            *grid.get_mut(start_row, start_col).unwrap() = '│';
            Direction::Up
        }
        (_, _, Some('─' | '┘' | '┐'), Some('│' | '┘' | '└')) => {
            *grid.get_mut(start_row, start_col).unwrap() = '┌';
            Direction::Up
        }
        _ => panic!("invalid start")
    }
}

fn count_inside_space(grid: &CustomGrid<char>) -> u32 {
    #[derive(Debug, PartialEq)]
    enum State {
        Outside,
        Inside,
        BottomCorner(bool),
        TopCorner(bool),
    }
    let mut count = 0;

    // Knot theory magic
    for row in 0..grid.rows() {
        let mut state = State::Outside;
        for col in 0..grid.cols() {
            state = match (state, grid.get(row, col).unwrap()) {
                (State::Outside, '│') => State::Inside,
                (State::Inside, '│') => State::Outside,
                (State::Inside, ' ') => {
                    count += 1;
                    State::Inside
                }
                (State::Outside, '┌') => State::BottomCorner(false),
                (State::Outside, '└') => State::TopCorner(false),
                (State::Inside, '┌') => State::BottomCorner(true),
                (State::Inside, '└') => State::TopCorner(true),
                (State::BottomCorner(inside), '─') => State::BottomCorner(inside),
                (State::BottomCorner(true), '┐') => State::Inside,
                (State::BottomCorner(true), '┘') => State::Outside,
                (State::BottomCorner(false), '┐') => State::Outside,
                (State::BottomCorner(false), '┘') => State::Inside,
                (State::TopCorner(inside), '─') => State::TopCorner(inside),
                (State::TopCorner(true), '┐') => State::Outside,
                (State::TopCorner(true), '┘') => State::Inside,
                (State::TopCorner(false), '┐') => State::Inside,
                (State::TopCorner(false), '┘') => State::Outside,
                (state, _) => state
            };
        }
    };
    count
}


pub fn part_one(input: &str) -> Option<u32> {
    let mut grid: CustomGrid<char> = input_to_grid(input).unwrap();

    let (start_row, start_col) = start_point(&grid);

    replace_with_box_char(&mut grid);
    let start_direction = replace_starting_point(&mut grid, start_row, start_col);

    let (mut next_row, mut next_col, mut direction) = (start_row, start_col, start_direction);

    let mut path_length = 0;
    loop {
        path_length += 1;
        (next_row, next_col, direction) = step(&grid, next_row, next_col, direction);

        if (next_row, next_col) == (start_row, start_col) {
            break;
        }
    }

    Some(path_length / 2)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut grid: CustomGrid<char> = input_to_grid(input).unwrap();

    let (start_row, start_col) = start_point(&grid);

    replace_with_box_char(&mut grid);
    let start_direction = replace_starting_point(&mut grid, start_row, start_col);

    let mut path = HashMap::from([
        ((start_row, start_col), true)
    ]);
    let (mut next_row, mut next_col, mut direction) = (start_row, start_col, start_direction);

    loop {
        path.insert((next_row, next_col), true);
        (next_row, next_col, direction) = step(&grid, next_row, next_col, direction);

        if (next_row, next_col) == (start_row, start_col) {
            break;
        }
    }

    // Remove everything not in path
    for row in 0..grid.rows() {
        for col in 0..grid.cols() {
            if !path.contains_key(&(row, col)) {
                *grid.get_mut(row, col).unwrap() = ' ';
            }
        }
    }

    Some(
        count_inside_space(&grid)
    )
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(80));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(10));
    }
}
