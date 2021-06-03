use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Game {
    pub _type: String,
    pub id: String,
    pub name: String,
    pub path: Option<PathBuf>,
    pub commands: GameCommands,
    pub state: GameState,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct GameCommands {
    pub install: Option<Vec<String>>,
    pub launch: Option<Vec<String>>,
    pub uninstall: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct GameState {
    pub installed: bool,
    pub needs_update: bool,
    pub downloading: bool,
    pub total_bytes: Option<i64>,
    pub received_bytes: Option<i64>,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum GameType {
    AmazonGames,
    Blizzard,
    EpicGames,
    GOG,
    Origin,
    RiotGames,
    Steam,
    Ubisoft,
}

impl GameType {
    pub fn to_string(&self) -> String {
        match self {
            Self::AmazonGames => "amazongames",
            Self::Blizzard => "blizzard",
            Self::EpicGames => "epicgames",
            Self::GOG => "gog",
            Self::Origin => "origin",
            Self::RiotGames => "riotgames",
            Self::Steam => "steam",
            Self::Ubisoft => "ubisoft",
        }
        .to_string()
    }
}

impl From<String> for GameType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "amazongames" => Self::AmazonGames,
            "blizzard" => Self::Blizzard,
            "epicgames" => Self::EpicGames,
            "gog" => Self::GOG,
            "origin" => Self::Origin,
            "riotgames" => Self::RiotGames,
            "steam" => Self::Steam,
            "ubisoft" => Self::Ubisoft,
            _ => panic!("invalid game type"),
        }
    }
}
