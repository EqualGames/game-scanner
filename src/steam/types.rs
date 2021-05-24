#[allow(dead_code)]
pub enum SteamAppState {
    Invalid,
    Uninstalled,
    UpdateRequired,
    FullyInstalled,
    Encrypted,
    Locked,
    FilesMissing,
    AppRunning,
    FilesCorrupt,
    UpdateRunning,
    UpdatePaused,
    UpdateStarted,
    Uninstalling,
    BackupRunning,
    Reconfiguring,
    Validating,
    AddingFiles,
    PreAllocating,
    Downloading,
    Staging,
    Committing,
    UpdateStopping,
}

impl SteamAppState {
    pub fn get_code(&self) -> i32 {
        match self {
            Self::Invalid => 0,
            Self::Uninstalled => 1,
            Self::UpdateRequired => 1 << 1,
            Self::FullyInstalled => 1 << 2,
            Self::Encrypted => 1 << 3,
            Self::Locked => 1 << 4,
            Self::FilesMissing => 1 << 5,
            Self::AppRunning => 1 << 6,
            Self::FilesCorrupt => 1 << 7,
            Self::UpdateRunning => 1 << 8,
            Self::UpdatePaused => 1 << 9,
            Self::UpdateStarted => 1 << 10,
            Self::Uninstalling => 1 << 11,
            Self::BackupRunning => 1 << 12,
            Self::Reconfiguring => 1 << 16,
            Self::Validating => 1 << 17,
            Self::AddingFiles => 1 << 18,
            Self::PreAllocating => 1 << 19,
            Self::Downloading => 1 << 20,
            Self::Staging => 1 << 21,
            Self::Committing => 1 << 22,
            Self::UpdateStopping => 1 << 23,
        }
    }
}

#[allow(dead_code)]
pub enum SteamUpdateResult {
    Downloading,
    Paused,
    InQueue,
}

impl SteamUpdateResult {
    #[allow(unused)]
    pub fn get_code(&self) -> i32 {
        match self {
            Self::Downloading => 0,
            Self::Paused => 1 << 1,
            Self::InQueue => 1 << 2,
        }
    }
}
