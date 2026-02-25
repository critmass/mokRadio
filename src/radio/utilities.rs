use std::sync::mpsc::Sender;

use crate::constants;
use crate::messages::FileRequest;
use crate::radio::station::{Station, content::{StationID, Band}};

pub fn generate_station_volume_profile() -> [f32; constants::TICKS_PER_STATION] {

    let center = (constants::TICKS_PER_STATION / 2) as f32;
    let plateau_half_width = center * 0.06;
    let steepness = 0.05 * constants::TICKS_PER_STATION as f32;
        
    std::array::from_fn(|tick| {
        // Get position within the station's band (0 to TICKS_PER_STATION)
        let x = (tick % constants::TICKS_PER_STATION) as f32;
            
        let left_tanh = ((x - (center - plateau_half_width)) / steepness).tanh();
        let right_tanh = ((x - (center + plateau_half_width)) / steepness).tanh();
            
        let volume = 0.5 * (left_tanh - right_tanh);
            
        // Round to 3 decimal places
        (volume * 1000.0).round() / 1000.0
    })
}

pub fn skip_dormant_stations_in_band(
    current_band: &mut [Station; constants::NUMBER_OF_STATIONS], 
    file_requester: &Sender<FileRequest>,
    band: Band
) {
    current_band.iter_mut().enumerate().for_each(|(index, station)| {
        if let Some(request_path) = station.skip() {
            let request = FileRequest::LoadTrack {
                station_id: StationID { band, index },
                file_path: request_path
            };
            file_requester.send(request).ok();
        }
    });
}
pub fn skip_dormant_stations_in_band_except_current(
    current_band: &mut [Station; constants::NUMBER_OF_STATIONS], 
    file_requester: &Sender<FileRequest>,
    band: Band,
    current_station_index:usize
) {
    current_band.iter_mut().enumerate().for_each(|(index, station)| {
        if current_station_index != index {
            if let Some(request_path ) = station.skip() {
                let request = FileRequest::LoadTrack {
                    station_id: StationID { band, index },
                    file_path: request_path
                };
                file_requester.send(request).ok();
            }
        }
    });
}