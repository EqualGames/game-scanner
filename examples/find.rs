fn main() {
    let game = game_scanner::steam::find("269210").unwrap();

    println!("{:#?}", game);
}
