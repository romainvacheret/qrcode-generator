use std::usize;
use std::cmp;

use crate::encoding::alphanumeric::*;
use crate::correction::Correction;

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

// TODO: current Encoding enum and imp is quite strange, should change later
pub enum Encoding {
    ALPHANUMERIC,
}

impl Encoding {

    pub fn to_binary(&self) -> Vec<bool> {
        return match self {
            Self::ALPHANUMERIC => Alphanumeric.get_binary()
        }
    }

    pub fn get_char_count_binary(&self, string_len: usize) -> Vec<bool> {
        // TODO: create table for all versions/encoding
        // For alpha version 1
        let char_count_size = 9;
        to_binary(string_len, char_count_size)
    }

    // TODO: look at table
    // fn get_char_count_size(&self) -> usize { }
    // TODO: look at table
    // fn get_maximum_size(&self) -> usize { }
    //
    fn get_zeros_count(&self, max_size: usize, current_size: usize) -> usize {
        // `max_size` should never be greater than `current_size`
        // Pad with at most 4 zeros or less if reached the max size
        let terminator = cmp::min(max_size - current_size, 4);
        // Pad until the total size is multiple of 8
        let padding_zeros = 8 - ((current_size + terminator) % 8);

        return terminator + padding_zeros;
    }

    fn get_padding(&self, max_size: usize, current_size: usize) -> Vec<bool> {
        let padding = [
            &[true, true, true, false, true, true, false, false], // 236
            &[false, false, false, true, false, false, false, true], // 17
        ];

        let padding_count = (max_size - current_size) / 8;
        let mut result = Vec::with_capacity(padding_count * 8);

        for i in 0..padding_count {
            result.extend_from_slice(padding[i % 2]);
        }

        return result;
    }

    fn get_all_padding(&self, max_size: usize, current_size: usize) -> Vec<bool> {
        let zeros = vec![false; self.get_zeros_count(max_size, current_size)];
        let padding = self.get_padding(max_size, current_size + zeros.len());
        
        return [zeros, padding].concat();
    }


    pub fn encode_text(&self, string: String) -> Option<Vec<bool>> {
        match self {
            Self::ALPHANUMERIC => Alphanumeric.encode_text(string)
        }
    }

    pub fn encode(&self, string: String, version: usize, correction: Correction) -> Vec<bool> {
        // For alpha version 1
        let char_count_size = 9;
        // For version 1/L
        let max_size = 152;
        let data = [self.to_binary(), 
            to_binary(string.len(), char_count_size),
            // TODO: remove unchecked unwrap
            self.encode_text(string).unwrap() ].concat();
        let padding = self.get_padding(max_size, data.len());
        
        return [data, padding].concat();
    }
}



fn pad(vect: &mut Vec<bool>, expected_size: usize) {
    // TODO: Current size should never be greater than expected_size
    // but should still handle possible error
    for _ in 0..(expected_size - vect.len()) {
        vect.push(false);
    }
}

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
