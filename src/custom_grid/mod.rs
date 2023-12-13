use std::fmt::{Debug, Display, Formatter};
use std::ops::{Deref, DerefMut};
use std::str::FromStr;
use grid::*;

#[repr(transparent)]
pub struct CustomGrid<T>(Grid<T>);

impl<T> Deref for CustomGrid<T> {
    type Target = Grid<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for CustomGrid<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: Display> Debug for CustomGrid<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in 0..self.0.rows() {
            for col in 0..self.0.cols() {
                write!(f, "{}", self.0.get(row, col).unwrap())?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<T> CustomGrid<T> {
    pub fn iter_neighbors(&self, row: usize, col: usize) -> impl Iterator<Item=((usize, usize), &T)> {
        [(0, -1), (0, 1), (1, 0), (-1, 0)]
            .iter()
            .map(move |(col_offset, row_offset)| ((col as isize + col_offset), (row as isize + row_offset)))
            .filter_map(|(col, row)| {
                if col.is_negative() || row.is_negative() {
                    None
                } else {
                    self.0.get(row as usize, col as usize)
                        .map(|val| ((row as usize, col as usize), val))
                }
            })
    }
    pub fn iter_diagonal_neighbors(&self, row: usize, col: usize) -> impl Iterator<Item=((usize, usize), &T)> {
        [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)]
            .iter()
            .map(move |(col_offset, row_offset)| ((col as isize + col_offset), (row as isize + row_offset)))
            .filter_map(|(col, row)| {
                if col.is_negative() || row.is_negative() {
                    None
                } else {
                    self.0.get(row as usize, col as usize)
                        .map(|val| ((row as usize, col as usize), val))
                }
            })
    }

    pub fn right(&self, row: usize, col: usize) -> Option<&T> {
        self.0.get(row, col + 1)
    }
    pub fn right_indexed(&self, row: usize, col: usize) -> Option<((usize, usize), &T)> {
        self.0.get(row, col + 1).map(|val| ((row, col + 1), val))
    }

    pub fn left(&self, row: usize, col: usize) -> Option<&T> {
        if col == 0 {
            None
        } else {
            self.0.get(row, col - 1)
        }
    }
    pub fn left_indexed(&self, row: usize, col: usize) -> Option<((usize, usize), &T)> {
        if col == 0 {
            None
        } else {
            self.0.get(row, col - 1).map(|val| ((row, col - 1), val))
        }
    }


    pub fn up(&self, row: usize, col: usize) -> Option<&T> {
        if row == 0 {
            None
        } else {
            self.0.get(row - 1, col)
        }
    }

    pub fn up_indexed(&self, row: usize, col: usize) -> Option<((usize, usize), &T)> {
        if row == 0 {
            None
        } else {
            self.0.get(row - 1, col).map(|val| ((row - 1, col), val))
        }
    }

    pub fn down(&self, row: usize, col: usize) -> Option<&T> {
        self.0.get(row + 1, col)
    }

    pub fn down_indexed(&self, row: usize, col: usize) -> Option<((usize, usize), &T)> {
        self.0.get(row + 1, col).map(|val| ((row + 1, col), val))
    }

    pub fn right_mut(&mut self, row: usize, col: usize) -> Option<((usize, usize), &mut T)> {
        self.0.get_mut(row, col + 1).map(|val| ((row, col + 1), val))
    }

    pub fn left_mut(&mut self, row: usize, col: usize) -> Option<((usize, usize), &mut T)> {
        if col == 0 {
            None
        } else {
            self.0.get_mut(row, col - 1).map(|val| ((row, col - 1), val))
        }
    }

    pub fn up_mut(&mut self, row: usize, col: usize) -> Option<((usize, usize), &mut T)> {
        if row == 0 {
            None
        } else {
            self.0.get_mut(row - 1, col).map(|val| ((row - 1, col), val))
        }
    }

    pub fn down_mut(&mut self, row: usize, col: usize) -> Option<((usize, usize), &mut T)> {
        self.0.get_mut(row + 1, col).map(|val| ((row + 1, col), val))
    }
}

impl<T: Display> CustomGrid<T> {
    pub fn print(grid: &CustomGrid<T>) {
        for row in 0..grid.0.rows() {
            for col in 0..grid.0.cols() {
                print!("{}", grid.0.get(row, col).unwrap())
            }
            println!()
        }
    }
}


pub fn input_to_grid<T: FromStr>(input: &str) -> Result<CustomGrid<T>, <T as FromStr>::Err> {
    let lines: Vec<&str> = input.lines().map(|line| line.trim()).collect();
    let cols = lines[0].len();

    let grid_data: Result<Vec<T>, <T as FromStr>::Err> = lines
        .into_iter()
        .flat_map(|line| line.chars())
        .map(|c| c.to_string().parse::<T>())
        .collect();

    Ok(CustomGrid(Grid::from_vec(grid_data?, cols)))
}