use std::{path::Path, process};

use sysinfo::{ProcessExt, System, SystemExt};

use crate::{
    error::{Error, ErrorKind, Result},
    prelude::Game,
};

pub fn launch_game(game: &Game) -> Result<()> {
    let mut command = process::Command::new("");

    for (index, arg) in game.commands.launch.iter().enumerate() {
        if index == 0 {
            command = process::Command::new(arg);
        } else {
            command.arg(arg);
        }
    }

    if cfg!(debug_assertions) {
        println!("Executing the command: {:?}", command);
    }

    let process = command
        .stdin(process::Stdio::null())
        .stdout(process::Stdio::null())
        .spawn()
        .expect(&format!("Couldn't run {}", game.name));

    if cfg!(debug_assertions) {
        println!("Launching {} [{}]", game.name, process.id());
    }

    Ok(())
}

pub fn get_processes(game: &Game) -> Option<Vec<usize>> {
    let sys = System::new_all();
    let processes = sys.get_processes();

    let str_array_contains =
        |path: &[String], value: &str| String::from(path.to_vec().join(" ")).contains(value);
    let path_contains = |path: &Path, value: &str| path.display().to_string().contains(value);

    let mut list = Vec::new();

    for (pid, process) in processes {
        let should_kill = path_contains(process.cwd(), &game.path)
            || path_contains(process.exe(), &game.path)
            || str_array_contains(process.cmd(), &game.path);

        if !should_kill {
            continue;
        }

        list.push(pid.clone());
    }

    Some(list)
}

pub fn close_game(game: &Game) -> Result<()> {
    let processes = get_processes(game)
        .ok_or(Error::new(
            ErrorKind::GameProcessNotFound,
            format!("Could not found the process of {}", game.name),
        ))
        .unwrap();

    let sys = System::new_all();

    if cfg!(debug_assertions) {
        println!("Closing {}", game.name);
    }

    for pid in processes {
        match sys.get_process(pid) {
            Some(process) => {
                if cfg!(debug_assertions) {
                    println!(
                        "Killing the process {:?} {:?} {:?} {:?}",
                        process.pid(),
                        process.name(),
                        process.exe(),
                        process.cwd()
                    );
                }

                process.kill(sysinfo::Signal::Quit);
            }
            None => {
                if cfg!(debug_assertions) {
                    println!("Could not kill the process: {}", pid);
                }
            }
        }
    }

    Ok(())
}
