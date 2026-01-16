use crate::structs::TerminalPos;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CityState {
    OwnedByPlayer,
    OwnedByComputer,
    Destroyed,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub enum InputOutcome {
    #[default]
    None,
    Redraw,
    Interact,
    EndTurn,
    QuitGame,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub enum GameState {
    #[default]
    Setup,
    PlayerTurn,
    ComputerTurn,
    PlayerWon,
    ComputerWon,
    Stalemate,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub enum GameDifficulty {
    Easy,
    #[default]
    Standard,
    Hard,
    NotEvenRemotelyFair,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum GameError {
    NotValidPosition,
    NoCityAtTarget,
    NoCityAtSource,
    NoWallAtTarget,
    TargetIsOccupied,
    TargetIsSource,
    NotEnoughResources,
    IO,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RawCommand {
    MoveLeft,
    MoveRight,
    MoveUp,
    MoveDown,
    Interact,
    EndTurn,
    QuitGame,
    None,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum GameAction {
    Produce {
        source: TerminalPos,
    },
    UpgradeAttack {
        source: TerminalPos,
    },
    UpgradeProduce {
        source: TerminalPos,
    },
    DestroyWall {
        source: TerminalPos,
        target: TerminalPos,
    },
    AttackCity {
        source: TerminalPos,
        target: TerminalPos,
    },
    GenerateCity {
        source: TerminalPos,
        target: TerminalPos,
    },
}
