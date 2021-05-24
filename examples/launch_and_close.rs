use std::io::Error;
use std::thread;
use std::time;

fn main() -> Result<(), Error> {
    let game = gamescanner::steam::games()
        .unwrap()
        .iter()
        .find(|app| app.state.installed)
        .map(|app| app.clone())
        .unwrap();

    gamescanner::manager::launch_game(&game).unwrap();

    thread::sleep(time::Duration::from_secs(30));

    gamescanner::manager::close_game(&game).unwrap();

    Ok(())
}
