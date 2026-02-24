// Station Manager Thread
// Manages all radio stations, receives input events, sends file requests
pub mod station;
pub mod utilities;
use std::{array, path::Path, sync::mpsc::{Receiver, Sender}};

use rodio::{OutputStream, OutputStreamBuilder, Sink};

use station::Station;

use crate::{constants::STATION_PATH, radio::station::content::{Band, StationID}};
use crate::messages;
use crate::constants;

struct Radio {
    current_station:StationID,
    current_dial_position:usize,
    am:[Station; constants::NUMBER_OF_STATIONS],
    fm:[Station; constants::NUMBER_OF_STATIONS],
    am_volume_profile:[f32; constants::ENCODER_HALF],
    fm_volume_profile:[f32; constants::ENCODER_HALF],
    station_volume_profile:[f32; constants::TICKS_PER_STATION],
    output:OutputStream,
    white_noise: Sink,
    input_events: Receiver<messages::InputEvent>,
    file_requester: Sender<messages::FileRequest>,
    file_returns: Receiver<messages::FileResponse>
}

impl Radio {
    pub fn new (
        current_dial_position:usize, 
        current_band:Band, 
        input_events: Receiver<messages::InputEvent>,
        file_requester: Sender<messages::FileRequest>,
        file_returns: Receiver<messages::FileResponse> 
    ) -> Self {

        let output_builder = OutputStreamBuilder::from_default_device().unwrap();
        let output = output_builder.open_stream().unwrap();

        let am = Radio::initialize_station_array(Band::AM, &output);
        let fm = Radio::initialize_station_array(Band::FM, &output);
        
        let station_volume_profile = utilities::generate_station_volume_profile();
        let am_volume_profile = Radio::initialize_volume_profile(
            &am,
            &station_volume_profile
        );
        let fm_volume_profile = Radio::initialize_volume_profile(
            &fm,
            &station_volume_profile
        );
        
        let white_noise = Sink::connect_new(output.mixer());
        white_noise.set_volume( 
            if current_band == Band::AM { 1.0 - am_volume_profile.get(current_dial_position).unwrap() }
            else { 1.0 - fm_volume_profile.get(current_dial_position).unwrap() }
        );

        let radio = Radio {
            current_station: StationID {
                band: current_band,
                index: current_dial_position / constants::TICKS_PER_STATION,
            },
            current_dial_position,
            am,
            fm,
            am_volume_profile,
            fm_volume_profile,
            station_volume_profile,
            output,
            white_noise,
            input_events,
            file_requester,
            file_returns
        };

        radio
    }
    fn initialize_station_array( 
        band: Band,
        output: &OutputStream
    ) -> [Station; constants::NUMBER_OF_STATIONS] {

        let station_array = array::from_fn(|station_number: usize| {
            let station_path_string = format!(
                "{}/{:?}/{:02}/",
                STATION_PATH,
                band,
                station_number
            );
            let station_path = Path::new(&station_path_string);
            if station_path.exists() {
                Station::new(station_path, output)
            } else {
                Station::new_dead(station_path)
            }
        });

        station_array
    }
    fn initialize_volume_profile(
        band:&[Station; constants::NUMBER_OF_STATIONS],
        station_volume_profile: &[f32; constants::TICKS_PER_STATION]
    ) -> [f32; constants::ENCODER_HALF] {

        let mut volume_profile = [0.0f32; constants::ENCODER_HALF];

        band.iter().enumerate().for_each(|(i, station)| {
            if !station.is_on_air() {return;};
            station_volume_profile.iter().enumerate().for_each(|(j, value)| {
                volume_profile[ i * constants::TICKS_PER_STATION + j ] = *value;
            });
        });

        volume_profile
    }
    pub fn station_on_air(&mut self, station_id:StationID) {
        let station = self.get_station(&station_id);
        station.go_on_air();

        if self.current_station == station_id {

        }
    }
    pub fn tune(&mut self, new_dial_position:usize) {
        self.current_dial_position = new_dial_position;
        let station_index = new_dial_position/constants::TICKS_PER_STATION;
        if station_index != self.current_station.index {
            self.get_current_station().pause();
            self.current_station.index = station_index;
            self.get_current_station().unpause();
        }
        let volume = self.get_station_volume();
        self.get_current_station().set_volume(volume);
        self.white_noise.set_volume(1.0 - volume);
    }
    pub fn switch_band(&mut self, new_band: Band) {
        self.get_current_station().pause();
        self.current_station.band = new_band;
        let volume = self.get_station_volume();
        self.white_noise.set_volume(1.0 - volume);
        let current_station = self.get_current_station();
        current_station.set_volume(volume);
        current_station.unpause();
    }
    fn get_station_volume(&self) -> f32 {
        if self.current_station.band == Band::AM {
            self.am_volume_profile[self.current_dial_position]
        }
        else{
            self.fm_volume_profile[self.current_dial_position]
        }
    }
    fn get_current_station(&mut self) -> &mut Station {
        if self.current_station.band == Band::AM {
            self.am.get_mut(self.current_station.index).unwrap()
        }
        else {
            self.fm.get_mut(self.current_station.index).unwrap()
        }
    }
    fn get_station(&mut self, id: &StationID) -> &mut Station {
        if id.band == Band::AM {
            self.am.get_mut(id.index).unwrap()
        }
        else {
            self.fm.get_mut(id.index).unwrap()
        }

    }
}

