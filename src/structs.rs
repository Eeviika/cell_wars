use crate::enums::*;
use crate::shared::*;

pub struct GameGuard;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub struct TerminalPos(pub u16, pub u16);

#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub struct Game {
    pub grid: [[Cell; GRID_SIZE]; GRID_SIZE],
    pub difficulty: GameDifficulty,
    pub cursor_loc: TerminalPos,
    pub state: GameState,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub struct Cell {
    pub city: Option<City>,
    pub blocked: bool, // If blocked, there can be no City.
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct City {
    pub state: CityState,
    pub generation_level: u32,
    pub combat_level: u32,
    pub resources: u32,
}
