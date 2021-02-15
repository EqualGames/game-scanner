pub enum BlizzardProducts {
    StarCraft,
    StarCraftII,
    WorldOfWarcraft,
    WorldOfWarcraftClassic,
    Overwatch,
    WarcraftIII,
    Hearthstone,
    HeroesOfTheStorm,
    DiabloIII,
    CallOfDutyBlackOps4,
    CallOfDutyModernWarfare,
    CallOfDutyMW2CampaignRemastered,
    CallOfDutyBlackOpsColdWar,
    Unknown,
}

impl BlizzardProducts {
    pub fn get_code(&self) -> &'static str {
        match self {
            Self::StarCraft => "S1",
            Self::StarCraftII => "S2",
            Self::WorldOfWarcraft => "WoW",
            Self::WorldOfWarcraftClassic => "WoW_wow_classic",
            Self::Overwatch => "Pro",
            Self::WarcraftIII => "W3",
            Self::Hearthstone => "WTCG",
            Self::HeroesOfTheStorm => "Hero",
            Self::DiabloIII => "D3",
            Self::CallOfDutyBlackOps4 => "VIPR",
            Self::CallOfDutyModernWarfare => "ODIN",
            Self::CallOfDutyMW2CampaignRemastered => "LAZR",
            Self::CallOfDutyBlackOpsColdWar => "ZEUS",
            Self::Unknown => "unknown",
        }
    }

    pub fn from_uid(uid: &str) -> BlizzardProducts {
        match uid {
            "s1" => Self::StarCraft,
            "s2" => Self::StarCraftII,
            "wow" => Self::WorldOfWarcraft,
            "wow_classic" => Self::WorldOfWarcraftClassic,
            "prometheus" => Self::Overwatch,
            "w3" => Self::WarcraftIII,
            "hs_beta" => Self::Hearthstone,
            "heroes" => Self::HeroesOfTheStorm,
            "diablo3" => Self::DiabloIII,
            "viper" => Self::CallOfDutyBlackOps4,
            "odin" => Self::CallOfDutyModernWarfare,
            "lazarus" => Self::CallOfDutyMW2CampaignRemastered,
            "zeus" => Self::CallOfDutyBlackOpsColdWar,
            _ => Self::Unknown,
        }
    }
}
