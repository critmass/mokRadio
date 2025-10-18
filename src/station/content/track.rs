use std::{fs::DirEntry, path::{Path, PathBuf}, time::SystemTime};

use chrono::{Duration, TimeDelta};

/// Audio track with metadata for playlist management
pub struct Track {
    duration: Duration,    // Length of audio file
    modified: SystemTime,  // File modification time (used for ordering)
    location: PathBuf,     // Full path to audio file
}

impl PartialEq for Track {
    fn eq(&self, other: &Self) -> bool {
        self.modified == other.modified
    }
}

impl Eq for Track {}

// Tracks are ordered by modification time for Chronologic/Reverse playlists
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
    /// Currently only supports MP3 files
    pub fn new(dir_entry: &DirEntry) -> Option<Self> {
        let location = dir_entry.path();
        let duration = Duration::from_std(mp3_duration::from_path(&location).unwrap()).unwrap();
        let modified = dir_entry.metadata().unwrap().modified().unwrap();
        return Some(Track {
            duration, modified, location
        });
    }

    pub fn get_location(&self) -> &Path {
        &self.location
    }

    pub fn get_duration(&self) -> &TimeDelta {
        &self.duration
    }

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
/// Returns an iterator of Track objects, skipping non-file entries
pub fn load_tracks_from_path(playlist_path: &Path) -> impl Iterator<Item = Track> {
    std::fs::read_dir(playlist_path)
        .unwrap()
        .filter_map(|dir_entry| {
            let unwrapped_entry = dir_entry.ok()?;
            let meta_data = unwrapped_entry.metadata().ok()?;
            if meta_data.is_file() {
                Track::new(&unwrapped_entry)
            } else {
                None
            }
        })
}
