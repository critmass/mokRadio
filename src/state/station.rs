
mod content;
mod config;

use std::path::Path;

use rodio::{OutputStream, Sink};

use content::{PlayType, Content};
use config::StationConfig;

pub struct Station {
    current_content: Option<Content>,
    next_content: Option<Content>,
    play_list: PlayType,
    purge: bool,
    on_air: bool,
    sink:Option<Sink>
}

impl Station {
    pub fn new (station_path:&Path, band:&OutputStream) -> Self {
        let station_sink = Sink::connect_new(band.mixer());
        let station_configurations = StationConfig::new(station_path);
        let play_list = PlayType::new(&station_configurations.play_type, station_path);
        let new_station = Station {
            current_content:None,
            next_content: None,
            play_list,
            purge: station_configurations.purge,
            on_air: false,
            sink:Some(station_sink)
        };
        return new_station;
    }
}