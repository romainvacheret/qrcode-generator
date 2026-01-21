use std::usize;

use crate::encoding::alphanumeric::Alphanumeric;
use crate::qrcode::version::Version;
use crate::utils::bin::to_binary;

pub mod alphanumeric;

pub trait Encode {
    fn get_binary(&self) -> Vec<bool>;
    fn encode_text(&self, string: String) -> Option<Vec<bool>>;
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
