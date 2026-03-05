// Input Thread
// Reads ADC (tuning pot) and GPIO (AM/FM switch) and sends events


use std::sync::mpsc::Sender;
use crate::constants;
use crate::messages::InputEvent;
use crate::input::band_switch::BandSwitchPinHandler;
use crate::input::tuner::Tuner;
use rppal::gpio::Gpio;

/// Runs the input thread
/// 
/// Responsibilities:
/// - Reads ADC potentiometer continuously
/// - Monitors AM/FM GPIO switch
/// - Sends InputEvent messages to Station Manager
pub fn run_input_thread(input_sender: Sender<InputEvent>) {
    let mut tuner: Tuner = Tuner::new();
    let gpio_pins = Gpio::new().ok().unwrap();
    let mut band_switch = BandSwitchPinHandler::new(gpio_pins, constants::BAND_SWITCH_PIN);
    let mut unsent_band_events: Vec<InputEvent> = Vec::new();
    let mut unsent_tuner_events: Vec<InputEvent> = Vec::new();

    while let Err(send_error) = input_sender.send(InputEvent::DialMoved(tuner.initial_read())) {
        print!(send_error);
    }
    while let Err(send_error) = input_sender.send(InputEvent::BandSwitched { new_band: band_switch.initial_read() }) {
        print!(send_error);
    }
    
    

    loop {
        if let Some(new_band) = band_switch.read_change() {
            let input_event = InputEvent::BandSwitched { new_band };
            if let Err( send_error ) = input_sender.send(input_event){
                print!(send_error);
                unsent_band_events.push(input_event);
            }
            else {unsent_band_events.clear();}
        }
        if let Some(new_dial_position) = tuner.read_change() {
            let input_event = InputEvent::DialMoved { new_dial_position };
            if let Err( send_error ) = input_sender.send(input_event){
                print!(send_error);
                unsent_band_events.push(input_event);
            }
            else {unsent_tuner_events.clear();}
        }
    }
}


