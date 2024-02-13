use std::{thread, time};

fn main() {
    let game = game_scanner::steam::games()
        .unwrap()
        .iter()
        .find(|app| app.state.installed)
        .cloned()
        .unwrap();

    game_scanner::manager::launch_game(&game).unwrap();

    thread::sleep(time::Duration::from_secs(30));

    game_scanner::manager::close_game(&game).unwrap();
}
