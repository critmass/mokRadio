// Station Manager Thread
// Manages all radio stations, receives input events, sends file requests
pub mod station;
pub mod utilities;
use std::{array, path::Path, sync::mpsc::{Receiver, Sender}, thread::sleep, time::{Duration, Instant}};

use rand::seq::index;
use rodio::{OutputStream, OutputStreamBuilder, Sink, source::TrackPosition};

use station::Station;

use crate::{constants::STATION_PATH, input, messages::{FileRequest, FileResponse, InputEvent}, radio::{station::content::{Band, StationID}, utilities::{skip_dormant_stations_in_band, skip_dormant_stations_in_band_except_current}}};
use crate::messages;
use crate::constants;

pub struct Radio {
    current_station:StationID,
    current_dial_position:usize,
    last_station_switch:Instant,
    has_skipped_since_last_station_switch:bool,
    am:[Station; constants::NUMBER_OF_STATIONS],
    fm:[Station; constants::NUMBER_OF_STATIONS],
    am_volume_profile:[f32; constants::ENCODER_HALF],
    fm_volume_profile:[f32; constants::ENCODER_HALF],
    station_volume_profile:[f32; constants::TICKS_PER_STATION],
    output:OutputStream,
    white_noise: Sink
}

impl Radio {
    pub fn new (current_dial_position:usize, current_band:Band) -> Self {

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
            last_station_switch:Instant::now(),
            has_skipped_since_last_station_switch:false,
            am,
            fm,
            am_volume_profile,
            fm_volume_profile,
            station_volume_profile,
            output,
            white_noise
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
        let is_on_air = self.get_station(station_id).go_on_air();
        self.update_volume_profile(station_id, is_on_air);

        if self.current_station == station_id {
            self.tune(self.current_dial_position);
        }
    }
    pub fn station_off_air(&mut self, station_id:StationID) {
        self.update_volume_profile(station_id, false);
        self.get_station(station_id).go_off_air();
    }
    fn update_volume_profile(&mut self, station_id:StationID, on_air:bool) {
        let start = station_id.index * constants::TICKS_PER_STATION;
        let end = ( 1 + station_id.index ) * constants::TICKS_PER_STATION;
        let updated_profile = if on_air {&self.station_volume_profile}else{&[0.0f32;constants::TICKS_PER_STATION]};
        if station_id.band == Band::AM {
            self.am_volume_profile[start..end].clone_from_slice(updated_profile);
        }else{
            self.fm_volume_profile[start..end].clone_from_slice(updated_profile);
        };
    }
    pub fn tune(&mut self, new_dial_position:usize) {
        self.current_dial_position = new_dial_position;
        let station_index = new_dial_position/constants::TICKS_PER_STATION;
        if station_index != self.current_station.index {
            self.get_current_station().pause();
            self.current_station.index = station_index;
            self.get_current_station().unpause();
            self.update_skip_conditions();
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
        self.update_skip_conditions();
    }
    fn update_skip_conditions(&mut self) {
        self.has_skipped_since_last_station_switch = false;
        self.last_station_switch = Instant::now();
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
    fn get_station(&mut self, id: StationID) -> &mut Station {
        if id.band == Band::AM {
            self.am.get_mut(id.index).unwrap()
        }
        else {
            self.fm.get_mut(id.index).unwrap()
        }
    }
    pub fn run(
        &mut self, 
        input_events: Receiver<messages::InputEvent>,
        file_requester: Sender<messages::FileRequest>,
        file_returns: Receiver<messages::FileResponse>
    ) {
        self.prime_stations(&file_requester);
        println!("radio on and ready");
        loop {
            while let Ok(input_event) = input_events.try_recv() {
                self.resolve_input_event(input_event);
                sleep(constants::KNOB_DELAY);
            }
            if let Ok(file_response) = file_returns.try_recv(){
                self.handle_file_return(file_response);
            }
            if self.get_current_station().is_on_air() {self.manage_current_station(&file_requester);}
            if !self.has_skipped_since_last_station_switch && self.last_station_switch.elapsed() > constants::TIME_BETWEEN_SKIPS {
                self.skip_dormant_stations(&file_requester);
                self.has_skipped_since_last_station_switch = true;
            }
            sleep(constants::LOOP_DELAY);
        }
        
    }
    fn manage_current_station( &mut self, file_requester: &Sender<messages::FileRequest> ) {
        let current_station = self.get_current_station();
        if current_station.needs_next() {
            if let Some(file_path) = current_station.next() {

                let request = FileRequest::LoadTrack { 
                    station_id: self.current_station, 
                    file_path
                };
                file_requester.send(request);
            }
        }
    }
    fn resolve_input_event(&mut self, input_event:InputEvent) {
        match input_event {
            InputEvent::DialMoved { new_dial_position } => {
                self.tune(new_dial_position);
            },
            InputEvent::BandSwitched { new_band } => {
                self.switch_band(new_band);
            }
        }
    }
    fn handle_file_return(&mut self, file_response:FileResponse) {
        match file_response {
            FileResponse::TrackLoaded { station_id, audio_content } => {
                self.get_station(station_id).push_to_sink(audio_content);
                self.station_on_air(station_id);
                
            },
            _ => {}
        }
    }
    fn prime_stations(&mut self, file_requester: &Sender<messages::FileRequest>) {
        self.am.iter_mut().enumerate().for_each(|(index, station)| {
            station.prime_content().iter().for_each(|request_path| {
                let request = FileRequest::LoadTrack { 
                    station_id: StationID { band: Band::AM, index }, 
                    file_path: request_path.clone()
                };
                file_requester.send(request);
            });
        });
        self.fm.iter_mut().enumerate().for_each(|(index, station)| {
            station.prime_content().iter().for_each(|request_path| {
                let request = FileRequest::LoadTrack { 
                    station_id: StationID { band: Band::FM, index }, 
                    file_path: request_path.clone()
                };
                file_requester.send(request);
            });
        });
    }
    fn skip_dormant_stations(&mut self, file_requester: &Sender<messages::FileRequest>) {
        match self.current_station.band {
            Band::AM => {
                skip_dormant_stations_in_band_except_current(
                    &mut self.am, 
                    &file_requester, Band::AM, 
                    self.current_station.index
                );
                skip_dormant_stations_in_band(
                    &mut self.fm, 
                    &file_requester, 
                    Band::FM
                );
            },
            Band::FM => {
                skip_dormant_stations_in_band_except_current(
                    &mut self.fm, 
                    &file_requester, Band::FM, 
                    self.current_station.index
                );
                skip_dormant_stations_in_band(
                    &mut self.am, 
                    &file_requester, 
                    Band::AM
                );
            }
        }
    }    
}

