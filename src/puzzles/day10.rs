use std::{fmt::Display, str::FromStr};

use anyhow::Error;
use hashbrown::HashSet;
use itertools::Itertools;

#[derive(Debug)]
struct Board {
    squares: Vec<Square>,
    rows: i32,
    cols: i32,
    hideouts: Vec<usize>,
    eaten_sheep: i32,
}

#[derive(Debug, PartialEq)]
enum Square {
    Dragon,
    Sheep,
    Empty,
    Hideout,
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.rows {
            for j in 0..self.cols {
                write!(
                    f,
                    "{}",
                    self.get_square(i, j).expect("Checked invalid square")
                )?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Dragon => 'D',
                Self::Sheep => 'S',
                Self::Empty => '.',
                Self::Hideout => '#',
            }
        )
    }
}

impl TryFrom<char> for Square {
    type Error = Error;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'D' => Ok(Self::Dragon),
            'S' => Ok(Self::Sheep),
            '.' => Ok(Self::Empty),
            '#' => Ok(Self::Hideout),
            c => Err(anyhow::anyhow!("Unknown board character {c}")),
        }
    }
}

impl FromStr for Board {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut rows = 0;
        let mut cols = 0;
        let mut squares = Vec::new();
        let mut hideouts = Vec::new();

        for (li, line) in s.lines().enumerate() {
            rows += 1;
            cols = i32::try_from(line.len())?;
            for (ci, c) in line.chars().enumerate() {
                let square = Square::try_from(c)?;
                if square == Square::Hideout {
                    hideouts.push(li * usize::try_from(cols)? + ci);
                }
                squares.push(square);
            }
        }

        match cols {
            0 => Err(anyhow::anyhow!("error counting cols: {s}")),
            _ => Ok(Self {
                squares,
                rows,
                cols,
                hideouts,
                eaten_sheep: 0,
            }),
        }
    }
}

impl Board {
    pub fn get_square(&self, row: i32, col: i32) -> Option<&Square> {
        let idx = usize::try_from(row * self.cols + col);
        idx.map_or(None, |idx| self.squares.get(idx))
    }

    pub const fn get_coords(&self, idx: i32) -> (i32, i32) {
        (idx / self.cols, idx % self.cols)
    }

    pub fn find_dragon(&self) -> Option<(i32, i32)> {
        let pos = self.squares.iter().find_position(|&s| s == &Square::Dragon);
        pos.and_then(|i| i32::try_from(i.0).map_or(None, |j| Some(self.get_coords(j))))
    }

    pub fn move_sheep(&mut self) {
        for idx in (0..self.squares.len()).rev() {
            let (row, col) =
                self.get_coords(i32::try_from(idx).expect("error converting coordinate to usize"));
            if row == 0 {
                self.squares[idx] = Square::Empty;
                continue;
            }

            let above = self.get_square(row - 1, col);

            if above == Some(&Square::Sheep) {
                self.squares[idx] = Square::Sheep;
                self.squares[idx
                    - usize::try_from(self.cols).expect("error converting coordinate to usize")] =
                    Square::Empty;
            } else if above == Some(&Square::Empty)
                && self.get_square(row, col) == Some(&Square::Sheep)
            {
                self.squares[idx] = Square::Empty;
            }
        }
    }

    pub fn eat_sheep(&mut self, dragons: &HashSet<(i32, i32)>) {
        for idx in 0..self.squares.len() {
            let (row, col) =
                self.get_coords(idx.try_into().expect("error converting idx to coord"));
            if self.get_square(row, col) == Some(&Square::Sheep)
                && dragons.contains(&(row, col))
                && !self.hideouts.contains(&idx)
            {
                self.squares[idx] = Square::Empty;
                self.eaten_sheep += 1;
            }
        }
    }
}

fn get_knight_moves(row: i32, col: i32, clamp: i32) -> Vec<(i32, i32)> {
    let mut moves = Vec::new();
    for (i, j) in &[
        (2, 1),
        (-2, 1),
        (2, -1),
        (-2, -1),
        (1, 2),
        (1, -2),
        (-1, 2),
        (-1, -2),
    ] {
        if row + i < clamp && col + j < clamp && row + i >= 0 && col + j >= 0 {
            moves.push((row + i, col + j));
        }
    }
    moves
}

pub fn solve1(data: &str) -> Result<String, Error> {
    println!("Text input: {data}");
    let board = Board::from_str(data)?;
    let dragon = board
        .find_dragon()
        .ok_or_else(|| anyhow::anyhow!("no dragon found!"))?;

    let mut moves = HashSet::new();
    moves.insert(dragon);
    let mut total_moves = moves.clone();
    for _ in 0..4 {
        moves = moves
            .iter()
            .flat_map(|m| get_knight_moves(m.0, m.1, board.rows))
            .collect::<HashSet<(i32, i32)>>();
        for m in &moves {
            total_moves.insert(*m);
        }
    }

    let dead_sheep = total_moves
        .iter()
        .filter(|(i, j)| board.get_square(*i, *j) == Some(&Square::Sheep))
        .count();

    Ok(dead_sheep.to_string())
}

pub fn solve2(data: &str) -> Result<String, Error> {
    println!("Text input: {data}");
    let mut board = Board::from_str(data)?;

    let dragon = board
        .find_dragon()
        .ok_or_else(|| anyhow::anyhow!("no dragon found!"))?;

    let mut moves = HashSet::new();
    moves.insert(dragon);
    for _ in 0..20 {
        moves = moves
            .iter()
            .flat_map(|m| get_knight_moves(m.0, m.1, board.rows))
            .collect::<HashSet<(i32, i32)>>();
        board.eat_sheep(&moves);
        board.move_sheep();
        board.eat_sheep(&moves);
    }

    Ok(board.eaten_sheep.to_string())
}

pub fn solve3(data: &str) -> Result<String, Error> {
    println!("Text input: {data}");
    Ok("Unimplemented".to_string())
}
