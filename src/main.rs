// mokRadio - Vintage Radio with Modern Playlists
// A Raspberry Pi project to turn a vintage radio into a playlist player

mod radio;
mod input;
mod file_loader;
mod audio;
mod messages;

use std::fs::File;
use std::path::{PathBuf};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

use rodio::Decoder;
use crate::messages::{FileRequest, FileResponse, InputEvent};

fn main() {
    println!("mokRadio starting...");
    
    // Create communication channels
    let (input_tx, input_rx):
        (Sender<InputEvent>,Receiver<InputEvent>) = channel();
    let (file_request_tx, file_request_rx):
        (Sender<FileRequest>, Receiver<FileRequest>) = channel();
    let (file_response_tx, file_response_rx):
        (Sender<FileResponse>, Receiver<FileResponse>) = channel();
    
    // TODO: Spawn threads
    // thread::spawn(|| input::thread::run_input_thread(input_tx));
    // thread::spawn(|| file_loader::thread::run_file_loader(file_req_rx, file_resp_tx));
    // station::manager::run_station_manager(input_rx, file_req_tx, file_resp_rx);
    
    println!("mokRadio initialized (threads not yet implemented)");
}
