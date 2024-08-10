use std::fmt::{Display, Formatter};

use crate::faces::Colours;
use crate::r#move::{Column, Face, Move, Row};

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

static INDICES: [[usize; 9]; 6] = [
    // Top face
    [0, 1, 2, 3, 4, 5, 6, 7, 8],
    // Left, Front, Right faces
    [9, 10, 11, 12, 13, 14, 15, 16, 17],
    [18, 19, 20, 21, 22, 23, 24, 25, 26],
    [27, 28, 29, 30, 31, 32, 33, 34, 35],
    // Bottom face
    [36, 37, 38, 39, 40, 41, 42, 43, 44],
    // Back face
    [45, 46, 47, 48, 49, 50, 51, 52, 53],
];

#[derive(Default)]
struct Selection {
    face: Face,
    row: Row,
    col: Column,
}

#[derive(derive_getters::Getters)]
pub struct StupidCube {
    faces: Vec<Colours>,
    selection: Selection,
    rotation_x: f32,
    rotation_y: f32,
    rotation_z: f32,
}

// TODO: merge this with the other impl block
impl StupidCube {
    pub fn rotate_x(&mut self, angle: f32) {
        self.rotation_x += angle;
    }

    pub fn rotate_y(&mut self, angle: f32) {
        self.rotation_y += angle;
    }

    pub fn rotate_z(&mut self, angle: f32) {
        self.rotation_z += angle;
    }
    pub fn rotate_point(&self, x: f32, y: f32, z: f32) -> (f32, f32, f32) {
        let (sin_x, cos_x) = self.rotation_x.to_radians().sin_cos();
        let (sin_y, cos_y) = self.rotation_y.to_radians().sin_cos();
        let (sin_z, cos_z) = self.rotation_z.to_radians().sin_cos();

        // Rotate around X-axis
        let (y, z) = (y * cos_x - z * sin_x, y * sin_x + z * cos_x);
        // Rotate around Y-axis
        let (x, z) = (x * cos_y + z * sin_y, -x * sin_y + z * cos_y);
        // Rotate around Z-axis
        let (x, y) = (x * cos_z - y * sin_z, x * sin_z + y * cos_z);

        (x, y, z)
    }
    pub fn project_point(&self, x: f32, y: f32, z: f32, width: u32, height: u32) -> (u32, u32) {
        // Simple perspective projection
        let fov = 50.0;
        let distance = 4.0;
        let factor = fov / (distance + z);
        let x_proj = x * factor + width as f32 / 2.0;
        let y_proj = y * factor + height as f32 / 2.0;

        (x_proj as u32, y_proj as u32)
    }
    pub fn face_normal(&self, face: [usize; 4], vertices: &[(f32, f32, f32)]) -> (f32, f32, f32) {
        let (x1, y1, z1) = vertices[face[0]];
        let (x2, y2, z2) = vertices[face[1]];
        let (x3, y3, z3) = vertices[face[2]];

        let u = (x2 - x1, y2 - y1, z2 - z1);
        let v = (x3 - x1, y3 - y1, z3 - z1);

        (
            u.1 * v.2 - u.2 * v.1,
            u.2 * v.0 - u.0 * v.2,
            u.0 * v.1 - u.1 * v.0,
        )
    }

    pub fn is_face_visible(&self, face: [usize; 4], vertices: &[(f32, f32, f32)]) -> bool {
        let normal = self.face_normal(face, vertices);
        normal.2 < 0.0
    }
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
                vec![Colours::Magenta; DIM],
                vec![Colours::Magenta; DIM],
                vec![Colours::Magenta; DIM],
            ],
            vec![
                vec![Colours::Cyan; DIM],
                vec![Colours::Cyan; DIM],
                vec![Colours::Cyan; DIM],
            ],
        ];
        let cube = cube
            .into_iter()
            .flat_map(|v| v.into_iter().flatten().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        StupidCube {
            faces: cube,
            selection: Selection::default(),
            rotation_x: 0.0,
            rotation_y: 0.0,
            rotation_z: 0.0,
        }
    }
}

impl Display for StupidCube {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        const PADDING: &str = "       ";

        for row in 0..3 {
            write!(f, "{}", PADDING)?; // Padding for top face
            for col in 0..3 {
                write!(f, "{} ", self.faces[INDICES[0][row * 3 + col]].as_str())?;
            }
            writeln!(f)?;
        }

        for row in 0..3 {
            for (i, face) in INDICES.iter().enumerate().take(3 + 1).skip(1) {
                for col in 0..3 {
                    write!(f, "{} ", self.faces[face[row * 3 + col]].as_str())?;
                }
                if i < 3 {
                    write!(f, " ")?; // Padding between faces
                }
            }
            writeln!(f)?;
        }

        for row in 0..3 {
            write!(f, "{}", PADDING)?; // Padding for bottom face
            for col in 0..3 {
                write!(f, "{} ", self.faces[INDICES[4][row * 3 + col]].as_str())?;
            }
            writeln!(f)?;
        }

        for row in 0..3 {
            write!(f, "{}", PADDING)?; // Padding for back face
            for col in 0..3 {
                write!(f, "{} ", self.faces[INDICES[5][row * 3 + col]].as_str())?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}
