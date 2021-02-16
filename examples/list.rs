fn main() -> gamescanner::error::Result<()> {
    let games = gamescanner::games();

    println!("{:#?}", games);

    Ok(())
}
