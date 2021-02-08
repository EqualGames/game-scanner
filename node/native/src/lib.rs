use crate::utils::{from_js_game, make_js_game};
use neon::prelude::*;

mod utils;

fn games(mut cx: FunctionContext) -> JsResult<JsArray> {
    let games = gamescanner::games();

    let js_array = JsArray::new(&mut cx, games.len() as u32);

    for (i, game) in games.iter().enumerate() {
        let js_game = make_js_game(&mut cx, game);

        js_array.set(&mut cx, i as u32, js_game).unwrap();
    }

    return Ok(js_array);
}

fn launch_game(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let object = cx.argument::<JsObject>(0).unwrap();

    let game = from_js_game(&mut cx, &object).unwrap();

    gamescanner::manager::launch_game(&game).unwrap();

    return Ok(cx.undefined());
}

fn close_game(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let object = cx.argument::<JsObject>(0).unwrap();

    let game = from_js_game(&mut cx, &object).unwrap();

    gamescanner::manager::close_game(&game).unwrap();

    return Ok(cx.undefined());
}

register_module!(mut m, {
    m.export_function("games", games)?;
    m.export_function("launch_game", launch_game)?;
    m.export_function("close_game", close_game)?;

    Ok(())
});
