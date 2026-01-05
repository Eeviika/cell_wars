use crossterm::{
    cursor, execute,
    style::{Color, Print},
    terminal::{self},
};
use inquire::{self};
use rand::prelude::*;
use std::{
    fmt,
    io::{self, stdout},
};

use crate::enums::*;
use crate::shared::*;
use crate::structs::*;

impl Drop for GameGuard {
    fn drop(&mut self) {
        let _ = terminal::disable_raw_mode();
        let _ = execute!(stdout(), terminal::LeaveAlternateScreen, cursor::Show);
    }
}

impl GameGuard {
    pub fn new() -> Self {
        let _ = terminal::enable_raw_mode();
        let _ = execute!(stdout(), terminal::EnterAlternateScreen);
        return GameGuard {};
    }
}

impl fmt::Display for GameDifficulty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            GameDifficulty::Easy => "Easy",
            GameDifficulty::Standard => "Standard (*)",
            GameDifficulty::Hard => "Hard",
            GameDifficulty::NotEvenRemotelyFair => "Not Even Remotely Fair",
        };
        write!(f, "{}", text)
    }
}

impl fmt::Display for GameAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            GameAction::Produce => "Produce Resources",
            GameAction::UpgradeAttack => "Upgrade Combat Readiness Level",
            GameAction::UpgradeProduce => "Upgrade Production Level",
            GameAction::AttackCity(_, _) => "Attack City",
            GameAction::DestroyWall(_, _) => "Destroy Wall",
            GameAction::GenerateCity(_, _) => "Build New City",
        };
        write!(f, "{}", text)
    }
}

impl Cell {
    pub fn info(&self) -> String {
        if self.blocked {
            return "Wall.\r\n\t- Cannot build a city here.\r\n\t- Can be destroyed for 10 Resources.".into();
        }

        if self.city.is_none() {
            return "Empty tile.".into();
        }

        let city = self.city.unwrap();

        match city.state {
            CityState::Destroyed => {
                return
                    "A destroyed city.\r\n\t- It is, effectively, now just an obstacle.\r\n\t- Can be cleaned up for 5 Resources, turning it into an empty tile.\r\n\t- Probably has SOME use case...".into();
            }
            CityState::OwnedByPlayer => {
                return format!(
                    "Your city.\r\n\t- Productivity Level: {}\r\n\t- Combat Readiness Level: {}\r\n\t- Resources: {}\r\n\t- Overall Power: {}",
                    city.generation_level,
                    city.combat_level,
                    city.resources,
                    city.get_power()
                );
            }
            CityState::OwnedByComputer => {
                return "Enemy city.\r\n\t- Statistics unknown.".into();
            }
        }
    }
}

impl GameDifficulty {
    pub fn block_chance(self) -> f64 {
        match self {
            GameDifficulty::Easy => 0.05,
            GameDifficulty::Standard => 0.10,
            GameDifficulty::Hard => 0.25,
            GameDifficulty::NotEvenRemotelyFair => 0.30,
        }
    }

    pub fn starting_resources(self) -> u32 {
        match self {
            GameDifficulty::Easy => 10,
            GameDifficulty::Standard => 3,
            _ => 0,
        }
    }

    pub fn starting_enemy_level(self) -> u32 {
        match self {
            GameDifficulty::Hard => 2,
            GameDifficulty::NotEvenRemotelyFair => 5,
            _ => 1,
        }
    }

    pub fn starting_player_level(self) -> u32 {
        match self {
            GameDifficulty::Easy => 2,
            _ => 1,
        }
    }
}

impl Game {
    pub fn cursor_up(&mut self) {
        self.cursor_loc.1 = self.cursor_loc.1.saturating_sub(1);
    }

    pub fn cursor_left(&mut self) {
        self.cursor_loc.0 = self.cursor_loc.0.saturating_sub(1);
    }

    pub fn cursor_right(&mut self) {
        self.cursor_loc.0 = self.cursor_loc.0.saturating_add(1);
        if self.cursor_loc.0 > (GRID_SIZE - 1) as u16 {
            self.cursor_loc.0 -= 1;
        }
    }

    pub fn cursor_down(&mut self) {
        self.cursor_loc.1 = self.cursor_loc.1.saturating_add(1);
        if self.cursor_loc.1 > (GRID_SIZE - 1) as u16 {
            self.cursor_loc.1 -= 1;
        }
    }

    pub fn draw_cell_info(&self) -> io::Result<()> {
        let screen_y = ((GRID_Y_OFFSET + GRID_SIZE * 2) + 1) as u16;

        let x = self.cursor_loc.0 as usize;
        let y = self.cursor_loc.1 as usize;

        let Some(cell) = self.grid.get(y).and_then(|row| row.get(x)) else {
            return Ok(());
        };
        move_cursor(0, screen_y)?;

        execute!(stdout(), Print(cell.info()))?;
        Ok(())
    }

    pub fn draw_grid_text(&self) -> io::Result<()> {
        for (y, _) in self.grid.iter().enumerate() {
            let screen_y = (GRID_Y_OFFSET + y * 2) as u16;
            move_cursor(0, screen_y)?;
            print!("{}", y);
        }

        if let Some(first_row) = self.grid.first() {
            for (x, _) in first_row.iter().enumerate() {
                let screen_x = ((GRID_X_OFFSET + 1) + x * 3) as u16;
                move_cursor(screen_x, 0)?;
                print!("{}", x);
            }
        }

        Ok(())
    }

    pub fn draw_grid(&self) -> io::Result<()> {
        for (y, row) in self.grid.iter().enumerate() {
            let screen_y = (GRID_Y_OFFSET + y * 2) as u16;

            for (x, cell) in row.iter().enumerate() {
                let screen_x = (GRID_X_OFFSET + x * 3) as u16;
                move_cursor(screen_x, screen_y)?;

                let symbol = if cell.blocked {
                    " # "
                } else {
                    match cell.city.as_ref().map(|c| c.state) {
                        None => " . ",
                        Some(CityState::Destroyed) => " x ",
                        Some(CityState::OwnedByPlayer) => " P ",
                        Some(_) => " C ",
                    }
                };

                let color = if cell.blocked {
                    Color::Grey
                } else {
                    match cell.city.as_ref().map(|c| c.state) {
                        None => Color::Grey,
                        Some(CityState::Destroyed) => Color::DarkRed,
                        Some(CityState::OwnedByPlayer) => Color::Cyan,
                        Some(_) => Color::Red,
                    }
                };

                print_with_color(symbol, Color::Black, color)?;

                if self.cursor_loc == TerminalPos(x as u16, y as u16) {
                    move_cursor(screen_x, screen_y)?;
                    print_with_color("[", Color::Blue, Color::White)?;
                    move_cursor(screen_x + 2, screen_y)?;
                    print_with_color("]", Color::Blue, Color::White)?;
                }
            }
        }

        Ok(())
    }

    pub fn reset_grid(&mut self) {
        for y in 0..GRID_SIZE {
            for x in 0..GRID_SIZE {
                self.grid[y][x] = Cell::default();
            }
        }
    }

    pub fn generate_random_map(&mut self) {
        let mut rng = rand::rng();
        self.reset_grid();

        let player_position = random_coord(&mut rng);
        let enemy_position = loop {
            let p = random_coord(&mut rng);
            if p != player_position {
                break p;
            }
        };

        self.grid[player_position.1 as usize][player_position.0 as usize].city = Some(City {
            state: CityState::OwnedByPlayer,
            generation_level: self.difficulty.starting_player_level(),
            combat_level: self.difficulty.starting_player_level(),
            resources: self.difficulty.starting_resources(),
        });

        self.grid[enemy_position.1 as usize][enemy_position.0 as usize].city = Some(City {
            state: CityState::OwnedByComputer,
            generation_level: self.difficulty.starting_enemy_level(),
            combat_level: self.difficulty.starting_enemy_level(),
            resources: self.difficulty.starting_resources(),
        });

        let block_chance = self.difficulty.block_chance();

        for y in 0..GRID_SIZE {
            for x in 0..GRID_SIZE {
                let cell = &mut self.grid[y][x];

                if cell.city.is_some() {
                    continue;
                }

                if rng.random_bool(block_chance) {
                    cell.blocked = true;
                }
            }
        }

        self.cursor_loc = player_position;
    }

    pub fn prompt_difficulty(&mut self) {
        let menu = inquire::Select::new(
            "Choose a difficulty.",
            vec![
                GameDifficulty::Easy,
                GameDifficulty::Standard,
                GameDifficulty::Hard,
                GameDifficulty::NotEvenRemotelyFair,
            ],
        );
        let chosen = menu.prompt();
        self.difficulty = chosen.unwrap();
    }

    pub fn draw(&self) -> io::Result<()> {
        self.draw_grid()?;
        self.draw_grid_text()?;
        self.draw_cell_info()?;
        Ok(())
    }
}

impl City {
    pub fn roll_for_attack(&self) -> u32 {
        let mut rng = rand::rng();

        let min_roll: u32 = self.combat_level.saturating_sub(5) + 1;
        let max_roll: u32 = self.combat_level.saturating_add(1);

        rng.random_range(min_roll..=max_roll)
    }

    pub fn produce(&mut self) {
        self.resources += self.generation_level.div_ceil(2);
    }

    pub fn get_power(&self) -> u32 {
        let resources_bonus = self.resources.div_ceil(3);
        self.combat_level
            .saturating_add(self.generation_level)
            .saturating_add(resources_bonus)
            .div_ceil(3)
    }
}

impl Default for City {
    fn default() -> Self {
        City {
            state: CityState::OwnedByPlayer,
            generation_level: 1,
            combat_level: 1,
            resources: 0,
        }
    }
}
