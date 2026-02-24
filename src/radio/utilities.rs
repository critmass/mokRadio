use crate::constants;

pub fn generate_station_volume_profile() -> [f32; constants::TICKS_PER_STATION] {

    let center = (constants::TICKS_PER_STATION / 2) as f32;
    let plateau_half_width = center * 0.06;
    let steepness = 0.05 * constants::TICKS_PER_STATION as f32;
        
    std::array::from_fn(|tick| {
        // Get position within the station's band (0 to TICKS_PER_STATION)
        let x = (tick % constants::TICKS_PER_STATION) as f32;
            
        let left_tanh = ((x - (center - plateau_half_width)) / steepness).tanh();
        let right_tanh = ((x - (center + plateau_half_width)) / steepness).tanh();
            
        let volume = 0.5 * (left_tanh - right_tanh);
            
        // Round to 3 decimal places
        (volume * 1000.0).round() / 1000.0
    })
}