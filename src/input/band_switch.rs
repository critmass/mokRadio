use rppal::gpio::{Gpio, InputPin};
use crate::radio::station::content::Band;

pub struct BandSwitchPinHandler {
    pin: InputPin,
    current_band: Band
}

impl BandSwitchPinHandler {
    pub fn new(gpio_pins: Gpio, pin_number: u8) -> BandSwitchPinHandler {
        let pin = gpio_pins.get(pin_number).ok().unwrap().into_input();
        let current_band = if pin.is_high() {Band::AM} else {Band::FM};
        BandSwitchPinHandler { pin, current_band }
    }
    pub fn initial_read(&self) -> Band {
        self.current_band
    }
    pub fn read_change(&mut self) -> Option<Band> {
        let band = if self.pin.is_high() {
            Band::AM}
            else {Band::FM};
        if band != self.current_band {
            self.current_band = band;
            Some(band)
        }
        else {None}
    }
}