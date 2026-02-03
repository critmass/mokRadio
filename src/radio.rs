// Station Manager Thread
// Manages all radio stations, receives input events, sends file requests
pub mod station;
use std::{path::Path, sync::mpsc::{Receiver, Sender}};

use rodio::OutputStream;
use station::Station;

use crate::{radio::station::content::{StationID, Band}};
use crate::messages;
use crate::constants;

struct Radio {
    current_station:StationID,
    current_dial_position:usize,
    am:[Station; constants::NUMBER_OF_STATIONS],
    fm:[Station; constants::NUMBER_OF_STATIONS],
    am_volume_profile:[f64; constants::ENCODER_HALF],
    fm_volume_profile:[f64; constants::ENCODER_HALF],
    output:OutputStream,
    input_events: Receiver<messages::InputEvent>,
    file_requester: Sender<messages::FileRequest>,
    file_returns: Receiver<messages::FileResponse>,
}

impl Radio {
    pub fn new (
        current_dial_position:usize, 
        current_band:Band, 
        input_events: Receiver<messages::InputEvent>,
        file_requester: Sender<messages::FileRequest>,
        file_returns: Receiver<messages::FileResponse> 

    ) -> Self {

        let current_station = StationID {
            index: current_dial_position / constants::TICKS_PER_STATION,
            band: current_band
        };
        let am = Radio::initialize_station_array(&Band::AM);
        let am = Radio::initialize_station_array(&Band::FM);

    }
    pub fn initialize_station_array(band: &Band) -> [Station; constants::NUMBER_OF_STATIONS] {

        let state_vector: Vec<Station> = Vec::new();
        let 

        for station_number in 0..constants::NUMBER_OF_STATIONS {
            let station_path = Path::new();
        }
    }
}

