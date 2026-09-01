use crate::{correction::reed_solomon::{get_generator_polynomial, polynomial::{self, NotationMode, Polynomial, divide_message_polynomial}}, utils::pad_until, version::Version};

mod reed_solomon;

#[derive(Clone, Hash, Eq, PartialEq)]
pub enum CorrectionLevel {
    L,
    M,
    Q,
    H
}

pub struct CorrectionInfo {
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

pub fn info(version: &Version, level: &CorrectionLevel) -> CorrectionInfo {
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


pub fn generate_ecc(data: Vec<u16>, info: &CorrectionInfo) -> Vec<u16> {
        let mut poly = Polynomial::new(NotationMode::Decimal, data);
        let mut other = get_generator_polynomial(info.ec_codewords);

        divide_message_polynomial(&mut poly, &mut other);

        poly.values
}


/// Generate error code bits for the format.
///
/// `format` should be 5 bits long starting with error code level then
/// mask pattern. 
/// Eg. 01 (indicator for error correction level L)
///     100 (binary for 4, i.e. mask pattern 4)
///     => 01100
///
/// Followed steps:
///     - Pad the format to reach 15 bits
///     - Remove the zeros on the left of the format
///     - Perform division
///         - Pad the generator to have the same length as the format
///             - Reset the generator to default after each iteration 
///         - XOR format to the generator
///         - Remove the zeros on the left of the format
///         - Repeat until the format is no more than 10 bits long 
///     - Pad the format if less than 10 bits long
fn generate_error_code_bits(format: &Vec<bool>) -> Vec<bool> {
    // From documentation, corresponds to x^10 + x^8 + x^5 + x^4 + x^2 + x + 1
    let polynomial = vec![true, false, true, false, false, true, true, false, true, true, true];
    let mut result = format.clone();
    pad_until(&mut result , 15);
    // TODO refactor
    result =  result.into_iter().skip_while(|&b| !b).collect();

    println!("Format {:?}", result);
    println!("Gen {:?}\n", polynomial);

    while result.len() > 10 {
        let mut poly = polynomial.clone();
        pad_until(&mut poly, result.len());
        println!("Padded gen {:?}", poly);
        result = result.iter()
            .zip(poly.iter())
            .map(|(&x, &y)| x ^ y)
            .collect();
        println!("Xored format {:?}", result);
        result =  result.into_iter().skip_while(|&b| !b).collect();
        println!("Trimmed format {:?} {}\n", result, result.len());
    }

    // pad(&mut result, 10);
    while result.len() < 10 {
        result.insert(0, false);
    }


    return result;
}

/// Generate the format string for the correction.
///
/// `format` should be 5 bits long starting with error code level then
/// mask pattern. 
/// Eg. 01 (indicator for error correction level L)
///     100 (binary for 4, i.e. mask pattern 4)
///     => 01100
///
/// Followed steps:
///     - Get the error code bits
///     - XOR with the generator
pub fn generate_format_string(format: &Vec<bool>) -> Vec<bool> {
    let error_code_bits = generate_error_code_bits(&format);
    let mut result = [format.clone(), error_code_bits].concat();
    // From documentation: 101010000010010
    let mask_string = vec![true, false, true, false, true, false, false, false, false, false, true, false, false, true, false];
    // TODO: refactor the XOR operation
    result = result.iter()
        .zip(mask_string.iter())
        .map(|(&x, &y)| x ^ y)
        .collect();

    return result;
}

#[cfg(test)]
mod tests {
    use crate::correction::{self, polynomial::divide_message_polynomial, generate_error_code_bits, generate_format_string, reed_solomon::get_generator_polynomial, CorrectionLevel, polynomial::Polynomial};


    #[test]
    pub fn test_divide_message_polynomial_1l() {
        // "HELLO WORLD", Version 1-L
        // 19 data codewords
        let vect = vec![
            32, 91, 11, 120, 209, 114, 220, 77, 67, 64,
            236, 17, 236, 17, 236, 17, 236, 17, 236
        ];

        let mut poly = Polynomial::new(correction::polynomial::NotationMode::Decimal, vect);
        let mut other = get_generator_polynomial(7);

        let expected_result = Polynomial::new(
            correction::polynomial::NotationMode::Decimal,
            vec![209, 239, 196, 207, 78, 195, 109],
        );

        divide_message_polynomial(&mut poly, &mut other);

        assert_eq!(poly, expected_result);
    }

    #[test]
    pub fn test_divide_message_polynomial() {
        let vect = vec![
            32, 91, 11, 120, 209, 114, 220, 77, 
            67, 64, 236, 17, 236, 17, 236, 17
        ];
        let mut poly = Polynomial::new(correction::polynomial::NotationMode::Decimal, vect);
        let mut other = get_generator_polynomial(10);

        let expected_result = Polynomial::new(
            correction::polynomial::NotationMode::Decimal, 
            vec![196, 35, 39, 119, 235, 215, 231, 226, 93, 23]
        );

        divide_message_polynomial(&mut poly, &mut other);

        assert_eq!(poly, expected_result);
    }


    #[test]
    pub fn test_divide_message_polynomial_1q() {
        // "HELLO WORLD", Version 1-Q
        // 13 data codewords
        let vect = vec![
            32, 91, 11, 120, 209, 114, 220,
            77, 67, 64, 236, 17, 236
        ];

        let mut poly = Polynomial::new(correction::polynomial::NotationMode::Decimal, vect);
        let mut other = get_generator_polynomial(13);

        let expected_result = Polynomial::new(
            correction::polynomial::NotationMode::Decimal,
            vec![168, 72, 22, 82, 217, 54, 156, 0, 46, 15, 180, 122, 16],
        );

        divide_message_polynomial(&mut poly, &mut other);

        assert_eq!(poly, expected_result);
    }


    #[test]
    pub fn test_divide_message_polynomial_1h() {
        // "HELLO WORL", Version 1-H
        // 9 data codewords
        let vect = vec![
            32, 83, 11, 120, 209, 114, 220, 77, 64
        ];

        let mut poly = Polynomial::new(correction::polynomial::NotationMode::Decimal, vect);
        let mut other = get_generator_polynomial(17);

        let expected_result = Polynomial::new(
            correction::polynomial::NotationMode::Decimal,
            vec![
                55, 122, 139, 105, 131, 8, 19, 170, 240,
                233, 77, 132, 155, 46, 33, 53, 158
            ],
        );

        divide_message_polynomial(&mut poly, &mut other);

        assert_eq!(poly, expected_result);
    }

    #[test]
    pub fn test_generate_error_code_bits() {
        // For L and Mask 4
        let format = vec![false, true, true, false, false];
        let result = generate_error_code_bits(&format);
        // The expected result is incorrect imo
        let expected_result = vec![true, false, false, false, true, true, true, true, false, true];

        assert_eq!(result, expected_result);
    }

    #[test]
    pub fn test_generate_format_string_l4() {
        // For L and Mask 4
        let format = vec![false, true, true, false, false];
        let result = generate_format_string(&format);
        let expected_result = vec![true, true, false, false, true, true, false, false, false, true, false, true, true, true, true];

        assert_eq!(result, expected_result);
    }

    #[test]
    pub fn test_generate_format_string_m2() {
        // For M and Mask 2
        let format = vec![false, false, false, true, false];
        let result = generate_format_string(&format);
        let expected_result = vec![true, false, true, true, true, true, false, false, true, true, true, true, true, false, false];

        assert_eq!(result, expected_result);
    }

}
