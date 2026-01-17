mod enums;
mod implementations;
mod shared;
mod structs;

use crossterm::{
    cursor,
    event::{self},
    execute, terminal,
};
use inquire::{self, InquireError};
use std::io::{ErrorKind, IsTerminal, Write, stdin};
use std::{
    fs,
    fs::OpenOptions,
    io::{self, stdout},
    panic,
};

use crate::{
    enums::{CityState, GameError, InputOutcome},
    structs::*,
};
use crate::{
    enums::{GameState, RawCommand},
    shared::*,
};

// Functions
fn main_menu_loop() -> io::Result<()> {
    println!("Welcome to Cell Wars!");
    loop {
        let main_menu =
            inquire::Select::new("Choose an option.", vec!["Play", "How To Play", "Quit"]);
        let result = main_menu.prompt();
        match result {
            Ok("Play") => {
                break;
            }
            Ok("How To Play") => {
                println!("The goal of the game is to destroy your opponent's cities.");
                println!("You both start out with one city, and must gather resources.");
                wait_for_enter(false)?;
            }
            Ok("Quit") => {
                quit(0);
            }
            Ok(&_) => {}
            Err(InquireError::OperationCanceled) => {
                quit(1);
            }
            Err(InquireError::OperationInterrupted) => {
                quit(1);
            }
            Err(_) => {
                quit(1);
            }
        }
    }
    Ok(())
}

fn do_input(game: &mut Game) -> io::Result<InputOutcome> {
    if let event::Event::Key(key) = event::read()? {
        match key_to_command(key) {
            RawCommand::MoveUp => {
                let cursor_pos = game.cursor_loc;
                game.cursor_up();
                if cursor_pos != game.cursor_loc {
                    return Ok(InputOutcome::Redraw);
                }
                Ok(InputOutcome::None)
            }
            RawCommand::MoveDown => {
                let cursor_pos = game.cursor_loc;
                game.cursor_down();
                if cursor_pos != game.cursor_loc {
                    return Ok(InputOutcome::Redraw);
                }
                Ok(InputOutcome::None)
            }
            RawCommand::MoveLeft => {
                let cursor_pos = game.cursor_loc;
                game.cursor_left();
                if cursor_pos != game.cursor_loc {
                    return Ok(InputOutcome::Redraw);
                }
                Ok(InputOutcome::None)
            }
            RawCommand::MoveRight => {
                let cursor_pos = game.cursor_loc;
                game.cursor_right();
                if cursor_pos != game.cursor_loc {
                    return Ok(InputOutcome::Redraw);
                }
                Ok(InputOutcome::None)
            }
            RawCommand::Interact => Ok(InputOutcome::Interact),
            RawCommand::EndTurn => Ok(InputOutcome::EndTurn),
            RawCommand::QuitGame => Ok(InputOutcome::QuitGame),
            RawCommand::None => Ok(InputOutcome::None),
        }
    } else {
        Ok(InputOutcome::None)
    }
}

fn player_interact(pos: TerminalPos, game: &mut Game) -> Result<(), GameError> {
    game.is_valid_grid_position(pos)?;
    let cell = game.get_cell_at_pos(pos)?;

    if cell.blocked {
        game.status = Some("That's just a wall.");
        return Err(GameError::NoCityAtTarget);
    }

    if cell.city.is_none() {
        game.status = Some("There's no city there!");
        return Err(GameError::NoCityAtTarget);
    }

    let city = game.get_city_at_pos(pos)?;

    if city.state == CityState::OwnedByComputer {
        game.status = Some("You cannot act on an opposing city!");
        return Err(GameError::NoCityAtTarget);
    }

    if city.state == CityState::Destroyed {
        game.status = Some("That city is destroyed...");
        return Err(GameError::NoCityAtTarget);
    }

    // we're all good

    // let action_menu = inquire::Select::new("Choose an action...", vec![]);

    Ok(())
}

fn player_turn(game: &mut Game) -> io::Result<()> {
    game.state = GameState::PlayerTurn;
    loop {
        let input = do_input(game)?;
        match input {
            InputOutcome::Redraw => {
                game.draw()?;
                game.status = None;
            }
            InputOutcome::Interact => {
                let _ = player_interact(game.cursor_loc, game);
            }
            InputOutcome::EndTurn => {
                break;
            }
            InputOutcome::None => {}
            InputOutcome::QuitGame => {
                game.state = GameState::Stalemate;
                break;
            }
        }
    }
    Ok(())
}

// This will panic. Please implement first!
// todo: implement computer_turn
fn computer_turn(game: &mut Game) {
    game.state = GameState::ComputerTurn;
    todo!("todo: implement computer_turn.");
}

fn main_game_loop(game: &mut Game) -> io::Result<()> {
    execute!(stdout(), cursor::Hide)?;
    game.draw()?;
    terminal::enable_raw_mode()?;
    while game.state != GameState::PlayerWon
        && game.state != GameState::ComputerWon
        && game.state != GameState::Stalemate
    {
        player_turn(game)?;
        computer_turn(game);
    }
    Ok(())
}

fn is_in_terminal() -> bool {
    stdout().is_terminal() && stdin().is_terminal()
}

fn supports_tui() -> bool {
    stdin().is_terminal()
        && stdout().is_terminal()
        && terminal::enable_raw_mode().is_ok_and(|_| {
            let _ = terminal::disable_raw_mode();
            true
        })
}

fn check_if_terminal() {
    if !is_in_terminal() {
        if let Ok(mut file) = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open("OPEN_IN_TERMINAL.txt")
        {
            let _ = writeln!(file, "Hey, you there! This is a TUI game!");
            let _ = writeln!(file, "That means you should be opening this in a TERMINAL!");
            let _ = writeln!(file, "Don't just open it from your file manager!");
            let _ = writeln!(
                file,
                "Assuming you're on Linux (because that's usually how you get this error),"
            );
            let _ = writeln!(file, "Here's what to do:");
            let _ = writeln!(file, "\t1. `cd` to this directory");
            let _ = writeln!(
                file,
                "\t2. Type the name of this program in (') single quotes"
            );
            let _ = writeln!(file, "\t3. Press ENTER and play!");
            let _ = writeln!(
                file,
                "\nThis file will delete itself once the program is properly opened."
            );
        }
        quit(-1);
    }

    if !supports_tui() {
        println!("Sorry, whatever you're running this in doesn't support TUI.");
        println!("Please use a different terminal emulator.");
        quit(-1);
    }

    // Check for OPEN_IN_TERMINAL.txt

    match fs::remove_file("OPEN_IN_TERMINAL.txt") {
        Ok(()) => {}
        Err(e) if e.kind() == ErrorKind::NotFound => {}
        Err(e) => {
            println!("{}", e.kind());
            wait_for_enter(false);
        }
    }
}

// Main

fn main() -> io::Result<()> {
    panic::set_hook(Box::new(|info| {
        if let Ok(mut file) = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open("panic.log")
        {
            let _ = writeln!(file, "Oops.");
            let _ = writeln!(file, "{}", info);
            let _ = writeln!(
                file,
                "\nBacktrace:\n{}",
                std::backtrace::Backtrace::force_capture()
            );
        }
    }));

    check_if_terminal();

    let guard = GameGuard::new();
    let mut game = Game::default();
    main_menu_loop()?;
    game.prompt_difficulty();
    clear_screen()?;
    game.generate_random_map();
    main_game_loop(&mut game)?;
    drop(guard);
    Ok(())
}
