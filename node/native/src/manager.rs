use neon::prelude::*;

use crate::utils::from_js;

pub fn init(context: &mut ModuleContext) {
    let manager = JsObject::new(context);

    let fn_install_game = JsFunction::new(context, install_game).unwrap();
    manager
        .set(context, "install_game", fn_install_game)
        .unwrap();

    let fn_uninstall_game = JsFunction::new(context, uninstall_game).unwrap();
    manager
        .set(context, "uninstall_game", fn_uninstall_game)
        .unwrap();

    let fn_launch_game = JsFunction::new(context, launch_game).unwrap();
    manager.set(context, "launch_game", fn_launch_game).unwrap();

    let fn_get_processes = JsFunction::new(context, get_processes).unwrap();
    manager
        .set(context, "get_processes", fn_get_processes)
        .unwrap();

    let fn_close_game = JsFunction::new(context, close_game).unwrap();
    manager.set(context, "close_game", fn_close_game).unwrap();

    context.export_value("manager", manager).unwrap();
}

fn install_game(mut context: FunctionContext) -> JsResult<JsUndefined> {
    let object = context.argument::<JsObject>(0).unwrap();

    let game = from_js(&mut context, &object).unwrap();

    game_scanner::manager::install_game(&game).unwrap();

    return Ok(context.undefined());
}

fn uninstall_game(mut context: FunctionContext) -> JsResult<JsUndefined> {
    let object = context.argument::<JsObject>(0).unwrap();

    let game = from_js(&mut context, &object).unwrap();

    game_scanner::manager::uninstall_game(&game).unwrap();

    return Ok(context.undefined());
}

fn launch_game(mut context: FunctionContext) -> JsResult<JsUndefined> {
    let object = context.argument::<JsObject>(0).unwrap();

    let game = from_js(&mut context, &object).unwrap();

    game_scanner::manager::launch_game(&game).unwrap();

    return Ok(context.undefined());
}

fn get_processes(mut context: FunctionContext) -> JsResult<JsValue> {
    let object = context.argument::<JsObject>(0).unwrap();

    let game = from_js(&mut context, &object).unwrap();

    let processes = game_scanner::manager::get_processes(&game);

    let result = match processes {
        Some(list) => {
            let arr = JsArray::new(&mut context, list.len() as u32);

            for (key, pid) in list.iter().enumerate() {
                let value = JsNumber::new(&mut context, *pid as f64);
                arr.set(&mut context, key as u32, value).unwrap();
            }

            arr.as_value(&mut context)
        }
        None => JsUndefined::new(&mut context).as_value(&mut context),
    };

    return Ok(result);
}

fn close_game(mut context: FunctionContext) -> JsResult<JsUndefined> {
    let object = context.argument::<JsObject>(0).unwrap();

    let game = from_js(&mut context, &object).unwrap();

    game_scanner::manager::close_game(&game).unwrap();

    return Ok(context.undefined());
}
