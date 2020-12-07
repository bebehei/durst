use super::*;

use serde::Deserialize;

use std::fs::File;
use std::io::BufReader;
use std::error::Error;
use std::io::Read;

#[derive(Deserialize, Debug)]
pub struct DurstConfiguration {
    //pub rules: Option<Vec<rules::Rule>>,
}

/// Loads the default config file, but without spitting errors if no config is there
pub fn load_config_default() -> Result<DurstConfiguration, Error> {
    let config_basepath = var("XDG_CONFIG_HOME")
        .or_else(|_| var("HOME").map(|home| format!("{}/.config", home)))
        .unwrap();
    let config_path = format!("{}/durst/config.yml", config_basepath);
    //FIXME
    let cp2 = config_path.to_string();

    match read_file(config_path) {
        Ok(config_str) => {
            parse_configuration(&config_str)
        }
        Err(err) => {
            warn!("Cannot read config file '{:?}': {:?}", cp2.to_string(), err.to_string());
            parse_configuration(&"".to_string())
        }
    }
}

/// Loads the 
pub fn load_config_path(path: &String) -> Result<DurstConfiguration, Error> {
    match read_file(path.to_string()) {
        Ok(config_str) => {
            parse_configuration(&config_str)
        }
        Err(err) => {
            error!("Cannot read config file '{:?}': {:?}", path, err.to_string());
            Err(err)
        }
    }
}

/// Parse any YAML-String and spit out the corresponding configuration
pub fn parse_configuration(config_in: &String) -> Result<DurstConfiguration, Error> {
    let yaml = serde_yaml::from_str(config_in);
    match yaml {
        Ok(config) => {
            debug!("{:?}", config);
            Ok(config)
        }
        Err(err) => {
            warn!("Cannot parse configuration: {:?}", err.to_string());
            Err(err)
        }
    }
}

fn read_file(path: String) -> Result<String, std::io::Error> {
    let file = File::open(path)?;
    println!("file {:?}", file);
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    Ok(contents)
}
