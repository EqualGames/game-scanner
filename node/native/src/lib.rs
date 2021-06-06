use neon::prelude::*;

use crate::types::{amazon, blizzard, epicgames, gog, origin, riotgames, steam, ubisoft};

mod manager;
mod types;
mod utils;

#[neon::main]
fn main(mut context: ModuleContext) -> NeonResult<()> {
    manager::init(&mut context);
    amazon::init(&mut context);
    blizzard::init(&mut context);
    epicgames::init(&mut context);
    gog::init(&mut context);
    origin::init(&mut context);
    riotgames::init(&mut context);
    steam::init(&mut context);
    ubisoft::init(&mut context);

    Ok(())
}
