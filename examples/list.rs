use std::io::Error;

fn main() -> Result<(), Error> {
    let games = gamescanner::steam::games().unwrap();

    println!("{:#?}", games);

    Ok(())
}
