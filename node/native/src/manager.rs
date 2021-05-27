use neon::prelude::*;

use crate::utils::from_js;

pub fn launch_game(mut context: FunctionContext) -> JsResult<JsUndefined> {
    let object = context.argument::<JsObject>(0).unwrap();

    let game = from_js(&mut context, &object).unwrap();

    game_scanner::manager::launch_game(&game).unwrap();

    return Ok(context.undefined());
}

pub fn get_processes(mut context: FunctionContext) -> JsResult<JsValue> {
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

pub fn close_game(mut context: FunctionContext) -> JsResult<JsUndefined> {
    let object = context.argument::<JsObject>(0).unwrap();

    let game = from_js(&mut context, &object).unwrap();

    game_scanner::manager::close_game(&game).unwrap();

    return Ok(context.undefined());
}
