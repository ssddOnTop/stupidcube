use crate::cube::{DIM, FACES};

#[derive(Clone, Default)]
pub struct Face(usize);

#[derive(Clone, Default)]
pub struct Row(usize);

#[derive(Clone, Default)]
pub struct Column(usize);

impl Face {
    pub fn get(&self) -> usize {
        self.0
    }
}

impl Row {
    pub fn get(&self) -> usize {
        self.0
    }
}

impl Column {
    pub fn get(&self) -> usize {
        self.0
    }
}

#[derive(Clone)]
pub enum Move {
    Up,
    Down,
    Left,
    Right,
    Pos((Face, Row, Column, Box<Move>)),
}

impl TryFrom<usize> for Column {
    type Error = anyhow::Error;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        if value > DIM {
            Err(anyhow::anyhow!("Invalid column index"))
        } else {
            Ok(Self(value))
        }
    }
}

impl TryFrom<usize> for Row {
    type Error = anyhow::Error;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        if value > DIM {
            Err(anyhow::anyhow!("Invalid row index"))
        } else {
            Ok(Self(value))
        }
    }
}

impl TryFrom<usize> for Face {
    type Error = anyhow::Error;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        if value > FACES {
            Err(anyhow::anyhow!("Invalid face index"))
        } else {
            Ok(Self(value))
        }
    }
}
