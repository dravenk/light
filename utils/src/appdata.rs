// Copyright (c) 2013-2017 The btcsuite developers
// Use of this source code is governed by an ISC
// license that can be found in the LICENSE file.

use std::env;
use std::path::PathBuf;

fn appdata_dir(os: &str, app_name: &str) -> PathBuf {
    if app_name.is_empty() || app_name == "." {
        return PathBuf::from(".");
    }

    let app_name = app_name.trim_start_matches('.');
    let app_name_upper = format!("{}{}", &app_name[0..1].to_uppercase(), &app_name[1..]);
    let app_name_lower = format!("{}{}", &app_name[0..1].to_lowercase(), &app_name[1..]);

    let home_dir = env::var("HOME").unwrap_or_default();

    match os {
        "windows" => {
            let app_data = env::var("LOCALAPPDATA").unwrap_or_else(|_| env::var("APPDATA").unwrap_or_default());
            if !app_data.is_empty() {
                return PathBuf::from(app_data).join(app_name_upper);
            }
        }
        "darwin" => {
            if !home_dir.is_empty() {
                return PathBuf::from(home_dir).join("Library").join("Application Support").join(app_name_upper);
            }
        }
        "plan9" => {
            if !home_dir.is_empty() {
                return PathBuf::from(home_dir).join(app_name_lower);
            }
        }
        _ => {
            if !home_dir.is_empty() {
                return PathBuf::from(home_dir).join(format!(".{}", app_name_lower));
            }
        }
    }

    PathBuf::from(".")
}

pub fn app_dir() -> PathBuf {
    let app_name = "light";
    appdata_dir(env::consts::OS, app_name)
}

pub fn key_path() -> String {
    let keyfile_path = format!("{}/{}", app_dir().display(), "key.toml");
    keyfile_path
}

pub fn db_dir() -> String {
    let db_path = format!("{}/{}", app_dir().display(), "db");
    db_path
}

pub fn db_path() -> String {
    // let path = data_dir().unwrap();
    let db_path = format!("{}/{}", db_dir(), "light.db");
    db_path
}
