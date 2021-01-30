use libgamescan::types::Game;
use neon::prelude::*;
use neon::types::JsObject;

pub fn make_js_game<'a>(cx: &mut FunctionContext<'a>, game: &Game) -> Handle<'a, JsObject> {
    let js_obj = JsObject::new(cx);

    let _type = cx.string(&game._type);
    let id = cx.string(&game.id);
    let name = cx.string(&game.name);
    let path = cx.string(&game.path);
    let launch_command = cx.string(&game.launch_command);

    js_obj.set(cx, "_type", _type).unwrap();
    js_obj.set(cx, "id", id).unwrap();
    js_obj.set(cx, "name", name).unwrap();
    js_obj.set(cx, "path", path).unwrap();
    js_obj.set(cx, "launch_command", launch_command).unwrap();

    return js_obj;
}