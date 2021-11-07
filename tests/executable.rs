use std::{any::type_name, io::Error, path::PathBuf};

#[cfg(target_os = "windows")]
mod amazon {
    use super::*;

    #[test]
    fn executable() -> Result<(), Error> {
        let executable = game_scanner::amazon::executable()
            .or::<Error>(Ok(PathBuf::new()))
            .unwrap();

        assert_eq!(GAME_LIST_RETURN_TYPE, type_of(&executable));

        Ok(())
    }
}

#[cfg(target_os = "windows")]
mod blizzard {
    use super::*;

    #[test]
    fn executable() -> Result<(), Error> {
        let executable = game_scanner::blizzard::executable()
            .or::<Error>(Ok(PathBuf::new()))
            .unwrap();

        assert_eq!(GAME_LIST_RETURN_TYPE, type_of(&executable));

        Ok(())
    }
}

mod epicexecutable {
    use super::*;

    #[test]
    fn executable() -> Result<(), Error> {
        let executable = game_scanner::epicgames::executable()
            .or::<Error>(Ok(PathBuf::new()))
            .unwrap();

        assert_eq!(GAME_LIST_RETURN_TYPE, type_of(&executable));

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

        assert_eq!(GAME_LIST_RETURN_TYPE, type_of(&executable));

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

        assert_eq!(GAME_LIST_RETURN_TYPE, type_of(&executable));

        Ok(())
    }
}

mod riotexecutable {
    use super::*;

    #[test]
    fn executable() -> Result<(), Error> {
        let executable = game_scanner::riotgames::executable()
            .or::<Error>(Ok(PathBuf::new()))
            .unwrap();

        assert_eq!(GAME_LIST_RETURN_TYPE, type_of(&executable));

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

        assert_eq!(GAME_LIST_RETURN_TYPE, type_of(&executable));

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

        assert_eq!(GAME_LIST_RETURN_TYPE, type_of(&executable));

        Ok(())
    }
}

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

const GAME_LIST_RETURN_TYPE: &str = "&std::path::PathBuf";
