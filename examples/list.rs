fn main() {
    let games = game_scanner::origin::games();

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
