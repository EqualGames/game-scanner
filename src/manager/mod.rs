use crate::error::Result;
use crate::prelude::Game;
use std::path::Path;
use std::process::{Command, Stdio};
use sysinfo::{ProcessExt, System, SystemExt};

pub fn launch_game(app: &Game) -> Result<()> {
    let mut command: Command = Command::new("");

    for (index, arg) in app.launch_command.iter().enumerate() {
        if index == 0 {
            command = Command::new(arg);
        } else {
            command.arg(arg);
        }
    }

    let process = command
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .spawn()
        .expect(&format!("Couldn't run {}", app.name));

    if cfg!(debug_assertions) {
        println!("Launching {} [{}]", app.name, process.id());
    }

    Ok(())
}

pub fn close_game(app: &Game) -> Result<()> {
    let sys = System::new_all();

    let launcher_folder = match app._type.as_str() {
        "amazon" => "\\Amazon Games\\",
        "blizzard" => "\\Battle.net\\",
        "epicgames" => "\\Epic Games\\",
        "gog" => "\\GOG Galaxy\\",
        "origin" => "\\Origin\\",
        "riotgames" => "\\Riot Games\\",
        "steam" => "\\Steam\\",
        "ubisoft" => "\\Ubisoft\\",
        _ => panic!("Invalid game"),
    };

    let str_array_contains =
        |path: &[String], value: &str| String::from(path.to_vec().join(" ")).contains(value);
    let path_contains = |path: &Path, value: &str| path.display().to_string().contains(value);

    // Find all processes related to launcher and game
    let processes = sys.get_processes();

    if cfg!(debug_assertions) {
        println!("Closing {}", app.name);
    }

    for (_pid, process) in processes {
        if path_contains(process.cwd(), launcher_folder)
            || path_contains(process.exe(), launcher_folder)
            || str_array_contains(process.cmd(), launcher_folder)
            || path_contains(process.cwd(), &app.path)
            || path_contains(process.exe(), &app.path)
            || str_array_contains(process.cmd(), &app.path)
        {
            if cfg!(debug_assertions) {
                println!(
                    "killing {:?} {:?} {:?} {:?}",
                    process.pid(),
                    process.name(),
                    process.exe(),
                    process.cwd()
                );
            }

            process.kill(sysinfo::Signal::Quit);
        }
    }

    Ok(())
}
