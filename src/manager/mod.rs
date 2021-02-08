use crate::prelude::Game;
use std::io;
use std::path::Path;
use std::process::{Command, Stdio};
use sysinfo::{ProcessExt, System, SystemExt};

pub fn launch_game(app: &Game) -> io::Result<()> {
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

    println!("Launching {} [{}]", app.name, process.id());

    Ok(())
}

pub fn close_game(app: &Game) -> io::Result<()> {
    let sys = System::new_all();

    let launcher_folder = match app._type.as_str() {
        "amazon" => "\\Amazon Games\\",
        "epicgames" => "\\Epic Games\\",
        "gog" => "\\GOG Galaxy\\",
        "origin" => "\\Origin\\",
        "steam" => "\\Steam\\",
        "ubisoft" => "\\Ubisoft\\",
        _ => panic!("Invalid game"),
    };

    let all_processes = sys.get_processes();

    let str_array_contains =
        |path: &[String], value: &str| String::from(path.to_vec().join(" ")).contains(value);
    let path_contains = |path: &Path, value: &str| path.display().to_string().contains(value);

    // Find all processes related to launcher and game
    let processes_to_kill = all_processes.clone().iter().filter(|&(_pid, process)| {
        path_contains(process.cwd(), launcher_folder)
            || path_contains(process.exe(), launcher_folder)
            || str_array_contains(process.cmd(), launcher_folder)
            || path_contains(process.cwd(), &app.path)
            || path_contains(process.exe(), &app.path)
            || str_array_contains(process.cmd(), &app.path)
    });

    // Find all process and your parents to kill
    let mut pid_list = Vec::new();
    processes_to_kill.for_each(|(pid, process)| {
        if std::process::id().to_string().contains(&pid.to_string()) {
            return;
        }

        pid_list.push(process.pid());

        match process.parent() {
            Some(parent_pid) => {
                if !std::process::id()
                    .to_string()
                    .contains(&parent_pid.to_string())
                {
                    pid_list.push(parent_pid)
                }
            }
            None => {}
        }
    });

    let processes = all_processes
        .clone()
        .iter()
        .filter(|&(pid, _process)| pid_list.contains(pid));

    println!("Closing {}", app.name);

    for (pid, process) in processes {
        if cfg!(debug_assertions) {
            println!(
                "killing {:?} {:?} {:?} {:?}",
                pid,
                process.name(),
                process.exe(),
                process.cwd()
            );
        }
        process.kill(sysinfo::Signal::Quit);
    }

    Ok(())
}
