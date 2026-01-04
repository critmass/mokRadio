// Station Manager Thread
// Manages all radio stations, receives input events, sends file requests
pub mod station;
use std::sync::mpsc::{Receiver, Sender};

use rodio::OutputStream;
use station::Station;

use crate::{messages, radio::station::content::{StationID, Band}};

struct Radio {
    current_station:StationID,
    am:[Station; 12],
    fm:[Station; 12],
    output:OutputStream
}

impl Radio {
    pub fn new (current_dial_position:u64, current_band:Band) {
        
    }
}

/// Runs the station manager thread
/// 
/// Responsibilities:
/// - Owns all Station structs
/// - Receives input events (dial position, AM/FM)
/// - Controls sink volumes based on dial position
/// - Requests files from File Loader thread
/// - Appends decoded audio to sinks
pub fn run_station_manager(
    input_rx: Receiver<messages::InputEvent>,
    file_req_tx: Sender<messages::FileRequest>,
    file_resp_rx: Receiver<messages::FileResponse>
) {
    // TODO: Initialize stations
    // TODO: Main loop
    //   - Check input events
    //   - Update station volumes based on dial
    //   - Check sink lengths
    //   - Request files as needed
    //   - Append received audio
    
}

