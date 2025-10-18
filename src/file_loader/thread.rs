// File Loader Thread
// Loads and decodes audio files, sends them back to Station Manager

use std::sync::mpsc::{Receiver, Sender};
use std::collections::VecDeque;

/// Runs the file loader thread
/// 
/// Responsibilities:
/// - Receives file load requests (FIFO queue)
/// - Loads audio files from disk
/// - Decodes audio into rodio sources
/// - Sends decoded audio back to Station Manager
pub fn run_file_loader(
    request_rx: Receiver<FileRequest>,
    response_tx: Sender<FileResponse>
) {
    let mut request_queue: VecDeque<FileRequest> = VecDeque::new();
    
    loop {
        // Check for new requests
        while let Ok(request) = request_rx.try_recv() {
            request_queue.push_back(request);
        }
        
        // Process next request in FIFO order
        if let Some(request) = request_queue.pop_front() {
            // TODO: Load and decode file
            // TODO: Send response
        }
        
        // Small sleep to avoid busy-waiting
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
}

// Placeholder types - will be defined in messages.rs
struct FileRequest;
struct FileResponse;
