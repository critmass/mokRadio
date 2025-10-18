// mokRadio - Vintage Radio with Modern Playlists
// A Raspberry Pi project to turn a vintage radio into a playlist player

mod station;
mod input;
mod file_loader;
mod audio;
mod messages;

use std::sync::mpsc;
use std::thread;

fn main() {
    println!("mokRadio starting...");
    
    // Create communication channels
    let (input_tx, input_rx) = mpsc::channel();
    let (file_req_tx, file_req_rx) = mpsc::channel();
    let (file_resp_tx, file_resp_rx) = mpsc::channel();
    
    // TODO: Spawn threads
    // thread::spawn(|| input::thread::run_input_thread(input_tx));
    // thread::spawn(|| file_loader::thread::run_file_loader(file_req_rx, file_resp_tx));
    // station::manager::run_station_manager(input_rx, file_req_tx, file_resp_rx);
    
    println!("mokRadio initialized (threads not yet implemented)");
}
