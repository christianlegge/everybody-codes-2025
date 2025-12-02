use std::str::FromStr;

use anyhow::Error;
use itertools::Itertools;

#[derive(Debug)]
struct Board {
    squares: Vec<Square>,
    rows: i32,
    cols: i32,
}

#[derive(Debug, PartialEq)]
enum Square {
    Dragon,
    Sheep,
    Empty,
}

impl TryFrom<char> for Square {
    type Error = Error;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'D' => Ok(Self::Dragon),
            'S' => Ok(Self::Sheep),
            '.' => Ok(Self::Empty),
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

        for line in s.lines() {
            rows += 1;
            cols = i32::try_from(line.len())?;
            for c in line.chars() {
                squares.push(Square::try_from(c)?);
            }
        }

        match cols {
            0 => Err(anyhow::anyhow!("error counting cols: {s}")),
            _ => Ok(Self {
                squares,
                rows,
                cols,
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

    let mut moves = vec![dragon];
    let mut total_moves = moves.clone();
    for _ in 0..4 {
        moves = moves
            .iter()
            .flat_map(|m| get_knight_moves(m.0, m.1, board.rows))
            .collect::<Vec<(i32, i32)>>();
        total_moves.append(&mut moves.clone());
    }

    let dead_sheep = total_moves
        .iter()
        .filter(|(i, j)| board.get_square(*i, *j) == Some(&Square::Sheep))
        .count();

    Ok(dead_sheep.to_string())
}

pub fn solve2(data: &str) -> Result<String, Error> {
    println!("Text input: {data}");
    Ok("Unimplemented".to_string())
}

pub fn solve3(data: &str) -> Result<String, Error> {
    println!("Text input: {data}");
    Ok("Unimplemented".to_string())
}
