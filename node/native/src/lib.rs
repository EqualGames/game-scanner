use neon::prelude::*;

#[cfg(target_os = "windows")]
use crate::types::{amazon, blizzard, ubisoft};
use crate::types::{epicgames, gog, origin, riotgames, steam};

mod manager;
mod types;
mod utils;

#[neon::main]
fn main(mut context: ModuleContext) -> NeonResult<()> {
    manager::init(&mut context);
    #[cfg(target_os = "windows")]
    amazon::init(&mut context);
    #[cfg(target_os = "windows")]
    blizzard::init(&mut context);
    epicgames::init(&mut context);
    gog::init(&mut context);
    origin::init(&mut context);
    riotgames::init(&mut context);
    steam::init(&mut context);
    #[cfg(target_os = "windows")]
    ubisoft::init(&mut context);

    Ok(())
}
