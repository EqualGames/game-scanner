use std::thread;
use std::time;

fn main() -> gamescanner::error::Result<()> {
    let games = gamescanner::games();

    let game = &games[0];

    gamescanner::manager::launch_game(game).unwrap();

    thread::sleep(time::Duration::from_secs(30));

    gamescanner::manager::close_game(game).unwrap();

    Ok(())
}
