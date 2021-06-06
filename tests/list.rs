use std::any::type_name;
use std::io::Error;

#[cfg(target_os = "windows")]
mod amazon {
    use super::*;

    #[test]
    fn games() -> Result<(), Error> {
        let games = game_scanner::amazon::games();

        assert_eq!(GAME_LIST_RETURN_TYPE, type_of(&games));

        Ok(())
    }
}

#[cfg(target_os = "windows")]
mod blizzard {
    use super::*;

    #[test]
    fn games() -> Result<(), Error> {
        let games = game_scanner::blizzard::games();

        assert_eq!(GAME_LIST_RETURN_TYPE, type_of(&games));

        Ok(())
    }
}

mod epicgames {
    use super::*;

    #[test]
    fn games() -> Result<(), Error> {
        let games = game_scanner::epicgames::games();

        assert_eq!(GAME_LIST_RETURN_TYPE, type_of(&games));

        Ok(())
    }
}

mod gog {
    use super::*;

    #[test]
    fn games() -> Result<(), Error> {
        let games = game_scanner::gog::games();

        assert_eq!(GAME_LIST_RETURN_TYPE, type_of(&games));

        Ok(())
    }
}

mod origin {
    use super::*;

    #[test]
    fn games() -> Result<(), Error> {
        let games = game_scanner::origin::games();

        assert_eq!(GAME_LIST_RETURN_TYPE, type_of(&games));

        Ok(())
    }
}

mod riotgames {
    use super::*;

    #[test]
    fn games() -> Result<(), Error> {
        let games = game_scanner::riotgames::games();

        assert_eq!(GAME_LIST_RETURN_TYPE, type_of(&games));

        Ok(())
    }
}

mod steam {
    use super::*;

    #[test]
    fn games() -> Result<(), Error> {
        let games = game_scanner::steam::games();

        assert_eq!(GAME_LIST_RETURN_TYPE, type_of(&games));

        Ok(())
    }
}

#[cfg(target_os = "windows")]
mod ubisoft {
    use super::*;

    #[test]
    fn games() -> Result<(), Error> {
        let games = game_scanner::ubisoft::games();

        assert_eq!(GAME_LIST_RETURN_TYPE, type_of(&games));

        Ok(())
    }
}

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

const GAME_LIST_RETURN_TYPE: &str = "&core::result::Result<alloc::vec::Vec<game_scanner::prelude::Game>, game_scanner::error::Error>";
