use std::{
    path::Path,
    process::{Command, Stdio},
};

use sysinfo::{Pid, System};

use crate::{
    error::{Error, ErrorKind, Result},
    prelude::Game,
};

/// # Errors
///
/// Will return `Err` if the game fails to install
pub fn install_game(game: &Game) -> Result<()> {
    let mut command = Command::new("");

    let Some(launch_command) = game.commands.install.as_ref() else {
        return Err(Error::new(
            ErrorKind::InvalidGame,
            "Error to install a game without install command",
        ));
    };

    for (index, arg) in launch_command.iter().enumerate() {
        if index == 0 {
            command = Command::new(arg);
        } else {
            command.arg(arg);
        }
    }

    if cfg!(debug_assertions) {
        println!("Executing the command: {command:?}");
    }

    let process = match command.stdin(Stdio::null()).stdout(Stdio::null()).spawn() {
        Ok(process) => process,
        Err(error) => {
            return Err(Error::new(
                ErrorKind::InstallFailed,
                format!("Couldn't install {}: {error}", game.name),
            ));
        }
    };

    if cfg!(debug_assertions) {
        println!("Installing {} [{}]", game.name, process.id());
    }

    Ok(())
}

/// # Errors
///
/// Will return `Err` if the game fails to uninstall
pub fn uninstall_game(game: &Game) -> Result<()> {
    let mut command = Command::new("");

    let Some(launch_command) = game.commands.install.as_ref() else {
        return Err(Error::new(
            ErrorKind::InvalidGame,
            "Error to install a game without install command",
        ));
    };

    for (index, arg) in launch_command.iter().enumerate() {
        if index == 0 {
            command = Command::new(arg);
        } else {
            command.arg(arg);
        }
    }

    if cfg!(debug_assertions) {
        println!("Executing the command: {command:?}");
    }

    let process = match command.stdin(Stdio::null()).stdout(Stdio::null()).spawn() {
        Ok(process) => process,
        Err(error) => {
            return Err(Error::new(
                ErrorKind::InstallFailed,
                format!("Couldn't uninstall {}: {error}", game.name),
            ));
        }
    };

    if cfg!(debug_assertions) {
        println!("Uninstalling {} [{}]", game.name, process.id());
    }

    Ok(())
}

/// # Errors
///
/// Will return `Err` if the game fails to launch
pub fn launch_game(game: &Game) -> Result<()> {
    let mut command = Command::new("");

    let Some(launch_command) = game.commands.install.as_ref() else {
        return Err(Error::new(
            ErrorKind::InvalidGame,
            "Error to install a game without install command",
        ));
    };

    for (index, arg) in launch_command.iter().enumerate() {
        if index == 0 {
            command = Command::new(arg);
        } else {
            command.arg(arg);
        }
    }

    if cfg!(debug_assertions) {
        println!("Executing the command: {command:?}");
    }

    let process = match command.stdin(Stdio::null()).stdout(Stdio::null()).spawn() {
        Ok(process) => process,
        Err(error) => {
            return Err(Error::new(
                ErrorKind::InstallFailed,
                format!("Couldn't launch {}: {error}", game.name),
            ));
        }
    };

    if cfg!(debug_assertions) {
        println!("Launching {} [{}]", game.name, process.id());
    }

    Ok(())
}

#[must_use]
pub fn get_processes(game: &Game) -> Option<Vec<u32>> {
    let sys = System::new_all();
    let processes = sys.processes();

    let str_array_contains = |path: &[String], value: &str| path.to_vec().join(" ").contains(value);
    let path_contains = |path: &Path, value: &str| path.display().to_string().contains(value);

    let mut list = Vec::new();

    let path = game.path.as_ref()?.display().to_string();

    for (pid, process) in processes {
        let should_kill = path_contains(process.cwd()?, &path)
            || path_contains(process.exe()?, &path)
            || str_array_contains(process.cmd(), &path);

        if !should_kill {
            continue;
        }

        list.push(pid.as_u32());
    }

    Some(list)
}

/// # Errors
///
/// Will return `Err` if the game fails to close
pub fn close_game(game: &Game) -> Result<()> {
    let processes = get_processes(game).ok_or_else(|| {
        Error::new(
            ErrorKind::GameProcessNotFound,
            format!("Could not found the process of {}", game.name),
        )
    })?;

    let sys = System::new_all();

    if cfg!(debug_assertions) {
        println!("Closing {}", game.name);
    }

    for pid in processes {
        match sys.process(Pid::from_u32(pid)) {
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

                process.kill();
            }
            None => {
                if cfg!(debug_assertions) {
                    println!("Could not kill the process: {pid}");
                }
            }
        }
    }

    Ok(())
}
