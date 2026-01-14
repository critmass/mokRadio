//! Track Module - Audio file metadata and loading
//! 
//! Represents individual audio files with metadata for playlist management.
//! Tracks are sorted by file modification time for Chronologic/Reverse playlists.

use std::{fs::DirEntry, path::{Path, PathBuf}, time::SystemTime};
use chrono::{Duration, TimeDelta};

/// Audio track with metadata for playlist management
/// 
/// Represents a single audio file with:
/// - Duration (for time tracking, UI display)
/// - Modification time (for Chronologic/Reverse ordering)
/// - File path (for loading and decoding)
pub struct Track {
    /// Length of the audio file
    duration: Duration,
    
    /// File modification time (used for Chronologic/Reverse playlist ordering)
    modified: SystemTime,
    
    /// Full path to the audio file
    location: PathBuf,
}

// Tracks are compared by modification time for BTreeSet ordering
impl PartialEq for Track {
    fn eq(&self, other: &Self) -> bool {
        self.modified == other.modified
    }
}

impl Eq for Track {}

/// Tracks are ordered by modification time for Chronologic/Reverse playlists
/// 
/// This allows BTreeSet to automatically maintain chronological order.
/// Chronologic playlists iterate forward, Reverse playlists iterate backward.
impl Ord for Track {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.modified.cmp(&other.modified)
    }
}

impl PartialOrd for Track {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Track {
    /// Creates a Track from a directory entry
    /// 
    /// Reads metadata from the filesystem and decodes duration from the audio file.
    /// 
    /// # Arguments
    /// * `dir_entry` - Directory entry from fs::read_dir()
    /// 
    /// # Returns
    /// - `Some(Track)` if file can be read and duration extracted
    /// - `None` if file is inaccessible or not a valid audio file
    /// 
    /// # Current Limitations
    /// Only supports MP3 files. Other formats will fail to parse duration.
    /// 
    /// # Panics
    /// Currently panics if metadata or duration extraction fails.
    /// TODO: Return None gracefully for invalid files
    pub fn new(dir_entry: &DirEntry) -> Option<Self> {
        let location = dir_entry.path();
        
        // Extract MP3 duration (will fail for non-MP3 files)
        let duration = Duration::from_std(
            mp3_duration::from_path(&location).unwrap()
        ).unwrap();
        
        // Get file modification time from filesystem metadata
        let modified = dir_entry.metadata().unwrap().modified().unwrap();
        
        Some(Track {
            duration,
            modified,
            location
        })
    }

    /// Returns the file path for this track
    /// 
    /// Used by Station to get the path for FileRequest messages.
    pub fn get_location(&self) -> &Path {
        &self.location
    }

    /// Returns the duration of this track
    /// 
    /// Can be used for UI display or calculating playlist length.
    pub fn get_duration(&self) -> &TimeDelta {
        &self.duration
    }

    /// Returns the file modification time
    /// 
    /// Used for Chronologic/Reverse playlist ordering.
    pub fn was_modified_on(&self) -> &SystemTime {
        &self.modified
    }
}

impl Clone for Track {
    fn clone(&self) -> Self {
        Track { 
            duration: self.duration.clone(), 
            modified: self.modified.clone(), 
            location: self.location.clone() 
        }
    }
}

/// Loads MP3 tracks from a playlist directory
/// 
/// Scans the directory and creates Track objects for all audio files.
/// Non-file entries (directories, symlinks) are silently skipped.
/// 
/// # Arguments
/// * `playlist_path` - Path to playlist directory (e.g., `/stations/am/00/playlist/`)
/// 
/// # Returns
/// Iterator of Track objects for each valid audio file found
/// 
/// # Behavior
/// - Only processes files (directories are skipped)
/// - Files that fail to load are filtered out (won't panic entire operation)
/// - Currently only works with MP3 files
/// 
/// # Panics
/// Panics if the directory cannot be read
/// 
/// # Example
/// ```
/// let tracks: Vec<Track> = load_tracks_from_path(Path::new("/stations/am/00/playlist"))
///     .collect();
/// ```
pub fn load_tracks_from_path(playlist_path: &Path) -> impl Iterator<Item = Track> {
    std::fs::read_dir(playlist_path)
        .unwrap()
        .filter_map(|dir_entry| {
            // Skip entries that can't be read
            let unwrapped_entry = dir_entry.ok()?;
            
            // Get metadata to check if this is a file
            let meta_data = unwrapped_entry.metadata().ok()?;
            
            // Only process files (skip directories)
            if meta_data.is_file() {
                Track::new(&unwrapped_entry)
            } else {
                None
            }
        })
}
