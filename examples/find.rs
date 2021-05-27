fn main() {
    let game = game_scanner::steam::find("570").unwrap();

    println!("{:#?}", game);
}
