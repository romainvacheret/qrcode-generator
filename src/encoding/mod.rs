use crate::error::Error;
use crate::version::Version;
use crate::utils::bin::to_binary;

mod alphanumeric;

// TODO: remove once QRCode struct has been refactored
#[derive(Clone)]
pub enum EncodingMode {
    Alphanumeric,
}

pub struct Segment {
    mode: EncodingMode,
    character_count: usize,
    data: Vec<bool>
}

// NOTE: required for other encoding modes (byte and kanji)
struct EncodedData {
    count: usize,
    bits: Vec<bool>
}

impl EncodingMode {
    fn to_binary(&self) -> Vec<bool> {
        return match self {
            Self::Alphanumeric => Vec::<bool>::from([false, false, true, false])
        }
    }

    fn get_char_count_bits(&self, version: &Version) -> usize {
        match (self, version.get()) {
            // (EncodingMode::Numeric,      1..=9)  => 10,
            // (EncodingMode::Numeric,     10..=26) => 12,
            // (EncodingMode::Numeric,     27..=40) => 14,

            (EncodingMode::Alphanumeric, 1..=9)  => 9,
            (EncodingMode::Alphanumeric,10..=26) => 11,
            (EncodingMode::Alphanumeric,27..=40) => 13,

            // (EncodingMode::Byte,         1..=9)  => 8,
            // (EncodingMode::Byte,        10..=40) => 16,
            //
            // (EncodingMode::Kanji,        1..=9)  => 8,
            // (EncodingMode::Kanji,       10..=26) => 10,
            // (EncodingMode::Kanji,       27..=40) => 12,

            _ => unreachable!(),
        }
    }
}

pub fn encode(text: &str, mode: EncodingMode) -> Result<Segment, Error> {
    let enconded_data = match mode {
        EncodingMode::Alphanumeric => alphanumeric::encode(text)?
    };

    Ok(Segment { 
        mode, 
        character_count: enconded_data.count,
        data: enconded_data.bits
    })
}

impl Segment {
    pub fn to_bits(&self, version: &Version) -> Vec<bool> {
        let count_size = self.mode.get_char_count_bits(version);

        [
            self.mode.to_binary(),
            to_binary(self.character_count, count_size),
            self.data.clone()
        ].concat()
    }
}
