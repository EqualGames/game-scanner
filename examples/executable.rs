fn main() {
    match game_scanner::steam::executable() {
        Ok(value) => {
            println!("{value:#?}");
        }
        Err(error) => {
            eprintln!("{error:#?}");
        }
    }
}
