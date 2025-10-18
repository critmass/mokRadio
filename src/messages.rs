// Centralized message types for inter-thread communication

use std::path::PathBuf;
use std::fs::File;
use std::io::BufReader;
use rodio::Decoder;

// ===== Input Thread → Station Manager =====

/// Events from the Input thread about user controls
#[derive(Debug, Clone)]
pub enum InputEvent {
    /// Tuning dial moved to new ADC value (0-4095 or similar)
    DialMoved { adc_value: u16 },
    
    /// AM/FM band switch toggled
    BandSwitched { is_fm: bool },
}

// ===== Station Manager → File Loader =====

/// Requests from Station Manager to File Loader thread
#[derive(Debug)]
pub enum FileRequest {
    /// Request to load a specific track for a station
    LoadTrack {
        station_id: usize,
        file_path: PathBuf,
    },
    
    /// Request to scan a directory and return track metadata
    ScanDirectory {
        station_id: usize,
        directory_path: PathBuf,
    },
}

// ===== File Loader → Station Manager =====

/// Responses from File Loader back to Station Manager
pub enum FileResponse {
    /// Decoded audio file ready to append to sink
    TrackLoaded {
        station_id: usize,
        decoder: Decoder<BufReader<File>>,
    },
    
    /// Directory scan complete with track metadata
    DirectoryScanned {
        station_id: usize,
        // TODO: Add track metadata list
    },
    
    /// Error loading file
    LoadError {
        station_id: usize,
        error_message: String,
    },
}
