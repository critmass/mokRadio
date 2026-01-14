//! Station Configuration Module
//! 
//! Handles loading and parsing of station.info JSON configuration files.
//! Each station directory contains a station.info file that defines:
//! - Playlist type (Random, Shuffle, Chronologic, etc.)
//! - Purge flag (whether to delete files after playing)

use std::{fs::read_to_string, path::Path};
use serde::Deserialize;
use serde_json::from_str;

/// Station configuration loaded from station.info JSON file
/// 
/// # JSON Format
/// ```json
/// {
///     "play_type": "Random",
///     "purge": false
/// }
/// ```
/// 
/// # Valid play_type Values
/// - "Random" - Pick random tracks, keep all in playlist
/// - "Shuffle" - Play all tracks once in random order
/// - "Chronologic" - Play tracks oldest to newest by file modification date
/// - "Reverse" - Play tracks newest to oldest by file modification date
/// - "Dead" - Station is off-air/inactive
#[derive(Deserialize)]
pub struct StationConfig {
    /// Type of playlist behavior
    pub play_type: String,
    
    /// Whether to delete audio files after playing (for ephemeral content)
    pub purge: bool,
}

impl StationConfig {
    /// Loads station configuration from station.info JSON file
    /// 
    /// # Arguments
    /// * `file_path` - Path to station directory (looks for station.info inside)
    /// 
    /// # Returns
    /// - Successfully parsed StationConfig if file exists and is valid JSON
    /// - Default "Dead" config if file is missing or malformed
    /// 
    /// # Error Handling
    /// Rather than propagating errors, this function returns a safe default
    /// (Dead station) and logs the error. This allows the system to continue
    /// operating even if individual station configs are corrupted.
    pub fn new(file_path: &Path) -> Self {
        // Attempt to read the configuration file
        let configuration_file = read_to_string(file_path);
        
        match configuration_file {
            Ok(configuration) => {
                // Parse JSON into StationConfig struct
                let station_config: StationConfig = from_str(&configuration).unwrap();
                station_config
            },
            Err(e) => {
                // Log error and return default "Dead" station
                eprintln!("Failed to load config from {}: {}", file_path.display(), e);
                
                // Return a default "Dead" station config
                // This allows system to continue even with missing/corrupted configs
                StationConfig { 
                    play_type: "Dead".to_string(), 
                    purge: false
                }
            }
        }
    }
}
