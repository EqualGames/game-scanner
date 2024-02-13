#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Database {
    #[prost(message, repeated, tag = "1")]
    pub product_installs: ::prost::alloc::vec::Vec<ProductInstall>,
    #[prost(message, repeated, tag = "2")]
    pub active_installs: ::prost::alloc::vec::Vec<InstallHandshake>,
    #[prost(message, repeated, tag = "3")]
    pub active_processes: ::prost::alloc::vec::Vec<ActiveProcess>,
    #[prost(message, repeated, tag = "4")]
    pub product_configs: ::prost::alloc::vec::Vec<ProductConfig>,
    #[prost(message, optional, tag = "5")]
    pub download_settings: ::core::option::Option<DownloadSettings>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProductInstall {
    #[prost(string, tag = "1")]
    pub uid: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub product_code: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub settings: ::core::option::Option<UserSettings>,
    #[prost(message, optional, tag = "4")]
    pub cached_product_state: ::core::option::Option<CachedProductState>,
    #[prost(message, optional, tag = "5")]
    pub product_operations: ::core::option::Option<ProductOperations>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InstallHandshake {
    #[prost(string, tag = "1")]
    pub product: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub uid: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub settings: ::core::option::Option<UserSettings>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserSettings {
    #[prost(string, tag = "1")]
    pub install_path: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub play_region: ::prost::alloc::string::String,
    #[prost(enumeration = "ShortcutOption", tag = "3")]
    pub desktop_shortcut: i32,
    #[prost(enumeration = "ShortcutOption", tag = "4")]
    pub startmenu_shortcut: i32,
    #[prost(enumeration = "LanguageSettingType", tag = "5")]
    pub language_settings: i32,
    #[prost(string, tag = "6")]
    pub selected_text_language: ::prost::alloc::string::String,
    #[prost(string, tag = "7")]
    pub selected_speech_language: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "8")]
    pub languages: ::prost::alloc::vec::Vec<LanguageSetting>,
    #[prost(string, tag = "9")]
    pub gfx_override_tags: ::prost::alloc::string::String,
    #[prost(string, tag = "10")]
    pub versionbranch: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LanguageSetting {
    #[prost(string, tag = "1")]
    pub language: ::prost::alloc::string::String,
    #[prost(enumeration = "LanguageOption", tag = "2")]
    pub option: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BuildConfig {
    #[prost(string, tag = "1")]
    pub region: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub build_config: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BaseProductState {
    #[prost(bool, tag = "1")]
    pub installed: bool,
    #[prost(bool, tag = "2")]
    pub playable: bool,
    #[prost(bool, tag = "3")]
    pub update_complete: bool,
    #[prost(bool, tag = "4")]
    pub background_download_available: bool,
    #[prost(bool, tag = "5")]
    pub background_download_complete: bool,
    #[prost(string, tag = "6")]
    pub current_version: ::prost::alloc::string::String,
    #[prost(string, tag = "7")]
    pub current_version_str: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "8")]
    pub installed_build_config: ::prost::alloc::vec::Vec<BuildConfig>,
    #[prost(message, repeated, tag = "9")]
    pub background_download_build_config: ::prost::alloc::vec::Vec<BuildConfig>,
    #[prost(string, tag = "10")]
    pub decryption_key: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "11")]
    pub completed_install_actions: ::prost::alloc::vec::Vec<
        ::prost::alloc::string::String,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BackfillProgress {
    #[prost(double, tag = "1")]
    pub progress: f64,
    #[prost(bool, tag = "2")]
    pub backgrounddownload: bool,
    #[prost(bool, tag = "3")]
    pub paused: bool,
    #[prost(uint64, tag = "4")]
    pub download_limit: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RepairProgress {
    #[prost(double, tag = "1")]
    pub progress: f64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateProgress {
    #[prost(string, tag = "1")]
    pub last_disc_set_used: ::prost::alloc::string::String,
    #[prost(double, tag = "2")]
    pub progress: f64,
    #[prost(bool, tag = "3")]
    pub disc_ignored: bool,
    #[prost(uint64, tag = "4")]
    pub total_to_download: u64,
    #[prost(uint64, tag = "5")]
    pub download_remaining: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CachedProductState {
    #[prost(message, optional, tag = "1")]
    pub base_product_state: ::core::option::Option<BaseProductState>,
    #[prost(message, optional, tag = "2")]
    pub backfill_progress: ::core::option::Option<BackfillProgress>,
    #[prost(message, optional, tag = "3")]
    pub repair_progress: ::core::option::Option<RepairProgress>,
    #[prost(message, optional, tag = "4")]
    pub update_progress: ::core::option::Option<UpdateProgress>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProductOperations {
    #[prost(enumeration = "Operation", tag = "1")]
    pub active_operation: i32,
    #[prost(uint64, tag = "2")]
    pub priority: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProductConfig {
    #[prost(string, tag = "1")]
    pub product_code: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub metadata_hash: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub timestamp: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActiveProcess {
    #[prost(string, tag = "1")]
    pub process_name: ::prost::alloc::string::String,
    #[prost(int32, tag = "2")]
    pub pid: i32,
    #[prost(string, repeated, tag = "3")]
    pub uri: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DownloadSettings {
    #[prost(int32, tag = "1")]
    pub download_limit: i32,
    #[prost(int32, tag = "2")]
    pub backfill_limit: i32,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum LanguageOption {
    LangoptionNone = 0,
    LangoptionText = 1,
    LangoptionSpeech = 2,
    LangoptionTextAndSpeech = 3,
}
impl LanguageOption {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            LanguageOption::LangoptionNone => "LANGOPTION_NONE",
            LanguageOption::LangoptionText => "LANGOPTION_TEXT",
            LanguageOption::LangoptionSpeech => "LANGOPTION_SPEECH",
            LanguageOption::LangoptionTextAndSpeech => "LANGOPTION_TEXT_AND_SPEECH",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "LANGOPTION_NONE" => Some(Self::LangoptionNone),
            "LANGOPTION_TEXT" => Some(Self::LangoptionText),
            "LANGOPTION_SPEECH" => Some(Self::LangoptionSpeech),
            "LANGOPTION_TEXT_AND_SPEECH" => Some(Self::LangoptionTextAndSpeech),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum LanguageSettingType {
    LangsettingNone = 0,
    LangsettingSingle = 1,
    LangsettingSimple = 2,
    LangsettingAdvanced = 3,
}
impl LanguageSettingType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            LanguageSettingType::LangsettingNone => "LANGSETTING_NONE",
            LanguageSettingType::LangsettingSingle => "LANGSETTING_SINGLE",
            LanguageSettingType::LangsettingSimple => "LANGSETTING_SIMPLE",
            LanguageSettingType::LangsettingAdvanced => "LANGSETTING_ADVANCED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "LANGSETTING_NONE" => Some(Self::LangsettingNone),
            "LANGSETTING_SINGLE" => Some(Self::LangsettingSingle),
            "LANGSETTING_SIMPLE" => Some(Self::LangsettingSimple),
            "LANGSETTING_ADVANCED" => Some(Self::LangsettingAdvanced),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ShortcutOption {
    ShortcutNone = 0,
    ShortcutUser = 1,
    ShortcutAllUsers = 2,
}
impl ShortcutOption {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ShortcutOption::ShortcutNone => "SHORTCUT_NONE",
            ShortcutOption::ShortcutUser => "SHORTCUT_USER",
            ShortcutOption::ShortcutAllUsers => "SHORTCUT_ALL_USERS",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SHORTCUT_NONE" => Some(Self::ShortcutNone),
            "SHORTCUT_USER" => Some(Self::ShortcutUser),
            "SHORTCUT_ALL_USERS" => Some(Self::ShortcutAllUsers),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Operation {
    OpUpdate = 0,
    OpBackfill = 1,
    OpRepair = 2,
    OpNone = -1,
}
impl Operation {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Operation::OpUpdate => "OP_UPDATE",
            Operation::OpBackfill => "OP_BACKFILL",
            Operation::OpRepair => "OP_REPAIR",
            Operation::OpNone => "OP_NONE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "OP_UPDATE" => Some(Self::OpUpdate),
            "OP_BACKFILL" => Some(Self::OpBackfill),
            "OP_REPAIR" => Some(Self::OpRepair),
            "OP_NONE" => Some(Self::OpNone),
            _ => None,
        }
    }
}
