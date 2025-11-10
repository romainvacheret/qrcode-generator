use std::usize;
use std::cmp;

use crate::encoding::alphanumeric::encode_alpha;
use crate::correction::Correction;

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

type EncodingFunc = fn(String) -> Option<Vec<bool>>;

impl Encoding {

    pub fn to_binary(&self) -> Vec<bool> {
        return match self {
            Self::ALPHANUMERIC => Vec::<bool>::from([false, false, true, false])
        }
    }

    pub fn get_char_count_binary(&self, string_len: usize) -> Vec<bool> {
        // TODO: create table for all versions/encoding
        // For alpha version 1
        let char_count_size = 9;
        to_binary(string_len, char_count_size)
    }

    fn get_encoding_func(&self) -> EncodingFunc {
        return match self {
            Self::ALPHANUMERIC => encode_alpha
        }
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

    pub fn encode(&self, string: String, version: usize, correction: Correction) -> Vec<bool> {
        let encoding_func = self.get_encoding_func();
        // For alpha version 1
        let char_count_size = 9;
        // For version 1/L
        let max_size = 152;
        let data = [self.to_binary(), 
            to_binary(string.len(), char_count_size),
            // TODO: remove unchecked unwrap
            encoding_func(string).unwrap()].concat();
        let padding = self.get_padding(max_size, data.len());
        
        return [data, padding].concat();
    }
}


pub mod alphanumeric {
    use crate::encoding::to_binary;

    fn get_alpha_value(character: char) -> Option<usize> {
        let characters = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ $%*+-./:";
        return characters.chars().position(|r| r == character);
    }

    fn encode_alpha_pair(char1: char, char2: char) -> Option<usize> {
        let opt_first_encoding = get_alpha_value(char1);

        // TODO: handle btter
        if opt_first_encoding.is_none() {
            panic!("ERROR: Invalid char");
        }

        let first_encoding = opt_first_encoding.unwrap();

        return Some(if char2 != '\0' { 
            first_encoding * 45 + get_alpha_value(char2)?
        } else { first_encoding });
    }

    fn encode_alpha_values(string: String) -> Option<Vec<usize>> {
        let mut result_vec = Vec::<usize>::new();

        for i in (0..string.len()).step_by(2) {
            let char1 = string.chars().nth(i)?;
            let char2 = if i < string.len() -1 { string.chars().nth(i + 1)? } else { '\0' };
            result_vec.push(encode_alpha_pair(char1, char2)?);
        }

        return Some(result_vec);
    }

    pub fn encode_alpha(string: String) -> Option<Vec<bool>> {
        const ALPHA_PAIR_SIZE: usize = 11;
        const ALPHA_SINGLE_SIZE: usize = 6;
        let current_size = string.len();

        return encode_alpha_values(string).map(|vec| {
            let mut result = Vec::<bool>::new(); 
            for (idx, &val) in vec.iter().enumerate() {
                // If is last element and odd number of chars
                let size = if (idx == vec.len() - 1) && (current_size % 2 == 1) {
                    ALPHA_SINGLE_SIZE
                } else {
                    ALPHA_PAIR_SIZE
                };

                result = [result, to_binary(val, size)].concat();
            }
            return result;
        })
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

#[cfg(test)]
mod tests {
    use crate::encoding::{alphanumeric, Encoding};

    #[test]
    fn test_encode_alpha() {
        let expected_value = Vec::from([false,false,true,true,true,false,false,true,true,true,false, 
            true,true,true,false,false,true,true,true,false,false,true,
            false,false,false,false,true,false]);
        let result = alphanumeric::encode_alpha(String::from("AC-42")).unwrap();

        assert_eq!(result, expected_value, "Results do no match. Got: {:?} and expected {:?}", result, expected_value);
    }

    #[test]
    fn test_encode_alpha2() {
        let expected_value = vec![false,true,true,false,false,false,false,true,false,true,true, false,true,true,true,true,false,false,false,true,true,false, true,false,false,false,true,false,true,true,true,false,false, true,false,true,true,false,true,true,true,false,false,false, true,false,false,true,true,false,true,false,true,false,false, false,false,true,true,false,true];
        let result = alphanumeric::encode_alpha(String::from("HELLO WORLD")).unwrap();

        assert_eq!(result, expected_value, "Results do no match. Got: {:?} and expected {:?}", result, expected_value);
    }

    #[test]
    fn test_get_all_padding() {
        let expected = vec![false,false,false,false,false,false, 
            true,true,true,false,true,true,false,false, 
            false,false,false,true,false,false,false,true, 
            true,true,true,false,true,true,false,false];
        let result = Encoding::ALPHANUMERIC.get_all_padding(104, 74);

        assert_eq!(result, expected);

    }
}
