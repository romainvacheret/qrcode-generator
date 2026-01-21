use std::usize;

use crate::encoding::alphanumeric::*;
use crate::qrcode::version::Version;

pub mod alphanumeric;

pub trait Encode {
    fn get_binary(&self) -> Vec<bool>;
    fn encode_text(&self, string: String) -> Option<Vec<bool>>;
}

fn vec_to_binary(chunk: &[bool]) -> u16 {
    
    return chunk.iter().fold(0, |acc, &b| (acc << 1) | (b as u16));
}

pub fn to_decimal(encoding: &Vec<bool>) -> Vec<u16> {
    return encoding.chunks(8)
        .map(|chunk| vec_to_binary(&chunk))
        .collect();
}

pub enum Encoding {
    ALPHANUMERIC,
}

impl Encoding {

    fn to_binary(&self) -> Vec<bool> {
        return match self {
            Self::ALPHANUMERIC => Alphanumeric.get_binary()
        }
    }

    fn get_char_count_binary(&self, string_len: usize, version: &Version) -> Vec<bool> {
        // TODO: create table for all versions/encoding
        // For alpha version 1
        to_binary(string_len, version.get_char_count(self))
    }

    fn encode_text(&self, string: String) -> Option<Vec<bool>> {
        match self {
            Self::ALPHANUMERIC => Alphanumeric.encode_text(string)
        }
    }

    pub fn encode(&self, string: String, version: &Version) -> Vec<bool> {
        [self.to_binary(), 
            self.get_char_count_binary(string.len(), version),
            // TODO: remove unchecked unwrap
            self.encode_text(string).unwrap()
        ].concat()
    }
}

fn pad(vect: &mut Vec<bool>, expected_size: usize) {
    // TODO: Current size should never be greater than expected_size
    // but should still handle possible error
    for _ in 0..(expected_size - vect.len()) {
        vect.push(false);
    }
}

// TODO: refactor
fn to_binary(mut val: usize, expected_size: usize) -> Vec<bool> {
    let mut result = Vec::<bool>::new();

    while val > 0 {
        result.push(val % 2 == 1);
        val /=  2;
    }

    pad(&mut result, expected_size);
    result.reverse();
    return result;
}


// TODO: add back
// #[cfg(test)]
// mod tests {
//     #[test]
//     fn test_get_all_padding() {
//         let expected = vec![false,false,false,false,false,false, 
//             true,true,true,false,true,true,false,false, 
//             false,false,false,true,false,false,false,true, 
//             true,true,true,false,true,true,false,false];
//         let result = Encoding::get_all_padding(104, 74);
//         assert_eq!(result, expected);
//     }
// }
