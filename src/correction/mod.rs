use crate::{version::Version};

mod reed_solomon;

#[derive(Clone, Hash, Eq, PartialEq)]
pub enum CorrectionLevel {
    L,
    M,
    Q,
    H
}

pub(crate) struct CorrectionInfo {
    pub data_codewords: usize,
    pub ec_codewords: usize,
    pub blocks: usize
}

impl CorrectionLevel {
    pub fn to_binary(&self) -> Vec<bool> {

        return match self {
            CorrectionLevel::L => vec![false, true],
            CorrectionLevel::M => vec![false, false],
            CorrectionLevel::Q => vec![true, true],
            CorrectionLevel::H => vec![true, false]
        }
    }
}

pub(crate) fn info(version: &Version, level: &CorrectionLevel) -> CorrectionInfo {
    match (version.get(), level) {
            (1, CorrectionLevel::L) => CorrectionInfo {
                data_codewords: 19,
                ec_codewords: 7,
                blocks: 1,
            },

            (1, CorrectionLevel::M) => CorrectionInfo {
                data_codewords: 16,
                ec_codewords: 10,
                blocks: 1,
            },

            (1, CorrectionLevel::Q) => CorrectionInfo {
                data_codewords: 13,
                ec_codewords: 13,
                blocks: 1,
            },

            (1, CorrectionLevel::H) => CorrectionInfo {
                data_codewords: 9,
                ec_codewords: 17,
                blocks: 1,
            },

            _ => unimplemented!(),
        }
}


pub(super) fn generate_ecc(data: Vec<u8>, info: &CorrectionInfo) -> Vec<u8> {
    reed_solomon::generate_ecc(data, info)
}
