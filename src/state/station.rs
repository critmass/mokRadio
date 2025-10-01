mod content;
mod config;

use std::path::Path;

use rodio::{OutputStream, Sink};

use content::{PlayType, Content};
use config::StationConfig;

/// Radio station with playlist management and audio sink
pub struct Station {
    current_content: Option<Content>,  // Currently playing track or stream
    next_content: Option<Content>,     // Next queued content
    play_list: PlayType,               // Playlist type and tracks/streams
    purge: bool,                       // Delete audio files after playing
    on_air: bool,                      // Station is currently broadcasting
    sink: Option<Sink>                 // Audio output sink
}

impl Station {
    /// Creates a new station from a folder containing station.info and playlist
    /// 
    /// # Arguments
    /// * `station_path` - Path to station folder
    /// * `band` - Audio output stream to connect sink to
    pub fn new(station_path: &Path, band: &OutputStream) -> Self {
        let station_sink = Sink::connect_new(band.mixer());
        let station_configurations = StationConfig::new(station_path);
        let play_list = PlayType::new(&station_configurations.play_type, station_path);
        let new_station = Station {
            current_content: None,
            next_content: None,
            play_list,
            purge: station_configurations.purge,
            on_air: false,
            sink: Some(station_sink)
        };
        return new_station;
    }
}
