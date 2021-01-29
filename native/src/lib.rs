pub mod amazon;
pub mod epicgames;
pub mod gog;
pub mod origin;
pub mod types;
pub mod steam;
pub mod ubisoft;
mod util;

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
