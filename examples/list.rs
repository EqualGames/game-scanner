use std::io;

fn main() -> io::Result<()> {
    let games = gamescanner::games();

    println!("{:#?}", games);

    Ok(())
}
