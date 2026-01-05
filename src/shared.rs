use crossterm::{
    cursor,
    event::{KeyCode, KeyEvent},
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{self, Clear, ClearType, LeaveAlternateScreen},
};
use rand::prelude::*;
use std::{
    io::{self, ErrorKind, stdout},
    process::exit,
};

use crate::{enums::RawCommand, structs::*};

pub const GRID_SIZE: usize = 10;

pub const GRID_X_OFFSET: usize = 5;
pub const GRID_Y_OFFSET: usize = 1;

pub fn random_coord<R: Rng>(rng: &mut R) -> TerminalPos {
    TerminalPos(
        rng.random_range(0..GRID_SIZE) as u16,
        rng.random_range(0..GRID_SIZE) as u16,
    )
}

pub fn move_cursor(x: u16, y: u16) -> io::Result<()> {
    let mut stdout = stdout();
    execute!(stdout, cursor::MoveTo(x, y))?;
    Ok(())
}

pub fn key_to_command(key: KeyEvent) -> RawCommand {
    match key.code {
        KeyCode::Up => RawCommand::MoveUp,
        KeyCode::Down => RawCommand::MoveDown,
        KeyCode::Left => RawCommand::MoveLeft,
        KeyCode::Right => RawCommand::MoveRight,
        KeyCode::Enter => RawCommand::Interact,
        KeyCode::Char('s') | KeyCode::End => RawCommand::EndTurn,
        KeyCode::Char('Q') | KeyCode::Esc => RawCommand::QuitGame,
        _ => RawCommand::None,
    }
}

pub fn clear_screen() -> io::Result<()> {
    let mut stdout = stdout();
    execute!(stdout, Clear(ClearType::All), cursor::MoveTo(0, 0))?;
    Ok(())
}

pub fn print_with_color(text: &str, background: Color, foreground: Color) -> io::Result<()> {
    execute!(
        stdout(),
        SetForegroundColor(foreground),
        SetBackgroundColor(background),
        Print(text),
        ResetColor
    )?;
    Ok(())
}

pub fn wait_for_enter(silent: bool) -> io::Result<()> {
    if terminal::is_raw_mode_enabled()? {
        return Err(io::Error::new(
            ErrorKind::Unsupported,
            "Cannot be done on Raw Terminal Mode",
        ));
    }
    if !silent {
        println!("Press ENTER (Return) to continue.");
    }
    let _ = io::stdin().read_line(&mut String::new())?;
    Ok(())
}

pub fn quit(code: i32) -> ! {
    let _ = terminal::disable_raw_mode();
    let _ = execute!(io::stdout(), cursor::Show, LeaveAlternateScreen);
    exit(code);
}
