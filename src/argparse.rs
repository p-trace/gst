use clap::{ArgAction, Parser};


const ABOUT_CLI: &str = "Screens your Git projects for unstaged, untracked files \
                        and commits ahead/behind.\n\
                        Per default, it only displays projects with unstaged, untracked, \
                        and diverged commits.\n\n\
                        +---------+-------+---------------------------------------+\n\
                        | Unicode | ASCII | Meaning                               |\n\
                        |---------|-------|---------------------------------------|\n\
                        |    ✓    |   +   | Project up to date                    |\n\
                        |    →    |   ->  | Ahead origin                          |\n\
                        |    ←    |   <-  | Behind origin                         |\n\
                        |    ◎    |   *   | Unstaged/Untracked files (pre commit) |\n\
                        |    ⨯    |   x   | Error occured                         |\n\
                        +---------+-------+---------------------------------------+\n\
                        \n\
                        Note on the output stream:\n\
                        \tAll handled errors are streamed to stderr. So you're able \
                        to pipe stdout to `wc -l`,\n\
                        \tand get the count of all found Git projects.";

#[derive(Parser)]
#[command(name = "GST (Status for Git)")]
#[command(about = ABOUT_CLI)]
#[command(version , long_about = None)]
pub struct Cli {
    /// Config: Specify an absolute path to your git projects.
    ///     Example: `-p /home/usr/all_my_git_projects`
    ///     Can be used multiple times -> or use `--paths` instead.
    #[arg(short, long, action = ArgAction::Append, verbatim_doc_comment)]
    pub path: Option<Vec<String>>,

    /// Config: Specify multiple paths, delimited by a space.  
    ///     Example: `gst --paths "/home/usr/pro1 /home/usr/pro2 /home/usr/pro3"`
    #[arg(long, verbatim_doc_comment)]
    pub paths: Option<String>,

    /// Displays more information:
    ///     `-v`: Show projects, that are up to date.
    ///     `-vv`: Additionally, show the absolute path per project.
    #[arg(short = 'v', long = "verbose", action = ArgAction::Count, verbatim_doc_comment)]
    pub verbose: u8,

    /// Config: Removes a single project path from the configuration.
    #[arg(short, long)]
    pub remove_path: Option<String>,

    /// Config: Removes all project paths from the configuration.
    #[arg(long)]
    pub purge_paths: bool,

    /// Config: Uses ASCII characters as status indicators, instead of UTF8/Unicode characters.
    ///     Example: `gst --ascii-mode true` -> uses ASCII characters instead.
    ///     Default: `--ascii-mode false`
    #[arg(short, long , verbatim_doc_comment)]
    pub ascii_mode: Option<String>,

    /// Displays the current configuration
    #[arg(short, long)]
    pub show_config: bool,

    /// Config: The search depth from 1 to 30 (Default: 5)
    #[arg(long)]
    pub search_depth: Option<i8>,

    /// Executes this program inside the current folder, without saving the path to the
    /// configuration
    #[arg(short, long)]
    pub current_dir: bool,
}
