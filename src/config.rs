use std::{
    env,
    path::PathBuf,
    io::{Read, Write},
    fs::{self, File, OpenOptions},
    process::exit,
};

use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Config {
    pub version: String,
    pub search_depth: i8,
    pub ascii_mode: bool,
    pub project_paths: Vec<PathBuf>,
}

impl Config {
    fn defaults() -> Config {
        let config = Config {
            version: option_env!("CARGO_PKG_VERSION").unwrap_or("UNKNOWN").to_string(),
            search_depth: 5,
            ascii_mode: false,
            project_paths: vec![],
        };
        config
    }

    pub fn new() -> Config {
        let config_file = Self::check_config();
        let config_fp = &config_file.0;
        let is_config_missing = &config_file.1;

        if *is_config_missing {
            let defaults = Config::defaults();
            write_json(config_fp.to_path_buf(), &defaults);
        }

        let mut file = match File::open(config_fp) {
            Ok(fh) => fh,
            Err(e) => {
                dbg!(e);
                exit(1);
            }
        };
        let mut buf = String::new();
        match file.read_to_string(&mut buf) {
            Ok(..) => (),
            Err(e) => {
                dbg!(e);
                exit(1);
            }
        };
        let deserialized: Config = match serde_json::from_str(&buf) {
            Ok(v) => v,
            Err(_) => {
                eprintln!("Config file seems to be empty or corrupted. Delete the file and run the program again.");
                exit(1);
            }
        };
        deserialized
    }

    fn config_dir() -> PathBuf {
        #[cfg(target_family = "windows")]
        let home_env_var = "USERPROFILE";

        #[cfg(target_family = "unix")]
        let home_env_var = "HOME";

        let home_dir = match env::var(home_env_var) {
            Ok(v) => v,
            Err(e) => {
                dbg!(e);
                exit(1);
            },
        };

        let mut config_fp = PathBuf::new();
        config_fp.push(home_dir);
        config_fp.push(".config");
        config_fp
    }

    fn check_config() -> (PathBuf, bool) {
        // check for config dir
        let pbuf = Self::config_dir();
        if !pbuf.try_exists().unwrap() {
            eprintln!("Can not find directory `{}`", pbuf.to_str().unwrap());
            exit(1);
        };

        // check for gst dir
        let mut gst_dir = PathBuf::new();
        gst_dir.push(pbuf);
        gst_dir.push("gst");

        if !gst_dir.try_exists().unwrap() {
            eprintln!("'gst' config dir does not exist. Creating it ...");
            match fs::create_dir(&gst_dir) {
                Ok(()) => (),
                Err(e) => {
                    dbg!(e);
                    exit(1);
                }
            }
        }

        // check for gst config file
        let mut config_fp = PathBuf::new();
        config_fp.push(gst_dir);
        config_fp.push("gst.json");

        let mut missing_config_file = false;

        if !config_fp.is_file() {
            eprintln!("'gst' config file does not exist. Creating it ...");
            missing_config_file = true;
            let _ = File::create_new(&config_fp);
        };
        (config_fp, missing_config_file)
    }

    pub fn show_config(&mut self) -> &mut Self {
        let config = serde_json::to_string_pretty(&self).unwrap();
        println!("{}", config);
        self
    }

    pub fn write_config(&mut self) -> () {
        let config_file = Self::check_config();
        write_json(config_file.0, self);
        println!("[*] Updated configuration");
        exit(0);
    }

    pub fn add_path(&mut self, fp: &str) -> &mut Self {
        match found_path_in_paths(&self.project_paths, fp) {
            true => (),
            false => self.project_paths.push(fp.into()),
        }
        self
    }

    pub fn remove_path(&mut self, fp: &str) -> &mut Self {
        match found_path_in_paths(&self.project_paths, fp) {
            true => self.project_paths.retain(|x| *x.to_str().unwrap() != *fp),
            false => (),
        }
        self
    }

    pub fn purge_paths(&mut self) -> &mut Self {
        self.project_paths = Vec::new();
        self
    }

    pub fn ascii_enabled(&mut self, ascii_mode_enabled: bool) -> &mut Self {
        self.ascii_mode = ascii_mode_enabled;
        self
    }

    pub fn search_depth(&mut self, depth: i8) -> &mut Self {
        self.search_depth = depth;
        self
    }
}

fn write_json(file_ref: PathBuf, content: &Config) -> () {
    let mut file = match OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(file_ref) {
            Ok(fh) => fh,
            Err(e) => {
                dbg!(e);
                exit(1);
            }
        };

    let serialized = serde_json::to_string_pretty(&content).unwrap();
    let buf = serialized.as_bytes();

    match file.write(buf) {
        Ok(..) => (),
        Err(e) => {
            eprintln!("Could not write into buffer: {e}");
            exit(1);
        }
    }
}

fn found_path_in_paths(paths: &Vec<PathBuf>, fp: &str) -> bool {
    if paths.iter().any(|i| i.to_str().unwrap() == fp) {
        return true
    }
    return false
}
