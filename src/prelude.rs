use std::{
    fmt::{Display, Formatter, Result as FmtResult},
    path::PathBuf,
};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Game {
    pub type_:    String,
    pub id:       String,
    pub name:     String,
    pub path:     Option<PathBuf>,
    pub commands: GameCommands,
    pub state:    GameState,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct GameCommands {
    pub install:   Option<Vec<String>>,
    pub launch:    Option<Vec<String>>,
    pub uninstall: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct GameState {
    pub installed:      bool,
    pub needs_update:   bool,
    pub downloading:    bool,
    pub total_bytes:    Option<u64>,
    pub received_bytes: Option<u64>,
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

impl Display for GameType {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let name = match self {
            Self::AmazonGames => "amazongames",
            Self::Blizzard => "blizzard",
            Self::EpicGames => "epicgames",
            Self::GOG => "gog",
            Self::Origin => "origin",
            Self::RiotGames => "riotgames",
            Self::Steam => "steam",
            Self::Ubisoft => "ubisoft",
        };

        write!(f, "{name}")
    }
}

// This could also be From<String> for Option<GameType>
impl TryFrom<String> for GameType {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "amazongames" => Ok(Self::AmazonGames),
            "blizzard" => Ok(Self::Blizzard),
            "epicgames" => Ok(Self::EpicGames),
            "gog" => Ok(Self::GOG),
            "origin" => Ok(Self::Origin),
            "riotgames" => Ok(Self::RiotGames),
            "steam" => Ok(Self::Steam),
            "ubisoft" => Ok(Self::Ubisoft),
            _ => Err("invalid game type"),
        }
    }
}
