use std::{path::PathBuf, str::FromStr};

use crate::{store, term::error};

pub fn get_config_base_dir() -> PathBuf {
    let usrhome = match std::env::var("HOME") {
        Ok(s) => {
            if s != "" {
                s
            } else {
                "~".into()
            }
        }
        Err(_) => "~".into(),
    };
    let config_home = match std::env::var("XDG_CONFIG_HOME") {
        Ok(s) => {
            if s != "" {
                s
            } else {
                format!("{usrhome}/.config")
            }
        }
        Err(_) => {
            format!("{usrhome}/.config")
        }
    };

    return PathBuf::from_str(format!("{config_home}/greathelm").as_str()).unwrap();
}

pub fn get_data_base_dir() -> PathBuf {
    let usrhome = match std::env::var("HOME") {
        Ok(s) => {
            if s != "" {
                s
            } else {
                "~".into()
            }
        }
        Err(_) => "~".into(),
    };
    return PathBuf::from_str(format!("{usrhome}/.local/share/greathelm").as_str()).unwrap();
}

pub fn ensure_config_dirs() {
    let ghconfig_base = get_config_base_dir();
    let ghconfig_plugins =
        PathBuf::from_str(format!("{}/plugins", ghconfig_base.to_str().unwrap()).as_str()).unwrap();
    let ghconfig_scripts =
        PathBuf::from_str(format!("{}/scripts", ghconfig_base.to_str().unwrap()).as_str()).unwrap();
    let ghdata_base = get_data_base_dir();
    let ghdata_store = store::get_store_path();
    ensure_dir(ghconfig_base);
    ensure_dir(ghconfig_plugins);
    ensure_dir(ghconfig_scripts);
    ensure_dir(ghdata_base);
    ensure_dir(ghdata_store);
}

pub fn ensure_dir(path: PathBuf) {
    if !match path.try_exists() {
        Ok(ex) => ex,
        Err(_) => {
            error(format!("Could not ensure config directories. Abort."));
            std::process::exit(1);
        }
    } {
        match std::fs::create_dir_all(path) {
            Ok(_) => {}
            Err(e) => {
                error(format!("Could not ensure config directories. Abort."));
                eprintln!("{}", e);
                std::process::exit(1);
            }
        }
    }
}
