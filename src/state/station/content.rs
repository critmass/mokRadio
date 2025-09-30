pub mod live;
pub mod track;

use std::{collections::BTreeSet, fs::{read_dir}, path::Path};
use chrono::Duration;

use live::LiveStream;
use track::Track;


pub enum PlayType {
    Random(Vec<Track>),
    Chronologic(BTreeSet<Track>),
    Reverse(BTreeSet<Track>),
    Shuffle(Vec<Track>),
    Live(BTreeSet<LiveStream>),
    Dead
}

impl PlayType {
    pub fn new(play_type:&str, station_path:&Path) -> Self {
        match play_type {
            "Dead" => {
                return PlayType::Dead
            },
            "Live" => {
                let return_type = PlayType::Live(BTreeSet::new());
                return return_type;
            },
            "Chronologic" | "Reverse" => {
                let mut return_type = PlayType::Chronologic(BTreeSet::new());
                let play_list: BTreeSet<Track> = read_dir(station_path)
                    .unwrap()
                    .filter_map(|dir_entry| {
                        let unwrapped_entry = dir_entry.unwrap();
                        let meta_data = unwrapped_entry.metadata().unwrap();
                        if meta_data.is_file() {
                            let entry_track = Track {
                                length:meta_data.len() as Duration,
                                title:unwrapped_entry.file_name(),
                                modified:meta_data.modified().unwrap()
                            };
                            return Some(entry_track);
                        }
                        else {
                            return None
                        }
                    })
                    .collect();
                return return_type;},
            "Random" => {
                let return_type = PlayType::Dead;
                return return_type;
            },
            "Shuffle" => {
                let return_type = PlayType::Dead;
                return return_type;},
            _ => {
                let return_type = PlayType::Dead;
                return return_type;
            }
        }
    }
}

pub enum Content {
    Track(Track),
    Live(LiveStream)
}