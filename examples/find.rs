use game_scanner::steam;

fn main() {
    match steam::games().unwrap().first() {
        Some(game) => {
            match steam::find(&game.id) {
                Ok(game) => {
                    println!("{game:#?}");
                }
                Err(error) => {
                    eprintln!("{error:#?}");
                }
            };
        }
        None => {
            println!("Library is empty");
        }
    }
}
