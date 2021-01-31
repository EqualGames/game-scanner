use std::any::type_name;
use std::io;

use libgamescanner::*;

mod list_games {
    use super::*;

    #[test]
    fn amazon() -> io::Result<()> {
        let games = amazon::games::list();

        assert_eq!(&true, &games.is_ok());
        assert_eq!(GAME_LIST_RESULT, type_of(&games));
        assert_eq!(GAME_LIST_TYPE, type_of(&games.unwrap()));

        Ok(())
    }

    #[test]
    fn epicgames() -> io::Result<()> {
        let games = epicgames::games::list();

        assert_eq!(&true, &games.is_ok());
        assert_eq!(GAME_LIST_RESULT, type_of(&games));
        assert_eq!(GAME_LIST_TYPE, type_of(&games.unwrap()));

        Ok(())
    }

    #[test]
    fn gog() -> io::Result<()> {
        let games = gog::games::list();

        assert_eq!(&true, &games.is_ok());
        assert_eq!(GAME_LIST_RESULT, type_of(&games));
        assert_eq!(GAME_LIST_TYPE, type_of(&games.unwrap()));

        Ok(())
    }

    #[test]
    fn origin() -> io::Result<()> {
        let games = origin::games::list();

        assert_eq!(&true, &games.is_ok());
        assert_eq!(GAME_LIST_RESULT, type_of(&games));
        assert_eq!(GAME_LIST_TYPE, type_of(&games.unwrap()));

        Ok(())
    }

    #[test]
    fn steam() -> io::Result<()> {
        let games = steam::games::list();

        assert_eq!(&true, &games.is_ok());
        assert_eq!(GAME_LIST_RESULT, type_of(&games));
        assert_eq!(GAME_LIST_TYPE, type_of(&games.unwrap()));

        Ok(())
    }

    #[test]
    fn ubisoft() -> io::Result<()> {
        let games = ubisoft::games::list();

        assert_eq!(&true, &games.is_ok());
        assert_eq!(GAME_LIST_RESULT, type_of(&games));
        assert_eq!(GAME_LIST_TYPE, type_of(&games.unwrap()));

        Ok(())
    }
}

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

const GAME_LIST_RESULT: &str =
    "&core::result::Result<alloc::vec::Vec<libgamescanner::prelude::Game>, std::io::error::Error>";
const GAME_LIST_TYPE: &str = "&alloc::vec::Vec<libgamescanner::prelude::Game>";
