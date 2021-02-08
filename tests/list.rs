use std::any::type_name;
use std::io;

mod list_games {
    use super::*;

    #[test]
    fn get_games() -> io::Result<()> {
        let games = gamescanner::games();

        assert_eq!(GAME_LIST_TYPE, type_of(&games));

        Ok(())
    }
}

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

const GAME_LIST_TYPE: &str = "&alloc::vec::Vec<gamescanner::prelude::Game>";
