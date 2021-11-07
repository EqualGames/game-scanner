use neon::prelude::*;

use game_scanner::ubisoft;

use crate::utils::from_rust;

pub fn init(context: &mut ModuleContext) {
    let launcher = JsObject::new(context);

    let fn_executable = JsFunction::new(context, executable).unwrap();
    launcher.set(context, "executable", fn_executable).unwrap();

    let fn_find = JsFunction::new(context, find).unwrap();
    launcher.set(context, "find", fn_find).unwrap();

    let fn_games = JsFunction::new(context, games).unwrap();
    launcher.set(context, "games", fn_games).unwrap();

    context.export_value("ubisoft", launcher).unwrap();
}

fn executable(mut context: FunctionContext) -> JsResult<JsValue> {
    match ubisoft::executable() {
        Ok(path) => {
            Ok(JsString::new(&mut context, path.display().to_string()).as_value(&mut context))
        }
        Err(_error) => Ok(JsUndefined::new(&mut context).as_value(&mut context)),
    }
}

fn find(mut context: FunctionContext) -> JsResult<JsValue> {
    let id: Handle<JsString> = context.argument(0).unwrap();
    let id = id.to_string(&mut context).unwrap().value(&mut context);

    let game = ubisoft::find(id.as_str());

    if game.is_err() {
        return Ok(JsUndefined::new(&mut context).as_value(&mut context));
    }

    return Ok(from_rust(&mut context, &game.unwrap()).as_value(&mut context));
}

fn games(mut context: FunctionContext) -> JsResult<JsArray> {
    let games = ubisoft::games().unwrap();

    let array = JsArray::new(&mut context, games.len() as u32);

    for (i, game) in games.iter().enumerate() {
        let item = from_rust(&mut context, game);

        array.set(&mut context, i as u32, item).unwrap();
    }

    return Ok(array);
}
