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
use std::io::{self, stdout};

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
    match event::read()? {
        event::Event::Key(key) => match key_to_command(key) {
            RawCommand::MoveUp => game.cursor_up(),
            RawCommand::MoveDown => game.cursor_down(),
            RawCommand::MoveLeft => game.cursor_left(),
            RawCommand::MoveRight => game.cursor_right(),

            RawCommand::Interact => {}
            RawCommand::EndTurn => {}
            RawCommand::QuitGame => game.state = GameState::Stalemate,
            RawCommand::None => {}
        },
        _ => {}
    }
    Ok(())
}

fn player_turn(game: &mut Game) -> io::Result<()> {
    input_loop(game)?;
    Ok(())
}

fn computer_turn(game: &mut Game) {}

fn main_game_loop(game: &mut Game) -> io::Result<()> {
    clear_screen()?;
    execute!(stdout(), cursor::Hide)?;
    game.draw()?;
    terminal::enable_raw_mode()?;
    while game.state != GameState::PlayerWon
        && game.state != GameState::ComputerWon
        && game.state != GameState::Stalemate
    {
        player_turn(game)?;
        computer_turn(game);
        clear_screen()?;
        game.draw()?;
    }
    Ok(())
}
// Main

fn main() -> io::Result<()> {
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
