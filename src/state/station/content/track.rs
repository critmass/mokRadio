use std::{ffi::OsString, path::Path, time::SystemTime};

use chrono::{Duration};

pub struct Track {
    pub length: Duration,
    pub title: OsString,
    pub modified: SystemTime
}

impl PartialEq for Track {
    fn eq(&self, other: &Self) -> bool {
        self.modified == other.modified
    }
}
impl Eq for Track {
    
}
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
    pub fn new(track_path:&Path) -> Self {

    }
}