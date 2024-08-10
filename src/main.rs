use crossterm::event::{Event, KeyCode, KeyEvent};
use crossterm::style::{ContentStyle, PrintStyledContent, SetForegroundColor, StyledContent};
use crossterm::terminal::{Clear, ClearType};
use crossterm::{cursor, event, ExecutableCommand};
use std::io::Write;
use stupidcube::cube::StupidCube;
use stupidcube::r#move::Move;

fn render(cube: &StupidCube) -> anyhow::Result<()> {
    let (width, height) = crossterm::terminal::size()?;
    let width = width.into(); // Assume terminal width
    let height = height.into(); // Assume terminal height

    let mut stdout = std::io::stdout();
    stdout.execute(Clear(ClearType::All)).unwrap();
    stdout.execute(cursor::MoveTo(0, 0)).unwrap();

    // Define the 8 vertices of the cube in 3D space
    let vertices = [
        (-1.0, -1.0, -1.0), // Back-bottom-left
        (1.0, -1.0, -1.0),  // Back-bottom-right
        (1.0, 1.0, -1.0),   // Back-top-right
        (-1.0, 1.0, -1.0),  // Back-top-left
        (-1.0, -1.0, 1.0),  // Front-bottom-left
        (1.0, -1.0, 1.0),   // Front-bottom-right
        (1.0, 1.0, 1.0),    // Front-top-right
        (-1.0, 1.0, 1.0),   // Front-top-left
    ];

    // Faces defined by indices into the vertices array
    let faces = [
        [0, 1, 2, 3], // Back face
        [4, 5, 6, 7], // Front face
        [0, 4, 7, 3], // Left face
        [1, 5, 6, 2], // Right face
        [3, 2, 6, 7], // Top face
        [0, 1, 5, 4], // Bottom face
    ];

    // Project and draw each face
    for (face_idx, &face) in faces.iter().enumerate() {
        let color_index = face_idx * 9; // Each face has 9 colors

        for i in 0..3 {
            for j in 0..3 {
                let (x0, y0, z0) = vertices[face[0]];
                let (x1, y1, z1) = vertices[face[1]];
                // let (x2, y2, z2) = vertices[face[2]];
                let (x3, y3, z3) = vertices[face[3]];

                let x = x0 + (x1 - x0) * j as f32 / 2.0 + (x3 - x0) * i as f32 / 2.0;
                let y = y0 + (y1 - y0) * j as f32 / 2.0 + (y3 - y0) * i as f32 / 2.0;
                let z = z0 + (z1 - z0) * j as f32 / 2.0 + (z3 - z0) * i as f32 / 2.0;

                let (x, y, z) = cube.rotate_point(x, y, z);
                let (x_proj, y_proj) = cube.project_point(x, y, z, width, height);

                // Get the color for this square
                let color: crossterm::style::Color = cube.faces()[color_index + i * 3 + j].into();

                stdout
                    .execute(cursor::MoveTo(x_proj as u16, y_proj as u16))
                    .unwrap();
                stdout.execute(SetForegroundColor(color)).unwrap();
                stdout
                    .execute(PrintStyledContent(StyledContent::new(
                        ContentStyle::new(),
                        "x",
                    )))
                    .unwrap();
            }
        }
    }

    stdout.flush()?;
    /*   let mut stdout = std::io::stdout();
    stdout.execute(crossterm::terminal::Clear(
        crossterm::terminal::ClearType::All,
    ))?;
    println!("{}", cube);*/
    Ok(())
}

fn main() -> anyhow::Result<()> {
    let mut cube = StupidCube::new();
    let mut stdout = std::io::stdout();

    render(&cube)?;

    loop {
        let eve = event::read()?;
        let move_ = if let Event::Key(KeyEvent { code, .. }) = eve {
            match code {
                KeyCode::Char('w') => Move::Up,
                KeyCode::Char('s') => Move::Down,
                KeyCode::Char('a') => Move::Left,
                KeyCode::Char('d') => Move::Right,
                KeyCode::Left => {
                    cube.rotate_y(10.0);
                    render(&cube)?;
                    continue;
                }
                KeyCode::Right => {
                    cube.rotate_y(-10.0);
                    render(&cube)?;
                    continue;
                }
                KeyCode::Up => {
                    cube.rotate_x(10.0);
                    render(&cube)?;
                    continue;
                }
                KeyCode::Down => {
                    cube.rotate_x(-10.0);
                    render(&cube)?;
                    continue;
                }
                KeyCode::Char('c') => {
                    stdout.execute(crossterm::terminal::Clear(
                        crossterm::terminal::ClearType::FromCursorUp,
                    ))?;
                    render(&cube)?;
                    continue;
                }
                KeyCode::Esc => break,
                _ => continue,
            }
        } else {
            continue;
        };

        if let Err(e) = cube.make_move(move_) {
            eprintln!("An error occurred: {}", e);
        }

        render(&cube)?
    }

    Ok(())
}
