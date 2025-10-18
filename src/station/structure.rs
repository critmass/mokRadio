use std::fs::File;
use std::path::Path;

use rodio::{OutputStream, Sink, Decoder};
use rand::seq::{IndexedRandom, SliceRandom};
use rand::rng;

use super::content::{PlayType, Content};
use super::config::StationConfig;

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
    pub fn go_on_air(&mut self) {
        match &mut self.play_list {
            PlayType::Dead => {},
            PlayType::Random(playlist) => {
                if self.current_content.is_none() {
                    if self.next_content.is_none() {
                        let current_track = playlist.choose(&mut rng());
                        self.current_content = Some(Content::Track(current_track.cloned().unwrap()));
                    }
                    else {
                        self.current_content = self.next_content.take();
                    }
                    let next_track = playlist.choose(&mut rng());
                    self.next_content = Some(Content::Track(next_track.cloned().unwrap()));
                    let current_file = File::open(
                        match self.current_content.as_ref().unwrap() {
                            Content::Track(current_track) => Ok(current_track.get_location()),
                            _ => Err("error")
                        }.unwrap()
                    ).unwrap();
                    let current_audio = Decoder::try_from(current_file).unwrap();
                }

            },
            PlayType::Shuffle(playlist) => {},
            PlayType::Chronologic(playlist) => {},
            PlayType::Reverse(playlist) => {},
            _ => {}
        }
    }
}
