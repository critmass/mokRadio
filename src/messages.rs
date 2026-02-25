// Centralized message types for inter-thread communication

use std::path::PathBuf;
use std::fs::File;
use rodio::Decoder;
use std::io::BufReader;

use crate::radio::station::content::track::Track;
use crate::radio::station::content::{Band, StationID};

// ===== Input Thread → Station Manager =====

/// Events from the Input thread about user controls
#[derive(Debug, Clone)]
pub enum InputEvent {
    /// Tuning dial moved to new_dial_position value (0-4095 or similar)
    DialMoved { new_dial_position: usize },
    
    /// AM/FM band switch toggled
    BandSwitched { new_band: Band }
}

// ===== Station Manager → File Loader =====

/// Requests from Station Manager to File Loader thread

pub enum FileRequest {
    /// Request to load a specific track for a station
    LoadTrack {
        station_id: StationID,
        file_path: PathBuf,
    },
    
    /// Request to scan a directory and return track metadata
    ScanDirectory {
        station_id: StationID,
        directory_path: PathBuf,
    },
}

// ===== File Loader → Station Manager =====

/// Responses from File Loader back to Station Manager
pub enum FileResponse {
    /// Decoded audio file ready to append to sink
    TrackLoaded {
        station_id: StationID,
        audio_content: Decoder<BufReader<File>>,
    },
    
    /// Directory scan complete with track metadata
    DirectoryScanned {
        station_id: StationID,
        tracks:Vec<Track>
        // TODO: Add track metadata list
    },
    
    /// Error loading file
    LoadError {
        station_id: StationID,
        error_message: String,
    },
}
