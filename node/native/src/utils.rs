use std::io;

use neon::prelude::*;
use neon::types::JsObject;

use game_scanner::prelude::{Game, GameCommands, GameState};

pub fn from_rust<'a>(ctx: &mut FunctionContext<'a>, game: &Game) -> Handle<'a, JsObject> {
    let game_object = JsObject::new(ctx);

    let _type = JsString::new(ctx, &game._type);
    game_object.set(ctx, "_type", _type).unwrap();

    let id = JsString::new(ctx, &game.id);
    game_object.set(ctx, "id", id).unwrap();

    let name = JsString::new(ctx, &game.name);
    game_object.set(ctx, "name", name).unwrap();

    let path = JsString::new(ctx, &game.path);
    game_object.set(ctx, "path", path).unwrap();

    // Commands
    let game_commands_object = JsObject::new(ctx);

    match &game.commands.install {
        Some(cmd) => {
            let value = make_array_of_string(ctx, cmd);
            game_commands_object.set(ctx, "install", value)
        }
        None => {
            let value = JsUndefined::new(ctx);
            game_commands_object.set(ctx, "install", value)
        }
    }
    .unwrap();

    let launch_command = make_array_of_string(ctx, &game.commands.launch);
    game_commands_object
        .set(ctx, "launch", launch_command)
        .unwrap();

    match &game.commands.uninstall {
        Some(cmd) => {
            let value = make_array_of_string(ctx, cmd);
            game_commands_object.set(ctx, "uninstall", value)
        }
        None => {
            let value = JsUndefined::new(ctx);
            game_commands_object.set(ctx, "uninstall", value)
        }
    }
    .unwrap();

    game_object
        .set(ctx, "commands", game_commands_object)
        .unwrap();

    // State
    let game_state_object = JsObject::new(ctx);

    let installed_state = JsBoolean::new(ctx, game.state.installed);
    game_state_object
        .set(ctx, "installed", installed_state)
        .unwrap();

    let needs_update_state = JsBoolean::new(ctx, game.state.needs_update);
    game_state_object
        .set(ctx, "needs_update", needs_update_state)
        .unwrap();

    let downloading_state = JsBoolean::new(ctx, game.state.downloading);
    game_state_object
        .set(ctx, "downloading", downloading_state)
        .unwrap();

    match game.state.total_bytes {
        Some(total_bytes) => {
            let value = JsNumber::new(ctx, total_bytes as f64);
            game_state_object.set(ctx, "total_bytes", value)
        }
        None => {
            let value = JsUndefined::new(ctx);
            game_state_object.set(ctx, "total_bytes", value)
        }
    }
    .unwrap();

    match game.state.received_bytes {
        Some(received_bytes) => {
            let value = JsNumber::new(ctx, received_bytes as f64);
            game_state_object.set(ctx, "received_bytes", value)
        }
        None => {
            let value = JsUndefined::new(ctx);
            game_state_object.set(ctx, "received_bytes", value)
        }
    }
    .unwrap();

    game_object.set(ctx, "state", game_state_object).unwrap();

    return game_object;
}

fn make_array_of_string<'a>(
    ctx: &mut FunctionContext<'a>,
    list: &Vec<String>,
) -> Handle<'a, JsArray> {
    let arr = JsArray::new(ctx, list.len() as u32);

    for (key, obj) in list.iter().enumerate() {
        let value = ctx.string(obj);
        arr.set(ctx, key as u32, value).unwrap();
    }

    return arr;
}

pub fn from_js<'a>(ctx: &mut FunctionContext<'a>, object: &JsObject) -> io::Result<Game> {
    let _type = object
        .get(ctx, "_type")
        .unwrap()
        .to_string(ctx)
        .unwrap()
        .value(ctx);
    let id = object
        .get(ctx, "id")
        .unwrap()
        .to_string(ctx)
        .unwrap()
        .value(ctx);
    let name = object
        .get(ctx, "name")
        .unwrap()
        .to_string(ctx)
        .unwrap()
        .value(ctx);
    let path = object
        .get(ctx, "path")
        .unwrap()
        .to_string(ctx)
        .unwrap()
        .value(ctx);

    let commands = object
        .get(ctx, "commands")
        .unwrap()
        .downcast::<JsObject, FunctionContext<'a>>(ctx)
        .unwrap();

    let install_command = commands
        .get(ctx, "install")
        .ok()
        .and_then(|value| value.downcast::<JsArray, FunctionContext<'a>>(ctx).ok())
        .and_then(|arr| arr.to_vec(ctx).ok())
        .map(|list| {
            list.iter()
                .map(|value| value.to_string(ctx).unwrap().value(ctx))
                .collect::<Vec<String>>()
        });

    let launch_command = commands
        .get(ctx, "launch")
        .unwrap()
        .downcast::<JsArray, FunctionContext<'a>>(ctx)
        .unwrap()
        .to_vec(ctx)
        .unwrap()
        .iter()
        .map(|value| value.to_string(ctx).unwrap().value(ctx))
        .collect::<Vec<String>>();

    let uninstall_command = commands
        .get(ctx, "uninstall")
        .ok()
        .and_then(|value| value.downcast::<JsArray, FunctionContext<'a>>(ctx).ok())
        .and_then(|arr| arr.to_vec(ctx).ok())
        .map(|list| {
            list.iter()
                .map(|value| value.to_string(ctx).unwrap().value(ctx))
                .collect::<Vec<String>>()
        });

    let state = object
        .get(ctx, "state")
        .unwrap()
        .downcast::<JsObject, FunctionContext<'a>>(ctx)
        .unwrap();

    let installed = state
        .get(ctx, "installed")
        .unwrap()
        .downcast::<JsBoolean, FunctionContext<'a>>(ctx)
        .unwrap()
        .value(ctx);

    let needs_update = state
        .get(ctx, "needs_update")
        .unwrap()
        .downcast::<JsBoolean, FunctionContext<'a>>(ctx)
        .unwrap()
        .value(ctx);

    let downloading = state
        .get(ctx, "downloading")
        .unwrap()
        .downcast::<JsBoolean, FunctionContext<'a>>(ctx)
        .unwrap()
        .value(ctx);

    let total_bytes = state
        .get(ctx, "total_bytes")
        .ok()
        .and_then(|value| value.downcast::<JsNumber, FunctionContext<'a>>(ctx).ok())
        .map(|number| number.value(ctx) as i64);

    let received_bytes = state
        .get(ctx, "received_bytes")
        .ok()
        .and_then(|value| value.downcast::<JsNumber, FunctionContext<'a>>(ctx).ok())
        .map(|number| number.value(ctx) as i64);

    return Ok(Game {
        _type,
        id,
        name,
        path,
        commands: GameCommands {
            install: install_command,
            launch: launch_command,
            uninstall: uninstall_command,
        },
        state: GameState {
            installed,
            needs_update,
            downloading,
            total_bytes,
            received_bytes,
        },
    });
}
