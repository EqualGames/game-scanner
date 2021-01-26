mod amazon;
mod epicgames;
mod gog;
mod origin;
mod scan;
mod steam;
mod ubisoft;
mod util;

use neon::prelude::*;

fn games(mut cx: FunctionContext) -> JsResult<JsArray> {
  let mut games = Vec::new();

  {
    let amazon_games = amazon::games().unwrap();
    games.extend(amazon_games);

    let epicgames_games = epicgames::games().unwrap();
    games.extend(epicgames_games);

    let gog_games = gog::games().unwrap();
    games.extend(gog_games);

    let origin_games = origin::games().unwrap();
    games.extend(origin_games);

    let steam_games = steam::games().unwrap();
    games.extend(steam_games);

    let ubisoft_games = ubisoft::games().unwrap();
    games.extend(ubisoft_games);
  }

  let js_array = JsArray::new(&mut cx, games.len() as u32);

  for (i, game) in games.iter().enumerate() {
    let js_obj = JsObject::new(&mut cx);

    let _type = cx.string(&game._type);
    let id = cx.string(&game.id);
    let name = cx.string(&game.name);
    let path = cx.string(&game.path);
    let launch_command = cx.string(&game.launch_command);

    js_obj.set(&mut cx, "_type", _type).unwrap();
    js_obj.set(&mut cx, "id", id).unwrap();
    js_obj.set(&mut cx, "name", name).unwrap();
    js_obj.set(&mut cx, "path", path).unwrap();
    js_obj.set(&mut cx, "launch_command", launch_command).unwrap();

    js_array
      .set(&mut cx, i as u32, js_obj)
      .unwrap();
  }

  return Ok(js_array);
}

register_module!(mut m, { m.export_function("games", games) });
