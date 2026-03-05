use std::time::Duration;



pub const NUMBER_OF_STATIONS: usize = 12; 
pub const ENCODER_MAX: usize = 8192;
pub const TICKS_PER_STATION: usize = ENCODER_MAX / NUMBER_OF_STATIONS / 2;
pub const ENCODER_HALF: usize = TICKS_PER_STATION * NUMBER_OF_STATIONS;
pub const STATION_PATH: &'static str = "/stations";
pub const TIME_BETWEEN_SKIPS: Duration = Duration::new(300, 0);
pub const KNOB_DELAY: Duration = Duration::new(0, 3000000);
pub const LOOP_DELAY: Duration = Duration::new(0, 10000000);
pub const LEADING_REGISTER : u8 = 0x03;
pub const BAND_SWITCH_PIN : u8 = 4;