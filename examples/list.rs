use std::io::Error;

fn main() -> Result<(), Error> {
    let games = game_scanner::steam::games().unwrap();

    println!("{:#?}", games);

    Ok(())
}
