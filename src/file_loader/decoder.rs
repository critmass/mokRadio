// Audio file loading and decoding
// Loads MP3 files and decodes them for rodio

use std::path::Path;
use std::fs::File;
use std::io::BufReader;
use rodio::Decoder;

/// Loads and decodes an audio file
/// 
/// Returns a rodio Decoder that can be appended to a Sink
pub fn load_and_decode(path: &Path) -> Result<Decoder<BufReader<File>>, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let decoder = Decoder::new(BufReader::new(file))?;
    Ok(decoder)
}
