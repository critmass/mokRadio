use rppal::i2c::{Error, I2c};

use crate::constants;

pub struct Tuner {
    rotary_encoder:I2c,
    buffer: [u8; 2]
}

impl Tuner {
    pub fn new() -> Self {
        let rotary_encoder = I2c::new().ok().unwrap();
        let buffer = [0u8; 2];
        Tuner {rotary_encoder, buffer}
    }
    pub fn initial_read(&mut self) -> Result<usize, Error> {
        Result(self.read_change())
    }
    pub fn read_change(&mut self) -> Result<Option<usize>, Error> {
        let write_buffer = [constants::LEADING_REGISTER,constants::LEADING_REGISTER+1];
        let mut read_buffer = [0u8; 2];
        if let Err(read_error) = self.rotary_encoder.write_read(&write_buffer, &mut read_buffer) {
            print!("Tuner Error: {}",read_error);
            return Result(read_error);
        }
        if read_buffer != self.buffer {
            self.buffer = read_buffer;
            let top = (self.buffer[0] as u16) << 6;
            let bottom = (self.buffer[1] as u16) >> 2;
            let value = (top | bottom) as usize;
            Result(Some(value))
        }
        else {Result(None)}
    }
}