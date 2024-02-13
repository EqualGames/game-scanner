use std::any::type_name;

#[cfg(target_os = "windows")]
mod amazon {
    use super::*;

    #[test]
    fn executable() {
        let executable = game_scanner::amazon::executable().unwrap_or_default();

        assert_eq!(LAUNCHER_EXECUTABLE_PATH, type_of(&executable));
    }
}

#[cfg(not(target_os = "linux"))]
mod blizzard {
    use super::*;

    #[test]
    fn executable() {
        let executable = game_scanner::blizzard::executable().unwrap_or_default();

        assert_eq!(LAUNCHER_EXECUTABLE_PATH, type_of(&executable));
    }
}

mod epicgames {
    use super::*;

    #[test]
    fn executable() {
        let executable = game_scanner::epicgames::executable().unwrap_or_default();

        assert_eq!(LAUNCHER_EXECUTABLE_PATH, type_of(&executable));
    }
}

mod gog {
    use super::*;

    #[test]
    fn executable() {
        let executable = game_scanner::gog::executable().unwrap_or_default();

        assert_eq!(LAUNCHER_EXECUTABLE_PATH, type_of(&executable));
    }
}

mod origin {
    use super::*;

    #[test]
    fn executable() {
        let executable = game_scanner::origin::executable().unwrap_or_default();

        assert_eq!(LAUNCHER_EXECUTABLE_PATH, type_of(&executable));
    }
}

mod riotgames {
    use super::*;

    #[test]
    fn executable() {
        let executable = game_scanner::riotgames::executable().unwrap_or_default();

        assert_eq!(LAUNCHER_EXECUTABLE_PATH, type_of(&executable));
    }
}

mod steam {
    use super::*;

    #[test]
    fn executable() {
        let executable = game_scanner::steam::executable().unwrap_or_default();

        assert_eq!(LAUNCHER_EXECUTABLE_PATH, type_of(&executable));
    }
}

#[cfg(target_os = "windows")]
mod ubisoft {
    use super::*;

    #[test]
    fn executable() {
        let executable = game_scanner::ubisoft::executable().unwrap_or_default();

        assert_eq!(LAUNCHER_EXECUTABLE_PATH, type_of(&executable));
    }
}

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

const LAUNCHER_EXECUTABLE_PATH: &str = "&std::path::PathBuf";
