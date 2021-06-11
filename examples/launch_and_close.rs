use std::io::Error;
use std::thread;
use std::time;

fn main() -> Result<(), Error> {
    let game = game_scanner::origin::games()
        .unwrap()
        .iter()
        .find(|app| app.state.installed)
        .unwrap()
        .clone();

    game_scanner::manager::launch_game(&game).unwrap();

    thread::sleep(time::Duration::from_secs(30));

    game_scanner::manager::close_game(&game).unwrap();

    Ok(())
}
