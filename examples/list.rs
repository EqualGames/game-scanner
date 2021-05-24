use std::io::Error;

fn main() -> Result<(), Error> {
    let games = gamescanner::amazon::games().unwrap();

    println!("{:#?}", games);

    Ok(())
}
