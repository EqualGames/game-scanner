use std::any::type_name;
use std::io::Error;

mod amazon {
    use super::*;

    #[test]
    fn games() -> Result<(), Error> {
        let games = game_scanner::amazon::games().unwrap();

        assert_eq!(GAME_LIST_TYPE, type_of(&games));

        Ok(())
    }
}

mod blizzard {
    use super::*;

    #[test]
    fn games() -> Result<(), Error> {
        let games = game_scanner::blizzard::games().unwrap();

        assert_eq!(GAME_LIST_TYPE, type_of(&games));

        Ok(())
    }
}

mod epicgames {
    use super::*;

    #[test]
    fn games() -> Result<(), Error> {
        let games = game_scanner::epicgames::games().unwrap();

        assert_eq!(GAME_LIST_TYPE, type_of(&games));

        Ok(())
    }
}

mod gog {
    use super::*;

    #[test]
    fn games() -> Result<(), Error> {
        let games = game_scanner::gog::games().unwrap();

        assert_eq!(GAME_LIST_TYPE, type_of(&games));

        Ok(())
    }
}

mod origin {
    use super::*;

    #[test]
    fn games() -> Result<(), Error> {
        let games = game_scanner::origin::games().unwrap();

        assert_eq!(GAME_LIST_TYPE, type_of(&games));

        Ok(())
    }
}

mod riotgames {
    use super::*;

    #[test]
    fn games() -> Result<(), Error> {
        let games = game_scanner::riotgames::games().unwrap();

        assert_eq!(GAME_LIST_TYPE, type_of(&games));

        Ok(())
    }
}

mod steam {
    use super::*;

    #[test]
    fn games() -> Result<(), Error> {
        let games = game_scanner::steam::games().unwrap();

        assert_eq!(GAME_LIST_TYPE, type_of(&games));

        Ok(())
    }
}

mod ubisoft {
    use super::*;

    #[test]
    fn games() -> Result<(), Error> {
        let games = game_scanner::ubisoft::games().unwrap();

        assert_eq!(GAME_LIST_TYPE, type_of(&games));

        Ok(())
    }
}

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

const GAME_LIST_TYPE: &str = "&alloc::vec::Vec<game_scanner::prelude::Game>";
