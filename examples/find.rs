fn main() {
    let game = gamescanner::steam::find("269210").unwrap();

    println!("{:#?}", game);
}
