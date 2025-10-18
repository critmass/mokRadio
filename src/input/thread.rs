// Input Thread
// Reads ADC (tuning pot) and GPIO (AM/FM switch) and sends events

use std::sync::mpsc::Sender;

/// Runs the input thread
/// 
/// Responsibilities:
/// - Reads ADC potentiometer continuously
/// - Monitors AM/FM GPIO switch
/// - Sends InputEvent messages to Station Manager
pub fn run_input_thread(tx: Sender<InputEvent>) {
    // TODO: Initialize ADC and GPIO
    // TODO: Main loop
    //   - Read ADC value
    //   - Read AM/FM switch
    //   - Send events when values change
}

// Placeholder - will be defined in messages.rs
struct InputEvent;
