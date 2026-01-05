#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CityState {
    OwnedByPlayer,
    OwnedByComputer,
    Destroyed,
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
    NotValidAction,
    NotEnoughResources,
    NoTarget,
    BadTarget,
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
    Produce,
    UpgradeAttack,
    UpgradeProduce,
    DestroyWall(usize, usize),
    AttackCity(usize, usize),
    GenerateCity(usize, usize),
}
