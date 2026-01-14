//! Content Management Module
//!
//! Defines the types of content a station can play and how playlists behave.
//! Includes track management, live stream support, and playlist strategies.

pub mod live;
pub mod track;

use std::{collections::BTreeSet, path::Path};

use live::LiveStream;
use track::{Track, load_tracks_from_path};
use rand::seq::SliceRandom;
use rand::rng;

/// Radio band identifier (AM or FM)
/// 
/// Used by Station Manager to organize stations and apply band shift
/// when mapping encoder values to station indices.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Band {
    AM,
    PM  // TODO: Should this be FM?
}

/// Unique identifier for a station combining band and index
/// 
/// Used in FileRequest/FileResponse messages to route decoded audio
/// back to the correct station.
/// 
/// # Example
/// ```
/// StationID { band: Band::AM, index: 3 }  // AM station #3 (4th station, 0-indexed)
/// ```
#[derive(Debug, PartialEq, Eq)]
pub struct StationID {
    pub band: Band,
    pub index: usize,  // 0-11 for 12 stations per band
}

impl Clone for StationID {
    fn clone(&self) -> Self {
        StationID { 
            band: self.band.clone(), 
            index: self.index.clone() 
        }
    }
}

/// Playlist behavior types for station content management
/// 
/// Each variant encapsulates both the playlist strategy and the
/// collection of tracks/streams that implement that strategy.
pub enum PlayType {
    /// Pick any random track from the list
    /// Tracks stay in the list and can be replayed
    Random(Vec<Track>),
    
    /// Play tracks oldest to newest by file modification date
    /// Tracks are removed as played; station goes off-air when empty
    Chronologic(BTreeSet<Track>),
    
    /// Play tracks newest to oldest by file modification date
    /// Tracks are removed as played; station goes off-air when empty
    Reverse(BTreeSet<Track>),
    
    /// Play all tracks once in random order, then reshuffle and repeat
    /// Tracks are removed as played; playlist reloads when exhausted
    Shuffle(Vec<Track>),
    
    /// Scheduled live streams (not yet implemented)
    Live(BTreeSet<LiveStream>),
    
    /// Station is off-air/inactive (no playlist)
    Dead
}

impl PlayType {
    /// Creates a PlayType from station.info configuration
    /// 
    /// Loads tracks from the station's playlist directory and initializes
    /// the appropriate data structure based on play_type string.
    /// 
    /// # Arguments
    /// * `play_type` - String from station.info ("Random", "Shuffle", etc.)
    /// * `station_path` - Path to station directory containing playlist/ folder
    /// 
    /// # Returns
    /// Initialized PlayType variant with tracks loaded from disk
    /// 
    /// # Playlist Directory Structure
    /// ```text
    /// station_00/
    ///   └── playlist/
    ///       ├── track1.mp3
    ///       ├── track2.mp3
    ///       └── track3.mp3
    /// ```
    pub fn new(play_type: &str, station_path: &Path) -> Self {
        match play_type {
            "Chronologic" => {
                // Load and sort tracks by modification date (oldest first)
                // BTreeSet automatically maintains sorted order
                let play_list: BTreeSet<Track> = 
                    load_tracks_from_path(&station_path.join("playlist")).collect();
                PlayType::Chronologic(play_list)
            },
            
            "Reverse" => {
                // Load and sort tracks by modification date (newest first)
                // BTreeSet maintains sorted order; iteration is reversed in utilities
                let play_list: BTreeSet<Track> = 
                    load_tracks_from_path(&station_path.join("playlist")).collect();
                PlayType::Reverse(play_list)
            },
            
            "Random" => {
                // Load tracks for random selection (tracks stay in list)
                let play_list: Vec<Track> = 
                    load_tracks_from_path(&station_path.join("playlist")).collect();
                PlayType::Random(play_list)
            },
            
            "Shuffle" => {
                // Load and shuffle tracks for one complete playthrough
                let mut play_list: Vec<Track> = 
                    load_tracks_from_path(&station_path.join("playlist")).collect();
                
                // Randomize the initial order
                play_list.shuffle(&mut rng());
                
                PlayType::Shuffle(play_list)
            },
            
            // Unknown play_type or explicit "Dead" -> inactive station
            _ => PlayType::Dead,
        }
    }
}

/// Content types that can be played on a station
/// 
/// Currently supports local audio files (Tracks) and live streams.
/// Live stream support is planned but not yet implemented.
pub enum Content {
    /// Local audio file (MP3, etc.)
    Track(Track),
    
    /// Live streaming content (planned feature)
    Live(LiveStream)
}
