use grid::*;
use std::fmt::{Debug, Display, Formatter};
use std::hash::{Hash, Hasher};
use std::ops::{Deref, DerefMut};
use std::str::FromStr;


#[derive(Clone, Copy, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl Direction {
    pub fn from_rdlu(rdlu: char) -> Result<Direction, ()> {
        match rdlu {
            'R' => Ok(Direction::Right),
            'D' => Ok(Direction::Down),
            'L' => Ok(Direction::Left),
            'U' => Ok(Direction::Up),
            _ => Err(())
        }
    }
}

#[derive(Eq, PartialEq, Clone)]
#[repr(transparent)]
pub struct CustomGrid<T: Eq + Hash>(Grid<T>);

impl<T> FromStr for CustomGrid<T>
    where
        T: Eq + Hash + FromStr
{
    type Err = <T as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = s.lines().map(|line| line.trim()).collect();
        let cols = lines[0].len();

        let grid_data: Result<Vec<T>, <T as FromStr>::Err> = lines
            .into_iter()
            .flat_map(|line| line.chars())
            .map(|c| c.to_string().parse::<T>())
            .collect();

        Ok(CustomGrid(Grid::from_vec(grid_data?, cols)))
    }
}

impl<T: Eq + Hash> Hash for CustomGrid<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for x in self.0.iter() {
            x.hash(state)
        }
    }
}

impl<T: Eq + Hash> Deref for CustomGrid<T> {
    type Target = Grid<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Eq + Hash> DerefMut for CustomGrid<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: Display + Eq + Hash> Debug for CustomGrid<T> {
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

impl<T: Eq + Hash> CustomGrid<T> {

    pub fn from_grid(grid: Grid<T>) -> Self {
        CustomGrid(grid)
    }

    pub fn is_border(&self, row: usize, col: usize) -> bool {
        row == 0 || row == self.rows() - 1 || col == 0 || col == self.cols() - 1
    }
    pub fn iter_neighbors(
        &self,
        row: usize,
        col: usize,
    ) -> impl Iterator<Item = ((usize, usize), &T)> {
        [(0, -1), (0, 1), (1, 0), (-1, 0)]
            .iter()
            .map(move |(col_offset, row_offset)| {
                ((col as isize + col_offset), (row as isize + row_offset))
            })
            .filter_map(|(col, row)| {
                if col.is_negative() || row.is_negative() {
                    None
                } else {
                    self.0
                        .get(row as usize, col as usize)
                        .map(|val| ((row as usize, col as usize), val))
                }
            })
    }
    pub fn iter_diagonal_neighbors(
        &self,
        row: usize,
        col: usize,
    ) -> impl Iterator<Item = ((usize, usize), &T)> {
        [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ]
        .iter()
        .map(move |(col_offset, row_offset)| {
            ((col as isize + col_offset), (row as isize + row_offset))
        })
        .filter_map(|(col, row)| {
            if col.is_negative() || row.is_negative() {
                None
            } else {
                self.0
                    .get(row as usize, col as usize)
                    .map(|val| ((row as usize, col as usize), val))
            }
        })
    }

    pub fn direction(&self, row: usize, col: usize, direction: Direction) -> Option<((usize, usize), &T)> {
        match direction {
            Direction::Up => self.up_indexed(row, col),
            Direction::Down => self.down_indexed(row, col),
            Direction::Right => self.right_indexed(row, col),
            Direction::Left => self.left_indexed(row, col),
        }
    }

    pub fn direction_indexed(&self, row: usize, col: usize, direction: Direction) -> Option<&T> {
        match direction {
            Direction::Up => self.up(row, col),
            Direction::Down => self.down(row, col),
            Direction::Right => self.right(row, col),
            Direction::Left => self.left(row, col),
        }
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
        self.0
            .get_mut(row, col + 1)
            .map(|val| ((row, col + 1), val))
    }

    pub fn left_mut(&mut self, row: usize, col: usize) -> Option<((usize, usize), &mut T)> {
        if col == 0 {
            None
        } else {
            self.0
                .get_mut(row, col - 1)
                .map(|val| ((row, col - 1), val))
        }
    }

    pub fn up_mut(&mut self, row: usize, col: usize) -> Option<((usize, usize), &mut T)> {
        if row == 0 {
            None
        } else {
            self.0
                .get_mut(row - 1, col)
                .map(|val| ((row - 1, col), val))
        }
    }

    pub fn down_mut(&mut self, row: usize, col: usize) -> Option<((usize, usize), &mut T)> {
        self.0
            .get_mut(row + 1, col)
            .map(|val| ((row + 1, col), val))
    }
}

impl<T: Display + Eq + Hash> CustomGrid<T> {
    pub fn print(grid: &CustomGrid<T>) {
        for row in 0..grid.0.rows() {
            for col in 0..grid.0.cols() {
                print!("{}", grid.0.get(row, col).unwrap())
            }
            println!()
        }
    }
}

pub fn input_to_grid<T: FromStr + Eq + Hash>(input: &str) -> Result<CustomGrid<T>, <T as FromStr>::Err> {
    let lines: Vec<&str> = input.lines().map(|line| line.trim()).collect();
    let cols = lines[0].len();

    let grid_data: Result<Vec<T>, <T as FromStr>::Err> = lines
        .into_iter()
        .flat_map(|line| line.chars())
        .map(|c| c.to_string().parse::<T>())
        .collect();

    Ok(CustomGrid(Grid::from_vec(grid_data?, cols)))
}
