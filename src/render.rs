use std::{process::exit, path::PathBuf};

use crate::colors::{TerminalColor, Color};
use crate::indicators::Indicators;
use crate::provider::{Information, InformationHandler};


#[derive(Copy, Clone, PartialEq)]
pub enum VerboseMode {
    Verbose1,
    Verbose2,
    Nothing,
}

#[derive(Copy, Clone)]
pub struct TerminalDisplay {
    pub terminal_color: TerminalColor,
    pub verbose_mode: VerboseMode,
}

impl TerminalDisplay {
    pub fn new(verbose_mode: VerboseMode) -> TerminalDisplay {
        let terminal_display = TerminalDisplay {
            terminal_color: TerminalColor::new(),
            verbose_mode,
        };
        terminal_display
    }

    pub fn render_git_ok(&mut self, git_output: String, path: &PathBuf, indicators: &Indicators,
                     project_state: &mut InformationHandler) -> () {
        let git_response: Vec<&str> = git_output.lines().collect();
        let mut indicator = String::new();

        if git_response.is_empty() {
            let err_msg = "Cannot read Git output. Maybe Git is not installed, or not in $PATH";
            eprintln!("{} - {}", path.parent().unwrap().to_str().unwrap(), err_msg);
            exit(1);
        }

        // Means: no unstaged, untracked, staged, tracked or new files are present
        let no_files = git_response.len() == 1;
        let not_ahead_and_not_behind = !git_response[0].contains("[ahead")
                                   && !git_response[0].contains("[behind");
        let repo_ok = no_files && not_ahead_and_not_behind;

        if self.verbose_mode == VerboseMode::Nothing && repo_ok {
            return;
        }

        indicator.push('[');

        if git_response[0].contains("[ahead") {
            let s = self.terminal_color.color(indicators.ahead, Color::Yellow);
            indicator.push_str(&s);
            project_state.set(Information::Warning);
        }

        if git_response[0].contains("[behind") {
            let s = self.terminal_color.color(indicators.behind, Color::Yellow);
            indicator.push_str(&s);
            project_state.set(Information::Warning);

        }

        if git_response.len() > 1 {
            let s = self.terminal_color.color(indicators.files, Color::Yellow);
            indicator.push_str(&s);
            project_state.set(Information::Warning);
        }

        if repo_ok {
            let s = self.terminal_color.color(indicators.ok, Color::Green);
            indicator.push_str(&s);
        }

        indicator.push(']');

        let path_variant = self.path_variant(path);
        println!("{} - {}", indicator, path_variant);
    }

    pub fn render_ok_msg(&self, msg: &str, indicators: &Indicators) -> () {
        let indicator_fmt = format!("{}", indicators.ok);
        let indicator = self.terminal_color.color(&indicator_fmt, Color::Green);
        // To stderr, because it's more a diagnostic information
        eprintln!("[{}] {}", indicator, msg);
    }

    pub fn render_err(&self, err_msg: &str, indicators: Option<&Indicators>, path: Option<&PathBuf>) -> () {
        let indicator = match indicators {
            Some(v) => {
                self.terminal_color.color(format!("{}", v.err).as_str(), Color::Red)
            },
            None => "".to_string(),
        };
        match path {
            Some(v) => {
                eprintln!("[{}] - {}\n └─■ Err: {}",
                        indicator, &v.parent().unwrap().to_str().unwrap(), err_msg);
            },
            None => {
                eprintln!("■ Err: {}", err_msg);
            },
        };
    }

    pub fn render_path_err(&self, err_msg: &str, indicators: &Indicators, path: &PathBuf) -> () {
        let indicator = self.terminal_color.color(format!("{}", indicators.err).as_str(), Color::Red);
        eprintln!("[{}] - {}\n └─■ Err: {}",
                        indicator, &path.to_str().unwrap(), err_msg);
    }


    fn path_variant(self, path: &PathBuf) -> &str {
        match self.verbose_mode {
            VerboseMode::Nothing => self.display_project_name(path),
            VerboseMode::Verbose1 => self.display_project_name(path),
            VerboseMode::Verbose2 => self.display_full_path(path),
        }
    }

    fn display_project_name(self, path: &PathBuf) -> &str {
        path.parent().unwrap().file_name().unwrap().to_str().unwrap()
    }

    fn display_full_path(self, path: &PathBuf) -> &str {
        path.parent().unwrap().to_str().unwrap()
    }
}
