// Station module - manages radio stations with playlists and audio
pub mod config;
pub mod content;
pub mod utilities;


use std::path::{Path, PathBuf};
use std::sync::mpsc::Sender;

use rodio::{OutputStream, Sink};
use rand::seq::{IndexedRandom};
use rand::rng;

use content::{PlayType, Content, StationID};
use config::StationConfig;

use crate::messages::FileRequest;
use crate::radio::station::content::track::Track;
use crate::radio::station::utilities::whats_next::{self, next_chronologic, next_random, next_shuffle};


/// Radio station with playlist management and audio sink
pub struct Station {
    current_content: Option<Content>,  // Currently playing track or stream
    next_content: Option<Content>,     // Next queued content
    play_list: PlayType,               // Playlist type and tracks/streams
    purge: bool,                       // Delete audio files after playing
    on_air: bool,                      // Station is currently broadcasting
    sink: Option<Sink>,                // Audio output sink
    station_path: PathBuf
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
            sink: Some(station_sink),
            station_path:station_path.to_path_buf()
        };
        return new_station;
    }
    pub fn what_next(&mut self) -> Option<Track> {
        match &mut self.play_list {
            PlayType::Dead => return None,
            PlayType::Random(playlist) => {
                let next_track = next_random(playlist);
                return next_track;
            },
            PlayType::Shuffle(playlist) => {
                let next_track = next_shuffle(playlist);
                if playlist.len() < 1 {
                    self.play_list = PlayType::new("Shuffle", &*self.station_path);
                } 
                return next_track;
            },
            PlayType::Chronologic(playlist) => {
                let next_track = next_chronologic(playlist);
                if playlist.len() < 1 {
                    self.play_list = PlayType::new("Chronologic", &*self.station_path);
                }
                return next_track;
            },
            PlayType::Reverse(playlist) => {
                let next_track = whats_next::next_reverse(playlist);
                if playlist.len() < 1 {
                    self.play_list = PlayType::new("Reverse", &*self.station_path);
                }
                return next_track;
            },
            _ => None
        }
        
    }
    pub fn next(&mut self) {

        let what_next = self.what_next();
    }
    pub fn go_on_air(&mut self) {
    }
}
