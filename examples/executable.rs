fn main() {
    let executable = game_scanner::steam::executable();

    match executable {
        Ok(value) => {
            println!("{:#?}", value);
        }
        Err(error) => {
            println!("{:#?}", error);
        }
    }

    ()
}
