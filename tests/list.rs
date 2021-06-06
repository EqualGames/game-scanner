use std::any::type_name;
use std::io::Error;

#[cfg(target_os = "windows")]
mod amazon {
    use super::*;

    #[test]
    fn games() -> Result<(), Error> {
        let games = game_scanner::amazon::games().or(Ok(vec![])).unwrap();

        assert_eq!(GAME_LIST_RETURN_TYPE, type_of(&games));

        Ok(())
    }
}

#[cfg(target_os = "windows")]
mod blizzard {
    use super::*;

    #[test]
    fn games() -> Result<(), Error> {
        let games = game_scanner::blizzard::games().or(Ok(vec![])).unwrap();

        assert_eq!(GAME_LIST_RETURN_TYPE, type_of(&games));

        Ok(())
    }
}

mod epicgames {
    use super::*;

    #[test]
    fn games() -> Result<(), Error> {
        let games = game_scanner::epicgames::games().or(Ok(vec![])).unwrap();

        assert_eq!(GAME_LIST_RETURN_TYPE, type_of(&games));

        Ok(())
    }
}

mod gog {
    use super::*;

    #[test]
    fn games() -> Result<(), Error> {
        let games = game_scanner::gog::games().or(Ok(vec![])).unwrap();

        assert_eq!(GAME_LIST_RETURN_TYPE, type_of(&games));

        Ok(())
    }
}

mod origin {
    use super::*;

    #[test]
    fn games() -> Result<(), Error> {
        let games = game_scanner::origin::games().or(Ok(vec![])).unwrap();

        assert_eq!(GAME_LIST_RETURN_TYPE, type_of(&games));

        Ok(())
    }
}

mod riotgames {
    use super::*;

    #[test]
    fn games() -> Result<(), Error> {
        let games = game_scanner::riotgames::games().or(Ok(vec![])).unwrap();

        assert_eq!(GAME_LIST_RETURN_TYPE, type_of(&games));

        Ok(())
    }
}

mod steam {
    use super::*;

    #[test]
    fn games() -> Result<(), Error> {
        let games = game_scanner::steam::games().or(Ok(vec![])).unwrap();

        assert_eq!(GAME_LIST_RETURN_TYPE, type_of(&games));

        Ok(())
    }
}

#[cfg(target_os = "windows")]
mod ubisoft {
    use super::*;

    #[test]
    fn games() -> Result<(), Error> {
        let games = game_scanner::ubisoft::games().or(Ok(vec![])).unwrap();

        assert_eq!(GAME_LIST_RETURN_TYPE, type_of(&games));

        Ok(())
    }
}

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

const GAME_LIST_RETURN_TYPE: &str = "&alloc::vec::Vec<game_scanner::prelude::Game>";
