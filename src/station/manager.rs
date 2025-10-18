// Station Manager Thread
// Manages all radio stations, receives input events, sends file requests

use std::sync::mpsc::{Receiver, Sender};

/// Runs the station manager thread
/// 
/// Responsibilities:
/// - Owns all Station structs
/// - Receives input events (dial position, AM/FM)
/// - Controls sink volumes based on dial position
/// - Requests files from File Loader thread
/// - Appends decoded audio to sinks
pub fn run_station_manager(
    input_rx: Receiver<InputEvent>,
    file_req_tx: Sender<FileRequest>,
    file_resp_rx: Receiver<FileResponse>
) {
    // TODO: Initialize stations
    // TODO: Main loop
    //   - Check input events
    //   - Update station volumes based on dial
    //   - Check sink lengths
    //   - Request files as needed
    //   - Append received audio
}

// Placeholder types - will be defined in messages.rs
struct InputEvent;
struct FileRequest;
struct FileResponse;
