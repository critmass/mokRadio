pub mod live;
pub mod track;

use std::{collections::BTreeSet, path::Path};

use live::LiveStream;
use track::{Track, load_tracks_from_path};
use rand::seq::SliceRandom;
use rand::rng;

pub enum Band {
    AM,
    PM
}
pub struct StationID {
    band:Band,
    index:usize
}

/// Playlist behavior types for station content management
pub enum PlayType {
    Random(Vec<Track>),           // Pick any track except the one just played
    Chronologic(BTreeSet<Track>), // Play tracks ordered by modification date
    Reverse(BTreeSet<Track>),     // Play tracks in reverse chronological order
    Shuffle(Vec<Track>),          // Play all tracks once in random order
    Live(BTreeSet<LiveStream>),   // Scheduled live streams (not yet implemented)
    Dead                          // Station is off-air/inactive
}

impl PlayType {
    /// Creates a PlayType from station.info configuration
    /// 
    /// Loads tracks from station_path/playlist/ folder or streams from schedule.info
    pub fn new(play_type: &str, station_path: &Path) -> Self {
        match play_type {
            "Chronologic" => {
                // Load and sort tracks by modification date (oldest first)
                let play_list: BTreeSet<Track> = 
                    load_tracks_from_path(&station_path.join("playlist")).collect();
                return PlayType::Chronologic(play_list);
            },
            
            "Reverse" => {
                // Load and sort tracks by modification date (newest first)
                let play_list: BTreeSet<Track> = 
                    load_tracks_from_path(&station_path.join("playlist")).collect();
                return PlayType::Reverse(play_list);
            },
            
            "Random" => {
                // Load tracks for random selection (excluding last played)
                let play_list: Vec<Track> = 
                    load_tracks_from_path(&station_path.join("playlist")).collect();
                return PlayType::Random(play_list);
            },
            
            "Shuffle" => {
                // Load and shuffle tracks for one complete playthrough
                let mut play_list: Vec<Track> = 
                    load_tracks_from_path(&station_path.join("playlist")).collect();
                play_list.shuffle(&mut rng());
                return PlayType::Shuffle(play_list);
            },
            
            _ => PlayType::Dead,
        }
    }
}

/// Content types that can be played on a station
pub enum Content {
    Track(Track),         // Local audio file
    Live(LiveStream)      // Streaming content
}
