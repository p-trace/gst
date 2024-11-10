use std::{env, fs, path::PathBuf, process::{exit, Command}};
use crate::indicators::Indicators;
use crate::config::Config;
use crate::render::{VerboseMode, TerminalDisplay};


pub enum ExecuteType {
    FromConfig,
    CurrentPath
}

pub struct CheckGitProjects<'a> {
    pub config: &'a Config,
    pub execute_type: ExecuteType,
    pub indicators: Indicators,
    pub terminal_display: TerminalDisplay,
    pub project_state: InformationHandler,
    found_git_dir: Information,
    rec_counter: i8,
    max_depth: i8,
    max_height: i8,
}

impl <'a>CheckGitProjects<'a> {
    pub fn init(config: &Config, execute_type: ExecuteType, verbose_mode: VerboseMode, max_search_depth: i8,
                max_search_height: i8) -> CheckGitProjects {
        let check_git = CheckGitProjects {
            config: &config,
            execute_type,
            indicators: Indicators::new(config.ascii_mode),
            terminal_display: TerminalDisplay::new(verbose_mode),
            project_state: InformationHandler::new(),
            found_git_dir: Information::NotFoundGitDir,
            rec_counter: 1,

            max_depth: max_search_depth,
            max_height: max_search_height,
        };
        check_git
    }

    pub fn scan(&mut self) -> &Self {
        let projects: &Vec<PathBuf>  = match self.execute_type {
            ExecuteType::FromConfig => {
                &self.config.project_paths
            }
            ExecuteType::CurrentPath => {
                &vec![env::current_dir().unwrap()]
            }
        };

        if projects.is_empty() {
            eprintln!("No paths configured. Provide project paths via `-p, --path <PATH>`\n \
                      or `--paths \"<PATH> <PATH> <PATH>\"`");
            exit(1);
        }

        for project in projects {
            self.rec_counter = 1;
            self.found_git_dir = Information::NotFoundGitDir;
            self.traversal(&project);

            match self.found_git_dir {
                Information::NotFoundGitDir => {
                    let msg_cannot_find_git_dirs: &'static str = "Cannot find any `.git` directory";
                    self.terminal_display.render_path_err(&msg_cannot_find_git_dirs,
                                                     &self.indicators, &project)
                }
                Information::NotValidPath => {
                    let msg_no_dir: &'static str = "Is not a valid path";
                    self.terminal_display.render_path_err(&msg_no_dir.to_string(), &self.indicators, &project);
                }
                _ => ()
            }
        }

        match self.found_git_dir {
            Information::NotFoundGitDir => {
                return self;
            }
            Information::NotValidPath => {
                return self;
            }
            _ => ()
        }

        match self.project_state.info_state {
            Information::AllGreen => {
                let msg_projects_ok: &'static str = "All projects are up to date";
                self.terminal_display.render_ok_msg(&msg_projects_ok, &self.indicators)
            },
            _ => ()
        }
        self
    }

    /// path counts from 0
    /// Depth is inclusive
    /// TODO: doc
    fn traversal(&mut self, path: &PathBuf) -> () {
        if !path.is_dir() {
            self.found_git_dir = Information::NotValidPath;
            return;
        }

        let dir_content = match fs::read_dir(path) {
            Ok(v) => v,
            Err(e) => {
                self.terminal_display.render_err(&e.to_string(), Some(&self.indicators), Some(&path));
                return;
            }
        };

        for entry in dir_content {
            // In case I f*ed up the base case, the recursion depth is limited to both 
            // dimensions. Negative and positive.
            // Where `max_height` corresponds to the negative limit.
            if (self.rec_counter < self.max_height) || (self.rec_counter >= self.max_depth + 1) {
                break;
            }

            let entry = match entry {
                Ok(v) => v,
                Err(e) => {
                    self.terminal_display.render_err(&e.to_string(), Some(&self.indicators), Some(&path));
                    break;
                }
            };
            let path = entry.path();

            if path.ends_with(".git") {
                match self.git_status(&path) {
                    Ok(v) => {
                        self.terminal_display.render_git_ok(v.to_string(), &path, &self.indicators, &mut self.project_state);
                    }
                    Err(e) => {
                        self.terminal_display.render_err(&e.to_string(), Some(&self.indicators), Some(&path));
                    }
                };
                self.found_git_dir = Information::FoundGitDir;
            }

            if path.is_dir() {
                self.rec_counter += 1;
                self.traversal(&path);
            };
        }
        self.rec_counter -= 1;
    }

    fn git_status(&self, project_path: &PathBuf) -> Result<String, String> {
        let project_path = project_path.parent();
        let parent_path = match project_path {
            Some(v) => v.to_str().unwrap(),
            None => "",
        };

        if parent_path == "" {
            return Err("Could not determine path".to_string());
        }

        let git_output = Command::new("git")
            .args(["-C", parent_path, "status", "-b", "--porcelain"])
            .output()
            .unwrap();

        let ok = String::from_utf8(git_output.stdout).unwrap();
        let err = String::from_utf8(git_output.stderr).unwrap();

        if !err.is_empty() {
            return Err(err);
        }
        Ok(ok)
    }
}

pub enum Information {
    AllGreen,
    Warning,
    FoundGitDir,
    NotFoundGitDir,
    NotValidPath,
}

pub struct InformationHandler {
    pub info_state: Information,
}

impl InformationHandler {
    pub fn new() -> InformationHandler {
        let info_handler = InformationHandler {
            info_state: Information::AllGreen,
        };
        info_handler
    }

    pub fn set(&mut self, state: Information) -> &Self {
        self.info_state = state;
        self
    }
}
