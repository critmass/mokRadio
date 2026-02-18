

pub const NUMBER_OF_STATIONS: usize = 12; 
pub const ENCODER_MAX: usize = 8192;
pub const TICKS_PER_STATION: usize = ENCODER_MAX / NUMBER_OF_STATIONS / 2;
pub const ENCODER_HALF: usize = TICKS_PER_STATION * NUMBER_OF_STATIONS;
pub const STATION_PATH: &'static str = "/stations";
