fn main() {
    let games = game_scanner::steam::games();

    match games {
        Ok(value) => {
            println!("{:#?}", value);
        }
        Err(error) => {
            println!("{:#?}", error);
        }
    }

    ()
}
