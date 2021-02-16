use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Game {
    pub _type: String,
    pub id: String,
    pub name: String,
    pub path: String,
    pub launch_command: Vec<String>,
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
