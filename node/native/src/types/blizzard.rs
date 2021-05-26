use neon::prelude::*;

use game_scanner::blizzard;

use crate::utils::from_rust;

pub fn init(ctx: &mut ModuleContext) {
    let launcher = JsObject::new(ctx);

    let fn_games = JsFunction::new(ctx, games).unwrap();
    launcher.set(ctx, "games", fn_games).unwrap();

    ctx.export_value("blizzard", launcher).unwrap();
}

fn games(mut cx: FunctionContext) -> JsResult<JsArray> {
    let games = blizzard::games().unwrap();

    let array = JsArray::new(&mut cx, games.len() as u32);

    for (i, game) in games.iter().enumerate() {
        let item = from_rust(&mut cx, game);

        array.set(&mut cx, i as u32, item).unwrap();
    }

    return Ok(array);
}
