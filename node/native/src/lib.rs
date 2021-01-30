mod utils;

use crate::utils::make_js_game;
use libgamescan::*;
use neon::prelude::*;

fn games(mut cx: FunctionContext) -> JsResult<JsArray> {
    let mut games = Vec::new();

    match amazon::games::list() {
        Ok(items) => games.extend(items),
        Err(_e) => {}
    }

    match epicgames::games::list() {
        Ok(items) => games.extend(items),
        Err(_e) => {}
    }

    match gog::games::list() {
        Ok(items) => games.extend(items),
        Err(_e) => {}
    }

    match origin::games::list() {
        Ok(items) => games.extend(items),
        Err(_e) => {}
    }

    match steam::games::list() {
        Ok(items) => games.extend(items),
        Err(_e) => {}
    }

    match ubisoft::games::list() {
        Ok(items) => games.extend(items),
        Err(_e) => {}
    }

    let js_array = JsArray::new(&mut cx, games.len() as u32);

    for (i, game) in games.iter().enumerate() {
        let js_game = make_js_game(&mut cx, game);

        js_array
            .set(&mut cx, i as u32, js_game)
            .unwrap();
    }

    return Ok(js_array);
}


register_module!(mut m, { m.export_function("games", games) });
