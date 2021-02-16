use error::print_error;

use crate::prelude::Game;

mod amazon;
mod blizzard;
mod epicgames;
pub(crate) mod error;
mod gog;
pub mod manager;
mod origin;
pub mod prelude;
mod riotgames;
mod steam;
mod ubisoft;
mod util;

pub fn games() -> Vec<Game> {
    let mut games = Vec::<Game>::new();

    match amazon::games::list() {
        Ok(data) => games.extend(data),
        Err(error) => print_error(&error),
    }

    match blizzard::games::list() {
        Ok(data) => games.extend(data),
        Err(error) => print_error(&error),
    }

    match epicgames::games::list() {
        Ok(data) => games.extend(data),
        Err(error) => print_error(&error),
    }

    match gog::games::list() {
        Ok(data) => games.extend(data),
        Err(error) => print_error(&error),
    }

    match origin::games::list() {
        Ok(data) => games.extend(data),
        Err(error) => print_error(&error),
    }

    match riotgames::games::list() {
        Ok(data) => games.extend(data),
        Err(error) => print_error(&error),
    }

    match steam::games::list() {
        Ok(data) => games.extend(data),
        Err(error) => print_error(&error),
    }

    match ubisoft::games::list() {
        Ok(data) => games.extend(data),
        Err(error) => print_error(&error),
    }

    return games;
}
