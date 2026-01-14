// Station module - manages radio stations with playlists and audio
pub mod config;
pub mod content;
pub mod utilities;


use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};

use rodio::{Decoder, OutputStream, Sink};

use content::{PlayType, Content};
use config::StationConfig;

use crate::radio::station::content::track::Track;
use crate::radio::station::utilities::whats_next::{self, next_chronologic, next_random, next_shuffle};


/// Radio station with playlist management and audio sink
pub struct Station {
    current_content: Option<Content>,  // Currently playing track or stream
    next_content: Option<Content>,     // Next queued content
    play_list: PlayType,               // Playlist type and tracks/streams
    purge: bool,                       // Delete audio files after playing
    on_air: bool,
    has_skipped: bool,                      // Station is currently broadcasting
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
        let mut new_station = Station {
            current_content: None,
            next_content: None,
            play_list,
            purge: station_configurations.purge,
            on_air: false,
            has_skipped: false,
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
                    self.play_list = PlayType::new("Shuffle", &self.station_path);
                } 
                return next_track;
            },
            PlayType::Chronologic(playlist) => {
                let next_track = next_chronologic(playlist);
                if playlist.len() < 1 {
                    self.go_off_air();
                }
                return next_track;
            },
            PlayType::Reverse(playlist) => {
                let next_track = whats_next::next_reverse(playlist);
                if playlist.len() < 1 {
                    self.go_off_air();
                }
                return next_track;
            },
            _ => return None
        } 
    }
    pub fn next(&mut self) -> Option<PathBuf> {

        let what_next = self.what_next()?;
        self.current_content = self.next_content.take();
        self.next_content = Some(Content::Track(what_next));
        match &self.next_content {
            None => return None,
            Some(content) => match content {
                Content::Track(track) => return Some(track.get_location().to_path_buf()),
                _ => return None
            }
        }
    }
    pub fn prime_content(&mut self) -> Vec<PathBuf> {

        let mut content_vector: Vec<PathBuf> = Vec::new();
        let next = self.next();

        if next.is_some() {
            content_vector.push(next.unwrap());
        }
        let next = self.next();
        if next.is_some() {
            content_vector.push(next.unwrap());
        }

        return content_vector;
    }
    pub fn push_to_sink(&mut self, audio_content: Decoder<BufReader<File>>) {

        if let Some(sink) = self.sink.as_mut() {

            sink.append(audio_content);
        }
    }
    pub fn go_on_air(&mut self) {

        self.on_air = true;
    }
    pub fn go_off_air(&mut self) {

        self.pause();
        self.on_air = false;
    }
    pub fn unpause(&mut self) {
        if let Some(sink) = self.sink.as_mut() {

            sink.play();
        }
        self.has_skipped = false;
    }
    pub fn pause(&mut self) {
        
        if let Some(sink) = self.sink.as_mut() {
            sink.pause();
        }
    }
    pub fn volume_set(&mut self, volume: f32) {

        if let Some(sink) = self.sink.as_mut() {
            sink.set_volume(volume);
        }
    }
    pub fn skip(&mut self) -> Option<PathBuf> {


        if self.has_skipped {
            return None;
        }
        if let Some(sink) = self.sink.as_mut() {
            
            self.has_skipped = true;
            sink.skip_one();
            return self.next();
        }
        else {
            return None;
        }
    }
    pub fn needs_next(&self) -> bool {

        if let Some(sink) = self.sink.as_ref() {
            return sink.len() < 2;
        }
        else {
            return false;
        }
        
    }
    pub fn is_on_air(&self) -> bool {
        return self.on_air;
    }
}
