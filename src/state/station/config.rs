use std::{fs::read_to_string, path::Path};
use serde::Deserialize;
use serde_json::from_str;

#[derive(Deserialize)]

pub struct StationConfig {
    pub play_type: String,
    pub purge: bool,
}

impl StationConfig {
    pub fn new(file_path: &Path) -> Self {
        let configuration_file = read_to_string(file_path);
        match configuration_file {
            Ok(configuration) => {
                let station_config: StationConfig = from_str(&configuration).unwrap();
                return station_config
            },
            Err(e) => {
                eprintln!("Failed to load config from {}: {}", file_path.display(), e);
                // Return a default "Dead" station config
                return StationConfig { 
                    play_type: "Dead".to_string(), 
                    purge: false
                }
            }
        }
    }
}
