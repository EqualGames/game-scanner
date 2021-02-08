use gamescanner::prelude::Game;
use neon::prelude::*;
use neon::types::JsObject;
use std::io;

pub fn make_js_game<'a>(cx: &mut FunctionContext<'a>, game: &Game) -> Handle<'a, JsObject> {
    let js_obj = JsObject::new(cx);

    let _type = cx.string(&game._type);
    let id = cx.string(&game.id);
    let name = cx.string(&game.name);
    let path = cx.string(&game.path);
    let launch_command = JsArray::new(cx, game.launch_command.len() as u32);

    for (key, obj) in game.launch_command.iter().enumerate() {
        let value = cx.string(obj);
        launch_command.set(cx, key as u32, value).unwrap();
    }

    js_obj.set(cx, "_type", _type).unwrap();
    js_obj.set(cx, "id", id).unwrap();
    js_obj.set(cx, "name", name).unwrap();
    js_obj.set(cx, "path", path).unwrap();
    js_obj.set(cx, "launch_command", launch_command).unwrap();

    return js_obj;
}

pub fn from_js_game<'a>(cx: &mut FunctionContext<'a>, object: &JsObject) -> io::Result<Game> {
    let _type = object
        .get(cx, "_type")
        .unwrap()
        .to_string(cx)
        .unwrap()
        .value();
    let id = object.get(cx, "id").unwrap().to_string(cx).unwrap().value();
    let name = object
        .get(cx, "name")
        .unwrap()
        .to_string(cx)
        .unwrap()
        .value();
    let path = object
        .get(cx, "path")
        .unwrap()
        .to_string(cx)
        .unwrap()
        .value();

    let launch_command = object
        .get(cx, "launch_command")
        .unwrap()
        .downcast::<JsArray>()
        .unwrap_or(cx.empty_array())
        .to_vec(cx)
        .unwrap()
        .iter()
        .map(|value| value.to_string(cx).unwrap().value())
        .collect::<Vec<String>>();

    return Ok(Game {
        _type,
        id,
        name,
        path,
        launch_command,
    });
}
