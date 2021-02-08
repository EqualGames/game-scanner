use crate::prelude::Game;

mod amazon;
mod epicgames;
mod gog;
pub mod manager;
mod origin;
pub mod prelude;
mod steam;
mod ubisoft;
mod util;

pub fn games() -> Vec<Game> {
    let mut games = Vec::<Game>::new();

    match amazon::games::list() {
        Ok(data) => games.extend(data),
        Err(error) => println!("[Error] gamescanner::amazon::games \n{:#?}", error),
    }

    match epicgames::games::list() {
        Ok(data) => games.extend(data),
        Err(error) => println!("[Error] gamescanner::epicgames::games \n{:#?}", error),
    }

    match gog::games::list() {
        Ok(data) => games.extend(data),
        Err(error) => println!("[Error] gamescanner::gog::games \n{:#?}", error),
    }

    match origin::games::list() {
        Ok(data) => games.extend(data),
        Err(error) => println!("[Error] gamescanner::origin::games \n{:#?}", error),
    }

    match steam::games::list() {
        Ok(data) => games.extend(data),
        Err(error) => println!("[Error] gamescanner::steam::games \n{:#?}", error),
    }
    match ubisoft::games::list() {
        Ok(data) => games.extend(data),
        Err(error) => println!("[Error] gamescanner::ubisoft::games \n{:#?}", error),
    }

    return games;
}
