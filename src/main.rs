use crossterm::event::{Event, KeyCode, KeyEvent};
use crossterm::{event, ExecutableCommand};

use stupidcube::cube::StupidCube;
use stupidcube::r#move::Move;

fn render(cube: &StupidCube) -> anyhow::Result<()> {
    let mut stdout = std::io::stdout();
    stdout.execute(crossterm::terminal::Clear(
        crossterm::terminal::ClearType::All,
    ))?;
    println!("{}", cube);
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
