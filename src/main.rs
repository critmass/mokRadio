// mokRadio - Vintage Radio with Modern Playlists
// A Raspberry Pi project to turn a vintage radio into a playlist player

mod radio;
mod input;
mod file_loader;
mod messages;
mod constants;

use std::fs::File;
use std::path::{PathBuf};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use crate::radio::Radio;
use crate::radio::station::content::Band;

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

    thread::spawn(|| input::thread::run_input_thread(input_tx));
    thread::spawn(|| file_loader::thread::run_file_loader(file_request_rx, file_response_tx));
        
    let current_dial_position= input::rotary_encoder_reader;
    let current_band= Band::AM;
        
    let mut _radio_ = Radio::new(current_dial_position, current_band);
}
