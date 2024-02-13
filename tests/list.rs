use std::any::type_name;

#[cfg(target_os = "windows")]
mod amazon {
    use super::*;

    #[test]
    fn games() {
        let games = game_scanner::amazon::games().unwrap_or_default();

        assert_eq!(GAME_LIST_RETURN_TYPE, type_of(&games));
    }
}

#[cfg(not(target_os = "linux"))]
mod blizzard {
    use super::*;

    #[test]
    fn games() {
        let games = game_scanner::blizzard::games().unwrap_or_default();

        assert_eq!(GAME_LIST_RETURN_TYPE, type_of(&games));
    }
}

mod epicgames {
    use super::*;

    #[test]
    fn games() {
        let games = game_scanner::epicgames::games().unwrap_or_default();

        assert_eq!(GAME_LIST_RETURN_TYPE, type_of(&games));
    }
}

mod gog {
    use super::*;

    #[test]
    fn games() {
        let games = game_scanner::gog::games().unwrap_or_default();

        assert_eq!(GAME_LIST_RETURN_TYPE, type_of(&games));
    }
}

mod origin {
    use super::*;

    #[test]
    fn games() {
        let games = game_scanner::origin::games().unwrap_or_default();

        assert_eq!(GAME_LIST_RETURN_TYPE, type_of(&games));
    }
}

mod riotgames {
    use super::*;

    #[test]
    fn games() {
        let games = game_scanner::riotgames::games().unwrap_or_default();

        assert_eq!(GAME_LIST_RETURN_TYPE, type_of(&games));
    }
}

mod steam {
    use super::*;

    #[test]
    fn games() {
        let games = game_scanner::steam::games().unwrap_or_default();

        assert_eq!(GAME_LIST_RETURN_TYPE, type_of(&games));
    }
}

#[cfg(target_os = "windows")]
mod ubisoft {
    use super::*;

    #[test]
    fn games() {
        let games = game_scanner::ubisoft::games().unwrap_or_default();

        assert_eq!(GAME_LIST_RETURN_TYPE, type_of(&games));
    }
}

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

const GAME_LIST_RETURN_TYPE: &str = "&alloc::vec::Vec<game_scanner::prelude::Game>";
