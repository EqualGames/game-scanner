use std::io;

use libgamescanner::*;

fn main() -> io::Result<()> {
    let mut games = Vec::new();

    match amazon::games::list() {
        Ok(items) => games.extend(items),
        Err(_e) => {}
    }

    match epicgames::games::list() {
        Ok(items) => games.extend(items),
        Err(_e) => {}
    }

    match gog::games::list() {
        Ok(items) => games.extend(items),
        Err(_e) => {}
    }

    match origin::games::list() {
        Ok(items) => games.extend(items),
        Err(_e) => {}
    }

    match steam::games::list() {
        Ok(items) => games.extend(items),
        Err(_e) => {}
    }

    match ubisoft::games::list() {
        Ok(items) => games.extend(items),
        Err(_e) => {}
    }

    println!("{:#?}", games);

    Ok(())
}
