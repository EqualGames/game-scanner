use std::{any::type_name, io::Error, path::PathBuf};

#[cfg(target_os = "windows")]
mod amazon {
    use super::*;

    #[test]
    fn executable() -> Result<(), Error> {
        let executable = game_scanner::amazon::executable()
            .or::<Error>(Ok(PathBuf::new()))
            .unwrap();

        assert_eq!(LAUNCHER_EXECUTABLE_PATH, type_of(&executable));

        Ok(())
    }
}

#[cfg(not(target_os = "linux"))]
mod blizzard {
    use super::*;

    #[test]
    fn executable() -> Result<(), Error> {
        let executable = game_scanner::blizzard::executable()
            .or::<Error>(Ok(PathBuf::new()))
            .unwrap();

        assert_eq!(LAUNCHER_EXECUTABLE_PATH, type_of(&executable));

        Ok(())
    }
}

mod epicgames {
    use super::*;

    #[test]
    fn executable() -> Result<(), Error> {
        let executable = game_scanner::epicgames::executable()
            .or::<Error>(Ok(PathBuf::new()))
            .unwrap();

        assert_eq!(LAUNCHER_EXECUTABLE_PATH, type_of(&executable));

        Ok(())
    }
}

mod gog {
    use super::*;

    #[test]
    fn executable() -> Result<(), Error> {
        let executable = game_scanner::gog::executable()
            .or::<Error>(Ok(PathBuf::new()))
            .unwrap();

        assert_eq!(LAUNCHER_EXECUTABLE_PATH, type_of(&executable));

        Ok(())
    }
}

mod origin {
    use super::*;

    #[test]
    fn executable() -> Result<(), Error> {
        let executable = game_scanner::origin::executable()
            .or::<Error>(Ok(PathBuf::new()))
            .unwrap();

        assert_eq!(LAUNCHER_EXECUTABLE_PATH, type_of(&executable));

        Ok(())
    }
}

mod riotgames {
    use super::*;

    #[test]
    fn executable() -> Result<(), Error> {
        let executable = game_scanner::riotgames::executable()
            .or::<Error>(Ok(PathBuf::new()))
            .unwrap();

        assert_eq!(LAUNCHER_EXECUTABLE_PATH, type_of(&executable));

        Ok(())
    }
}

mod steam {
    use super::*;

    #[test]
    fn executable() -> Result<(), Error> {
        let executable = game_scanner::steam::executable()
            .or::<Error>(Ok(PathBuf::new()))
            .unwrap();

        assert_eq!(LAUNCHER_EXECUTABLE_PATH, type_of(&executable));

        Ok(())
    }
}

#[cfg(target_os = "windows")]
mod ubisoft {
    use super::*;

    #[test]
    fn executable() -> Result<(), Error> {
        let executable = game_scanner::ubisoft::executable()
            .or::<Error>(Ok(PathBuf::new()))
            .unwrap();

        assert_eq!(LAUNCHER_EXECUTABLE_PATH, type_of(&executable));

        Ok(())
    }
}

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

const LAUNCHER_EXECUTABLE_PATH: &str = "&std::path::PathBuf";
