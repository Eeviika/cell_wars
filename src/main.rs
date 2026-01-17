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
use std::io::{IsTerminal, Write, stderr, stdin};
use std::{
    fs::OpenOptions,
    io::{self, stdout},
    panic,
};

use crate::structs::*;
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

fn input_loop(game: &mut Game) -> io::Result<()> {
    loop {
        if let event::Event::Key(key) = event::read()? { match key_to_command(key) {
            RawCommand::MoveUp => {
                game.cursor_up();
                game.draw()?;
            }
            RawCommand::MoveDown => {
                game.cursor_down();
                game.draw()?;
            }
            RawCommand::MoveLeft => {
                game.cursor_left();
                game.draw()?;
            }
            RawCommand::MoveRight => {
                game.cursor_right();
                game.draw()?;
            }

            RawCommand::Interact => {
                break;
            }
            RawCommand::EndTurn => {
                break;
            }
            RawCommand::QuitGame => {
                game.state = GameState::Stalemate;
                break;
            }
            RawCommand::None => {}
        } }
    }
    Ok(())
}

fn player_turn(game: &mut Game) -> io::Result<()> {
    game.state = GameState::PlayerTurn;
    input_loop(game)?;
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
    io::stdin().is_terminal()
        && io::stdout().is_terminal()
        && crossterm::terminal::enable_raw_mode().is_ok_and(|_| {
            let _ = crossterm::terminal::disable_raw_mode();
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
