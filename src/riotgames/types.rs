pub enum RiotGamesProducts {
    LeagueOfLegendsLive,
    LegendsOfRuneterraLive,
    ValorantLive,
    Unknown,
}

impl RiotGamesProducts {
    pub fn from_manifest_name(manifest_name: &str) -> RiotGamesProducts {
        let manifest_info = manifest_name.split(".").collect::<Vec<&str>>();
        let code = manifest_info.get(0).unwrap().to_owned();
        let server = manifest_info.get(1).unwrap().to_owned();

        match code {
            "league_of_legends" => {
                if server.eq("live") {
                    Self::LeagueOfLegendsLive
                } else {
                    Self::Unknown
                }
            }
            "bacon" => {
                if server.eq("live") {
                    Self::LegendsOfRuneterraLive
                } else {
                    Self::Unknown
                }
            }
            "valorant" => {
                if server.eq("live") {
                    Self::ValorantLive
                } else {
                    Self::Unknown
                }
            }
            _ => Self::Unknown,
        }
    }

    pub fn get_code(&self) -> &'static str {
        match self {
            Self::LeagueOfLegendsLive => "league_of_legends",
            Self::LegendsOfRuneterraLive => "bacon",
            Self::ValorantLive => "valorant",
            Self::Unknown => "unknown",
        }
    }

    pub fn get_server(&self) -> &'static str {
        match self {
            Self::LeagueOfLegendsLive => "live",
            Self::LegendsOfRuneterraLive => "live",
            Self::ValorantLive => "live",
            Self::Unknown => "unknown",
        }
    }

    pub fn get_name(&self) -> &'static str {
        match self {
            Self::LeagueOfLegendsLive => "League Of Legends",
            Self::LegendsOfRuneterraLive => "Legends Of Runeterra",
            Self::ValorantLive => "Valorant",
            Self::Unknown => "Unknown",
        }
    }
}
