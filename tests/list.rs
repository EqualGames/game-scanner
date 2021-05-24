use std::any::type_name;
use std::io::Error;

mod amazon {
    use super::*;

    #[test]
    fn games() -> Result<(), Error> {
        let games = gamescanner::amazon::games().unwrap();

        assert_eq!(GAME_LIST_TYPE, type_of(&games));

        Ok(())
    }
}

mod blizzard {
    use super::*;

    #[test]
    fn games() -> Result<(), Error> {
        let games = gamescanner::blizzard::games().unwrap();

        assert_eq!(GAME_LIST_TYPE, type_of(&games));

        Ok(())
    }
}

mod epicgames {
    use super::*;

    #[test]
    fn games() -> Result<(), Error> {
        let games = gamescanner::epicgames::games().unwrap();

        assert_eq!(GAME_LIST_TYPE, type_of(&games));

        Ok(())
    }
}

mod gog {
    use super::*;

    #[test]
    fn games() -> Result<(), Error> {
        let games = gamescanner::gog::games().unwrap();

        assert_eq!(GAME_LIST_TYPE, type_of(&games));

        Ok(())
    }
}

mod origin {
    use super::*;

    #[test]
    fn games() -> Result<(), Error> {
        let games = gamescanner::origin::games().unwrap();

        assert_eq!(GAME_LIST_TYPE, type_of(&games));

        Ok(())
    }
}

mod riotgames {
    use super::*;

    #[test]
    fn games() -> Result<(), Error> {
        let games = gamescanner::riotgames::games().unwrap();

        assert_eq!(GAME_LIST_TYPE, type_of(&games));

        Ok(())
    }
}

mod steam {
    use super::*;

    #[test]
    fn games() -> Result<(), Error> {
        let games = gamescanner::steam::games().unwrap();

        assert_eq!(GAME_LIST_TYPE, type_of(&games));

        Ok(())
    }
}

mod ubisoft {
    use super::*;

    #[test]
    fn games() -> Result<(), Error> {
        let games = gamescanner::ubisoft::games().unwrap();

        assert_eq!(GAME_LIST_TYPE, type_of(&games));

        Ok(())
    }
}

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

const GAME_LIST_TYPE: &str = "&alloc::vec::Vec<gamescanner::prelude::Game>";
