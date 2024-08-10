use crate::faces::Colours;
use crate::r#move::{Column, Face, Move, Row};
use std::fmt::Display;
pub const DIM: usize = 3;
pub const FACES: usize = 6;

type Moves = ([usize; DIM], [usize; DIM], [usize; DIM], [usize; DIM]);

static COLUMN_MOVES: [Moves; DIM] = [
    ([0, 3, 6], [9, 12, 15], [18, 21, 24], [27, 30, 33]), // Left column
    ([1, 4, 7], [10, 13, 16], [19, 22, 25], [28, 31, 34]), // Middle column
    ([2, 5, 8], [11, 14, 17], [20, 23, 26], [29, 32, 35]), // Right column
];

static ROW_MOVES: [Moves; DIM] = [
    ([0, 1, 2], [36, 37, 38], [18, 19, 20], [45, 46, 47]), // Top row
    ([3, 4, 5], [39, 40, 41], [21, 22, 23], [48, 49, 50]), // Middle row
    ([6, 7, 8], [42, 43, 44], [24, 25, 26], [51, 52, 53]), // Bottom row
];

#[derive(Default)]
struct Selection {
    face: Face,
    row: Row,
    col: Column,
}

pub struct StupidCube {
    faces: Vec<Colours>,
    selection: Selection,
}

impl StupidCube {
    pub fn new() -> Self {
        Default::default()
    }
    pub fn make_move(&mut self, move_: Move) -> anyhow::Result<()> {
        match move_ {
            Move::Up | Move::Down => {
                self.move_column(self.selection.col.get(), move_)?;
            }
            Move::Left | Move::Right => {
                self.move_row(self.selection.row.get(), move_)?;
            }
            Move::Pos((face, row, col, next)) => {
                self.selection.face = face;
                self.selection.row = row;
                self.selection.col = col;
                self.make_move(next.as_ref().clone())?;
            }
        }

        Ok(())
    }
    fn move_row(&mut self, row: usize, move_: Move) -> anyhow::Result<()> {
        let indices = ROW_MOVES[row];

        match move_ {
            Move::Left => {
                self.shift_row(indices.0, indices.3, indices.2, indices.1);
            }
            Move::Right => {
                self.shift_row(indices.0, indices.1, indices.2, indices.3);
            }
            _ => return Err(anyhow::anyhow!("Invalid move on row")),
        }
        Ok(())
    }

    fn move_column(&mut self, col: usize, move_: Move) -> anyhow::Result<()> {
        let indices = COLUMN_MOVES[col];

        match move_ {
            Move::Up => {
                self.shift_column(indices.0, indices.1, indices.2, indices.3);
            }
            Move::Down => {
                self.shift_column(indices.0, indices.3, indices.2, indices.1);
            }
            _ => return Err(anyhow::anyhow!("Invalid move on column")),
        }
        Ok(())
    }

    fn shift_row(
        &mut self,
        front: [usize; DIM],
        left: [usize; DIM],
        back: [usize; DIM],
        right: [usize; DIM],
    ) {
        let temp = [
            self.faces[left[0]],
            self.faces[left[1]],
            self.faces[left[2]],
        ];

        for i in 0..3 {
            self.faces[left[i]] = self.faces[front[i]];
            self.faces[front[i]] = self.faces[right[i]];
            self.faces[right[i]] = self.faces[back[i]];
            self.faces[back[i]] = temp[i];
        }
    }

    fn shift_column(
        &mut self,
        up: [usize; DIM],
        front: [usize; DIM],
        down: [usize; DIM],
        back: [usize; DIM],
    ) {
        let temp = [self.faces[up[0]], self.faces[up[1]], self.faces[up[2]]];

        for i in 0..3 {
            self.faces[up[i]] = self.faces[front[i]];
            self.faces[front[i]] = self.faces[down[i]];
            self.faces[down[i]] = self.faces[back[2 - i]];
            self.faces[back[2 - i]] = temp[i];
        }
    }
}

impl Default for StupidCube {
    fn default() -> Self {
        let cube = vec![
            vec![
                vec![Colours::White; DIM],
                vec![Colours::White; DIM],
                vec![Colours::White; DIM],
            ],
            vec![
                vec![Colours::Blue; DIM],
                vec![Colours::Blue; DIM],
                vec![Colours::Blue; DIM],
            ],
            vec![
                vec![Colours::Red; DIM],
                vec![Colours::Red; DIM],
                vec![Colours::Red; DIM],
            ],
            vec![
                vec![Colours::Yellow; DIM],
                vec![Colours::Yellow; DIM],
                vec![Colours::Yellow; DIM],
            ],
            vec![
                vec![Colours::Orange; DIM],
                vec![Colours::Orange; DIM],
                vec![Colours::Orange; DIM],
            ],
            vec![
                vec![Colours::Brown; DIM],
                vec![Colours::Brown; DIM],
                vec![Colours::Brown; DIM],
            ],
        ];
        let cube = cube
            .into_iter()
            .flat_map(|v| v.into_iter().flatten().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        StupidCube {
            faces: cube,
            selection: Selection::default(),
        }
    }
}

impl Display for StupidCube {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let cube = &self.faces;
        let mut s = String::new();
        for (i, colour) in cube.iter().enumerate() {
            if i % 3 == 0 {
                s.push_str("------------\n");
                if i > 0 && i % 9 == 0 {
                    s.push_str("\n------------\n");
                }
            }
            s.push_str(&format!(
                " {} |{}",
                to_ascii(colour),
                if ((i + 1) % 3) == 0 { "\n" } else { "" }
            ));
        }

        s.push_str("------------");
        write!(f, "{}", s)
    }
}

fn to_ascii(colour: &Colours) -> char {
    match colour {
        Colours::White => 'W',
        Colours::Blue => 'B',
        Colours::Red => 'R',
        Colours::Yellow => 'Y',
        Colours::Orange => 'O',
        Colours::Brown => 'N',
    }
}
