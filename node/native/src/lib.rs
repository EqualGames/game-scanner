use neon::prelude::*;

use crate::{
    manager::{close_game, launch_game},
    types::{amazon, blizzard, epicgames, gog, origin, riotgames, steam, ubisoft},
};

mod manager;
mod types;
mod utils;

#[neon::main]
fn main(mut ctx: ModuleContext) -> NeonResult<()> {
    let manager = JsObject::new(&mut ctx);

    let fn_launch_game = JsFunction::new(&mut ctx, launch_game).unwrap();
    manager
        .set(&mut ctx, "launch_game", fn_launch_game)
        .unwrap();

    let fn_close_game = JsFunction::new(&mut ctx, close_game).unwrap();
    manager.set(&mut ctx, "close_game", fn_close_game).unwrap();

    ctx.export_value("manager", manager).unwrap();

    amazon::init(&mut ctx);
    blizzard::init(&mut ctx);
    epicgames::init(&mut ctx);
    gog::init(&mut ctx);
    origin::init(&mut ctx);
    riotgames::init(&mut ctx);
    steam::init(&mut ctx);
    ubisoft::init(&mut ctx);

    Ok(())
}
