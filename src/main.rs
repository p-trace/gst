/// GST traverses through Git projects, and displays their current `git status`.
/// Copyright (C) 2024  adam at p-trace.com  key.p-trace.com
///
/// This program is free software: you can redistribute it and/or modify
/// it under the terms of the GNU General Public License as published by
/// the Free Software Foundation, either version 3 of the License, or
/// (at your option) any later version.
///
/// This program is distributed in the hope that it will be useful,
/// but WITHOUT ANY WARRANTY; without even the implied warranty of
/// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
/// GNU General Public License for more details.
///
/// You should have received a copy of the GNU General Public License
/// along with this program.  If not, see <https://www.gnu.org/licenses/>.
use std::process::{exit, Command};
use clap::Parser;

mod config;
mod argparse;
mod colors;
mod indicators;
mod provider;
mod render;


const MAX_SEARCH_DEPTH: i8 = 30;
const MIN_SEARCH_DEPTH: i8 = 1;
const MAX_SEARCH_HEIGHT: i8 = 0;


fn main() -> () {
    check_git_client();

    let cli = argparse::Cli::parse();

    let mut config = config::Config::new();
    let mut state = ConfigStateHandler::new();

    check_config_version(&config);

    // args, that are writing into the config
    path(&cli, &mut config, &mut state);
    paths(&cli, &mut config, &mut state);
    remove_path(&cli, &mut config, &mut state);
    purge_paths(&cli, &mut config, &mut state);
    ascii_mode(&cli, &mut config, &mut state);
    search_depth(&cli, &mut config, &mut state);

    // write config and exit with 0
    match state.state {
        ConfigState::ConfigChange => {
            config.write_config();
        }
        ConfigState::NoConfigChange => ()
    }

    // args, that won't modify the config
    show_config(&cli, &mut config);  // exits with 0
    let verbose_mode = verbose(&cli);
    let execute_type = current_dir(&cli);

    provider::CheckGitProjects::init(&config, execute_type, verbose_mode, MAX_SEARCH_DEPTH, MAX_SEARCH_HEIGHT).scan();
}

#[derive(Debug)]
enum ConfigState {
    NoConfigChange,
    ConfigChange,
}

struct ConfigStateHandler {
    state: ConfigState,
}

impl ConfigStateHandler {
    fn new() -> ConfigStateHandler {
        let state = ConfigStateHandler {
            state: ConfigState::NoConfigChange,
        };
        state
    }

    fn config_change(&mut self) -> &mut Self {
        self.state = ConfigState::ConfigChange;
        self
    }
}


fn normalize_path_string(path: &String) -> String {
    #[cfg(target_family = "windows")]
    let pattern = ("/", "\\");

    #[cfg(target_family = "windows")]
    let s = path.replace(pattern.0, pattern.1);

    #[cfg(target_family = "unix")]
    let s = path.clone();

    s
}

fn normalize_path_str(path: &str) -> String {
    #[cfg(target_family = "windows")]
    let pattern = ("/", "\\");

    #[cfg(target_family = "windows")]
    let s = path.replace(pattern.0, pattern.1);

    #[cfg(target_family = "unix")]
    let s = path.to_string();

    s
}

fn path(cli: &argparse::Cli, config: &mut config::Config,
        state_handler: &mut ConfigStateHandler) -> () {
    if let Some(paths) = cli.path.as_deref() {
        for path in paths {
            config.add_path(&normalize_path_string(&path));
            state_handler.config_change();
        }
    }
}

fn paths(cli: &argparse::Cli, config: &mut config::Config,
         state_handler: &mut ConfigStateHandler) -> () {
    if let Some(paths) = cli.paths.as_deref() {
        let paths_buf: Vec<&str> = paths.split(' ').collect();
        for path in paths_buf {
            config.add_path(&normalize_path_str(path));
            state_handler.config_change();
        }
    }
}

fn remove_path(cli: &argparse::Cli, config: &mut config::Config,
               state_handler: &mut ConfigStateHandler) -> () {
    if let Some(remove_path) = cli.remove_path.as_deref() {
        config.remove_path(&normalize_path_str(remove_path));
        state_handler.config_change();
    }
}

fn purge_paths(cli: &argparse::Cli, config: &mut config::Config,
               state_handler: &mut ConfigStateHandler) -> () {
    match cli.purge_paths {
        true => {
            config.purge_paths();
            state_handler.config_change();
        }
        false => ()
    }
}

fn ascii_mode(cli: &argparse::Cli, config: &mut config::Config,
              state_handler: &mut ConfigStateHandler) -> () {
    match cli.ascii_mode.as_deref().unwrap_or("") {
        s if s.to_lowercase() == "true" => {
            config.ascii_enabled(true);
            state_handler.config_change();
        }
        s if s.to_lowercase() == "false" => {
            config.ascii_enabled(false);
            state_handler.config_change();
        },
        s if !s.is_empty() => {
            eprintln!("Unkown value. Try `true` or `false`");
            exit(1);
        }
        _ => ()
    }
}

fn search_depth(cli: &argparse::Cli, config: &mut config::Config,
                state_handler: &mut ConfigStateHandler) -> () {
    match cli.search_depth {
        Some(v) => {
            if (v < MIN_SEARCH_DEPTH) || (v > MAX_SEARCH_DEPTH) {
                eprintln!("Provided search depth is out of bounds. \
                          Please choose a number from {} to {}",
                          MIN_SEARCH_DEPTH,
                          MAX_SEARCH_DEPTH);
                exit(1);
            }
            config.search_depth(v);
            state_handler.config_change();
        }
        None => ()
    }
}

fn show_config(cli: &argparse::Cli, config: &mut config::Config) -> () {
    match cli.show_config {
        true => {
            config.show_config();
            exit(0);
        }
        false => ()
    }
}

fn verbose(cli: &argparse::Cli) -> render::VerboseMode {
    match cli.verbose {
        1 => {
            render::VerboseMode::Verbose1
        }
        2 => {
            render::VerboseMode::Verbose2
        }
        _ => render::VerboseMode::Nothing
    }
}

fn current_dir(cli: &argparse::Cli) -> provider::ExecuteType {
    match cli.current_dir {
        true => provider::ExecuteType::CurrentPath,
        false => provider::ExecuteType::FromConfig,
    }
}

fn check_git_client() -> () {
    let git = Command::new("git")
        .args(["--version"])
        .output()
        .unwrap();

    let r = String::from_utf8(git.stdout).unwrap().to_lowercase();

    if !r.contains("git") {
        eprintln!("Cannot find 'Git', make sure you have Git installed \
                 and/or Git in $PATH");
        exit(1);
    }
}

fn check_config_version(config: &config::Config) -> () {
    let current_version = option_env!("CARGO_PKG_VERSION").unwrap().to_string();
    let keyword = String::from("UNKNOWN");
    let version_from_config = &config.version;

    if *version_from_config == current_version {
        return;
    } else if *version_from_config == keyword {
        eprintln!("The configuration file in `$HOME/.config/gst/gst.json` contains an \
                  unknown version. Consider to delete the config and run this program \
                  again, to build a new one.");
        exit(1);
    } else {
        eprintln!("The configuration file in `$HOME/.config/gst/gst.json` is too old. \
                  Please remove it, and run this programm again.");
        exit(1);
    }
}
