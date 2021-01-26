mod steam;
mod ubisoft;
mod epicgames;
mod origin;
mod util;
mod gog;
mod scan;
mod amazon;

use std::env;

fn main() {
  println!(
    "OS: {} ARCH: {} FAMILY: {}\n",
    env::consts::OS,
    env::consts::ARCH,
    env::consts::FAMILY,
  );


  println!("\n>> Steam:");
  let steam_games = steam::games();

  match steam_games {
    Ok(games) => {
      println!("|> Total: {}", games.len());

      for game in &games {
        println!("|- {:?}", game);
      }
    }
    Err(e) => println!("{:#?}", e),
  }

  println!("\n>> Ubisoft:");
  let ubisoft_games = ubisoft::games();

  match ubisoft_games {
    Ok(games) => {
      println!("|> Total: {}", games.len());

      for game in &games {
        println!("|- {:?}", game);
      }
    }
    Err(e) => println!("{:#?}", e),
  }

  println!("\n>> Epic Games:");
  let epicgames_games = epicgames::games();

  match epicgames_games {
    Ok(games) => {
      println!("|> Total: {}", games.len());

      for game in &games {
        println!("|- {:?}", game);
      }
    }
    Err(e) => println!("{:#?}", e),
  }

  println!("\n>> Origin:");
  let origin_games = origin::games();

  match origin_games {
    Ok(games) => {
      println!("|> Total: {}", games.len());

      for game in &games {
        println!("|- {:?}", game);
      }
    }
    Err(e) => println!("{:#?}", e),
  }

  println!("\n>> GOG:");
  let gog_games = gog::games();

  match gog_games {
    Ok(games) => {
      println!("|> Total: {}", games.len());

      for game in &games {
        println!("|- {:?}", game);
      }
    }
    Err(e) => println!("{:#?}", e),
  }

  println!("\n>> Amazon:");
  let amazon_games = amazon::games();

  match amazon_games {
    Ok(games) => {
      println!("|> Total: {}", games.len());

      for game in &games {
        println!("|- {:?}", game);
      }
    }
    Err(e) => println!("{:#?}", e),
  }
}
