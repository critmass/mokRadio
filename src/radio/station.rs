//! Station Module - Core radio station implementation
//! 
//! This module provides the Station struct which represents a single radio station
//! with its own playlist, audio sink, and state management. Each station operates
//! independently with its own content queue and playback controls.
//! 
//! # Architecture
//! - Each station has an audio `Sink` for playback
//! - Maintains current and next content for gapless playback
//! - Manages playlist state (Random, Shuffle, Chronologic, etc.)
//! - Provides interface for Station Manager to control playback

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
/// 
/// Represents a single station that can play audio content according to
/// different playlist strategies. Owned and controlled by Station Manager.
pub struct Station {
    /// Currently playing content (track or live stream)
    current_content: Option<Content>,
    
    /// Next queued content for gapless playback
    next_content: Option<Content>,
    
    /// Playlist type and associated track collection
    play_list: PlayType,
    
    /// Whether to delete audio files after playing (for ephemeral content)
    purge: bool,
    
    /// Station has valid configuration and can broadcast
    on_air: bool,
    
    /// Flag to prevent duplicate skips during turnover events
    has_skipped: bool,
    
    /// Audio output sink for this station's playback
    sink: Option<Sink>,
    
    /// Path to station directory (for reloading playlists)
    station_path: PathBuf
}

impl Station {
    /// Creates a new station from a directory containing station.info and playlist files
    /// 
    /// # Arguments
    /// * `station_path` - Path to station folder (e.g., `/stations/am/00/`)
    /// * `band` - Shared audio output stream to connect this station's sink to
    /// 
    /// # Station Directory Structure
    /// ```text
    /// station_00/
    ///   ├── station.info     (JSON config: play_type, purge)
    ///   └── playlist/        (Audio files)
    ///       ├── track1.mp3
    ///       └── track2.mp3
    /// ```
    /// 
    /// # Returns
    /// A new Station instance with:
    /// - Sink connected to the output stream
    /// - Playlist loaded according to station.info
    /// - Content fields initialized as None (call `prime_content()` to load)
    pub fn new(station_path: &Path, band: &OutputStream) -> Self {
        // Create dedicated audio sink for this station
        let station_sink = Sink::connect_new(band.mixer());
        
        // Load station configuration from JSON
        let station_configurations = StationConfig::new(station_path);
        
        // Initialize playlist based on play_type
        let play_list = PlayType::new(&station_configurations.play_type, station_path);
        
        let new_station = Station {
            current_content: None,
            next_content: None,
            play_list,
            purge: station_configurations.purge,
            on_air: false,
            has_skipped: false,
            sink: Some(station_sink),
            station_path: station_path.to_path_buf()
        };

        new_station
    }
    
    /// Gets the next track according to the station's playlist strategy
    /// 
    /// Behavior depends on playlist type:
    /// - **Random**: Picks any random track from the list
    /// - **Shuffle**: Removes and returns next track; reloads when empty
    /// - **Chronologic**: Returns oldest unplayed track; goes off-air when empty
    /// - **Reverse**: Returns newest unplayed track; goes off-air when empty
    /// - **Dead**: Always returns None
    /// 
    /// # Returns
    /// - `Some(Track)` - Next track to queue
    /// - `None` - Playlist exhausted or station is Dead
    pub fn what_next(&mut self) -> Option<Track> {
        match &mut self.play_list {
            // Dead stations have no content
            PlayType::Dead => None,
            
            // Random: pick any track (track stays in list)
            PlayType::Random(playlist) => {
                next_random(playlist)
            },
            
            // Shuffle: remove and return track, reload when empty
            PlayType::Shuffle(playlist) => {
                let next_track = next_shuffle(playlist);
                
                // Reload shuffle playlist when exhausted
                if playlist.is_empty() {
                    self.play_list = PlayType::new("Shuffle", &self.station_path);
                }
                
                next_track
            },
            
            // Chronologic: play oldest first, go off-air when done
            PlayType::Chronologic(playlist) => {
                let next_track = next_chronologic(playlist);
                
                if playlist.is_empty() {
                    self.go_off_air();
                }
                
                next_track
            },
            
            // Reverse: play newest first, go off-air when done
            PlayType::Reverse(playlist) => {
                let next_track = whats_next::next_reverse(playlist);
                
                if playlist.is_empty() {
                    self.go_off_air();
                }
                
                next_track
            },
            
            // Catch-all for future playlist types
            _ => None
        }
    }
    
    /// Advances the content queue and returns the path for the new next track
    /// 
    /// State transitions:
    /// 1. Moves `next_content` → `current_content`
    /// 2. Gets new track from playlist → new `next_content`
    /// 3. Returns path of new `next_content` for File Loader to decode
    /// 
    /// # Returns
    /// - `Some(PathBuf)` - Path to file for Station Manager to request
    /// - `None` - No more tracks available (playlist exhausted)
    /// 
    /// # Usage
    /// Called by Station Manager when:
    /// - Sink needs more audio (`needs_next()` returns true)
    /// - Station is skipped during turnover
    pub fn next(&mut self) -> Option<PathBuf> {
        // Get next track from playlist
        let what_next = self.what_next()?;
        
        // Shift content queue forward
        self.current_content = self.next_content.take();
        self.next_content = Some(Content::Track(what_next));
        
        // Return path for file request
        match &self.next_content {
            None => None,
            Some(content) => match content {
                Content::Track(track) => Some(track.get_location().to_path_buf()),
                _ => None
            }
        }
    }
    
    /// Initializes the station with first two tracks
    /// 
    /// Loads:
    /// 1. First track → `current_content`
    /// 2. Second track → `next_content`
    /// 
    /// # Returns
    /// Vector of file paths for Station Manager to send to File Loader
    /// 
    /// # Usage
    /// Called by Station Manager during initialization to start loading
    /// audio files for this station. Station is not ready for playback
    /// until File Loader returns decoded audio via `push_to_sink()`.
    pub fn prime_content(&mut self) -> Vec<PathBuf> {
        let mut content_vector: Vec<PathBuf> = Vec::new();
        
        // Get first track
        let next = self.next();
        if next.is_some() {
            content_vector.push(next.unwrap());
        }
        
        // Get second track
        let next = self.next();
        if next.is_some() {
            content_vector.push(next.unwrap());
        }

        content_vector
    }
    
    /// Appends decoded audio to this station's sink
    /// 
    /// Called by Station Manager when File Loader returns a decoded track.
    /// The audio is added to the sink's queue and will play when:
    /// - This is the active station (sink is playing)
    /// - Previous audio in the queue finishes
    /// 
    /// # Arguments
    /// * `audio_content` - Decoded audio stream ready for playback
    pub fn push_to_sink(&mut self, audio_content: Decoder<BufReader<File>>) {
        if let Some(sink) = self.sink.as_mut() {
            sink.append(audio_content);
        }
    }
    
    /// Marks station as on-air (has valid configuration and content)
    /// 
    /// Sets the `on_air` flag to true. This indicates the station:
    /// - Successfully loaded its configuration
    /// - Has a valid playlist
    /// - Can broadcast when selected
    /// 
    /// Note: Station may be on-air but paused (not the currently active station)
    pub fn go_on_air(&mut self) {
        self.on_air = true;
    }
    
    /// Takes station off-air and pauses playback
    /// 
    /// Called when:
    /// - Chronologic/Reverse playlists are exhausted
    /// - Station encounters unrecoverable errors
    /// 
    /// Effects:
    /// - Pauses the sink
    /// - Sets `on_air = false`
    /// - Station becomes inactive (pure static on the dial)
    pub fn go_off_air(&mut self) {
        self.pause();
        self.on_air = false;
    }
    
    /// Resumes playback of this station's sink
    /// 
    /// Called by Station Manager when user tunes to this station.
    /// Also resets the `has_skipped` flag to allow future turnover events.
    pub fn unpause(&mut self) {
        if let Some(sink) = self.sink.as_mut() {
            sink.play();
        }
        self.has_skipped = false;
    }
    
    /// Pauses this station's sink
    /// 
    /// Called by Station Manager when user tunes away from this station.
    /// Audio playback halts but position is maintained.
    pub fn pause(&mut self) {
        if let Some(sink) = self.sink.as_mut() {
            sink.pause();
        }
    }
    
    /// Sets the volume of this station's audio output
    /// 
    /// # Arguments
    /// * `volume` - Volume level from 0.0 (silent) to 1.0 (full volume)
    /// 
    /// # Usage
    /// Called by Station Manager based on dial position to create the
    /// smooth fade between station audio and static as the dial is tuned.
    pub fn volume_set(&mut self, volume: f32) {
        if let Some(sink) = self.sink.as_mut() {
            sink.set_volume(volume);
        }
    }
    
    /// Skips the current track and advances to the next
    /// 
    /// Used during turnover events to keep all non-active stations
    /// moving forward in "radio time". Prevents duplicate skips with
    /// the `has_skipped` flag.
    /// 
    /// # Returns
    /// - `Some(PathBuf)` - Path to new track for File Loader to decode
    /// - `None` - Already skipped this session, or no more tracks available
    /// 
    /// # Turnover Behavior
    /// The `has_skipped` flag ensures each station only skips once per
    /// turnover event. Flag is reset when station is unpaused (becomes active).
    pub fn skip(&mut self) -> Option<PathBuf> {
        // Prevent duplicate skips
        if self.has_skipped {
            return None;
        }
        
        if let Some(sink) = self.sink.as_mut() {
            self.has_skipped = true;
            sink.skip_one();
            return self.next();
        }
        
        None
    }
    
    /// Checks if station's sink needs more audio
    /// 
    /// # Returns
    /// `true` if sink has fewer than 2 sources queued, indicating it's
    /// time to request the next track to prevent playback gaps.
    /// 
    /// # Usage
    /// Called by Station Manager in main loop to determine when to
    /// request next track from File Loader.
    pub fn needs_next(&self) -> bool {
        if let Some(sink) = self.sink.as_ref() {
            return sink.len() < 2;
        }
        
        false
    }
    
    /// Returns whether this station is currently on-air
    /// 
    /// # Returns
    /// `true` if station has valid configuration and can broadcast,
    /// `false` if station is Dead or off-air
    pub fn is_on_air(&self) -> bool {
        self.on_air
    }
}
