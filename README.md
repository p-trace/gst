# GST (Status for Git)
## tl;dr
**What:** GST traverses through your local Git projects, and displays their current `git status` as 
[symbols](#Legend).  
**[Build](#Build):** `cargo build --release`  
**[Usage](#Usage):** `gst -h` or see [examples](#Examples)  

**Linux?** - yes  
**Windows??** - ... yes  


## What's going on
Ever lost the overview of your projects? Or even forgot, that a projects still exists?  
GST creates a simple overview, which projects contains:  
- untracked/tracked files,
- unstaged/staged files,
- and ahead/behind commits.

Example:  
```bash
$ gst
[◎] - uber-secret-project
[←] - rmrf-root
[◎] - look-fancy-plots
[→◎] - pkill-9-vim
```
> [!NOTE]
> Per default, it will omit projects, that are up-to-date.
> You can display them by passing `-v`

### Further Details
This is how it works:  
1. Read args & evaluate
2. Write changes into config, if any
3. Read config
4. Traverse through all given paths (per default 5 levels deep)
5. If `.git` dir found -> `git --git-dir {dir} fetch`
6. If no error -> `git -C {dir} status -b --porcelain`
7. Output result to stdout/stderr

## Legend
```text
+---------+-------+---------------------------------------+
| Unicode | ASCII | Meaning                               |
|---------|-------|---------------------------------------|
|    ✓    |   +   | Project up-to-date                    |
|    →    |   ->  | Ahead origin                          |
|    ←    |   <-  | Behind origin                         |
|    ◎    |   *   | Unstaged/Untracked files (pre commit) |
|    ⨯    |   x   | Error occured                         |
+---------+-------+---------------------------------------+
```

## Usage
```bash
Usage: gst [OPTIONS]

Options:
  -p, --path <PATH>                  Config: Specify an absolute path to your git projects.
                                         Example: `-p /home/usr/all_my_git_projects`
                                         Can be used multiple times -> or use `--paths` instead.
      --paths <PATHS>                Config: Specify multiple paths, delimited by a space.
                                         Example: `gst --paths "/home/usr/pro1 /home/usr/pro2 /home/usr/pro3"`
  -v, --verbose...                   Displays more information:
                                         `-v`: Show projects, that are up-to-date.
                                         `-vv`: Additionally, show the absolute path per project.
  -r, --remove-path <REMOVE_PATH>    Config: Removes a single project path from the configuration
      --purge-paths                  Config: Removes all project paths from the configuration
  -a, --ascii-mode <ASCII_MODE>      Config: Uses ASCII characters as status indicators, instead of UTF8/Unicode characters.
                                         Example: `gst --ascii-mode true` -> uses ASCII characters instead.
                                         Default: `--ascii-mode false`
  -s, --show-config                  Displays the current configuration
      --search-depth <SEARCH_DEPTH>  Config: The search depth from 1 to 30 (Default: 5)
  -c, --current-dir                  Executes this program inside the current folder, without saving the path to the configuration
  -h, --help                         Print help
  -V, --version                      Print version

```
## Configuration File
At its first run, it will try to create:  
- the directory `gst`
- the file `gst/gst.json`

It will be stored at:  
- Linux: `$HOME/.config/gst/gst.json`
- Windows: `%USERPROFILE%/.config/gst/gst.json`

So make sure, that the `.config` directory exists.  

## Examples
Run GST inside the current directory, and output up-to-date projects also  
```bash
gst -cv
```

Add path one by one, and adjust the search depth  
```bash
gst -p /home/usr/myprojects -p /home/usr/myotherprojects --search-depth 10
```

Add multiple paths  
```bash
gst --paths "/home/usr/myprojects /home/usr/myotherprojects"
```

## ASCII Mode
If your ancient terminal cannot display unicode, or if you want to use this program 
inside an pipeline, you can switch to ASCII characters.  

`gst --ascii-mode true`  

## Dependencies
- clap
- serde
- serde_json

## Build
Linux/Windows:  
`cargo build --release` or `make`  

Linux (static with musl, x86_64):  
Hint: you'll need `rustup target add x86_64-unknown-linux-musl`  
`cargo build --release --target x86_64-unknown-linux-musl`  
or  
`make build-linux-static`

## Tests
No (automated) tests. Sorry!  
Most of the functions are very simple. Only the `provider.rs/CheckGitProjects/traversal()` was not a first try.  
Since this method is recursive, it contains boundaries. Sooo it won't create a black hole.

