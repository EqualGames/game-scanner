use std::path::Path;
use std::process;

use sysinfo::{ProcessExt, System, SystemExt};

use crate::{error::Result, prelude::Game};

pub fn launch_game(app: &Game) -> Result<()> {
    let mut command = process::Command::new("");

    for (index, arg) in app.commands.launch.iter().enumerate() {
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
        .expect(&format!("Couldn't run {}", app.name));

    if cfg!(debug_assertions) {
        println!("Launching {} [{}]", app.name, process.id());
    }

    Ok(())
}

pub fn close_game(app: &Game) -> Result<()> {
    let sys = System::new_all();

    let str_array_contains =
        |path: &[String], value: &str| String::from(path.to_vec().join(" ")).contains(value);
    let path_contains = |path: &Path, value: &str| path.display().to_string().contains(value);

    // Find all processes related to launcher and game
    let processes = sys.get_processes();

    if cfg!(debug_assertions) {
        println!("Closing {}", app.name);
    }

    for (_pid, process) in processes {
        let should_kill = path_contains(process.cwd(), &app.path)
            || path_contains(process.exe(), &app.path)
            || str_array_contains(process.cmd(), &app.path);

        if !should_kill {
            continue;
        }

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

    Ok(())
}
