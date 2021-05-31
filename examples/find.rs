use game_scanner::steam;

fn main() {
    let games = steam::games().unwrap();

    match games.get(0) {
        Some(game) => {
            match steam::find(&game.id) {
                Ok(game) => {
                    println!("{:#?}", game);
                }
                Err(error) => {
                    println!("{:#?}", error);
                }
            };
        }
        None => {
            println!("Library is empty");
        }
    }

    ()
}
