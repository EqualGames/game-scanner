fn main() {
    match game_scanner::steam::games() {
        Ok(value) => {
            println!("{value:#?}");
        }
        Err(error) => {
            eprintln!("{error:#?}");
        }
    }
}
