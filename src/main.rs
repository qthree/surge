#![crate_type = "bin"]

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate hyper;
extern crate hyper_native_tls;
extern crate toml;
extern crate rustyline;

mod youtube;
mod youtube_dl;
mod command;
mod backend;

use command::CommandCenter;

use rustyline::error::ReadlineError;
use rustyline::Editor;

use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

const SURGE_DIR: &'static str = ".surge";

#[derive(Deserialize)]
struct Config {
    download_path: String,
    youtube: Yt,
}

#[derive(Deserialize)]
struct Yt {
    api_key: String,
    max_results: u32,
}

fn main() {
    let surge_dir = format!("{}/{}", env!("HOME"), SURGE_DIR);
    let conf_path = format!("{}/surgeconf.toml", surge_dir);
    let history_path = format!("{}/history.txt", surge_dir);
    let history_path = history_path.as_str();

    let mut config_contents = String::new();

    match File::open(conf_path) {
        Ok(x) => {
            let mut buf_reader = BufReader::new(x);
            match buf_reader.read_to_string(&mut config_contents) {
                Ok(_) => (),
                Err(e) => panic!(e),
            }
        }
        Err(e) => panic!(e),
    }

    let config: Config = toml::from_str(&config_contents).unwrap();

    let mut cmd = CommandCenter::for_youtube(config.youtube.api_key,
                                             config.youtube.max_results,
                                             config.download_path);

    let mut rl = Editor::<()>::new();
    if rl.load_history(history_path).is_err() {
        ()
    }
    loop {
        let readline = rl.readline("surge ♫ ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(&line);
                if line != "" {
                    cmd.search(line);
                }
                continue;
            }
            Err(ReadlineError::Interrupted) => continue,
            Err(ReadlineError::Eof) |
            Err(_) => break,
        }
    }
    rl.save_history(history_path).unwrap();
}
