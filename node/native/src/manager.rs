use neon::prelude::*;

use crate::utils::from_js;

pub fn launch_game(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let object = cx.argument::<JsObject>(0).unwrap();

    let game = from_js(&mut cx, &object).unwrap();

    gamescanner::manager::launch_game(&game).unwrap();

    return Ok(cx.undefined());
}

pub fn close_game(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let object = cx.argument::<JsObject>(0).unwrap();

    let game = from_js(&mut cx, &object).unwrap();

    gamescanner::manager::close_game(&game).unwrap();

    return Ok(cx.undefined());
}
