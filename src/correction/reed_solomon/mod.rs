use crate::correction::{CorrectionInfo, reed_solomon::polynomial::{Polynomial, divide_polynoms}};

mod polynomial;
mod gf256;


pub(super) fn generate_ecc(data: Vec<u8>, info: &CorrectionInfo) -> Vec<u8> {
    let poly = Polynomial::new(data);
    let generator = get_generator_polynomial(info.ec_codewords);

    divide_msg_poly(poly, generator).into_values()
}

fn get_generator_polynomial(ecc_count: usize) -> polynomial::Polynomial {
    match ecc_count {
        7 => polynomial::Polynomial::new(vec![
            1, 127, 122, 154, 164, 11, 68, 117
        ]),

        10 => polynomial::Polynomial::new(vec![
            1, 216, 194, 159, 111, 199,
            94, 95, 113, 157, 193
        ]),

        13 => polynomial::Polynomial::new(vec![
            1, 137, 73, 227, 17, 177, 17,
            52, 13, 46, 43, 83, 132, 120
        ]),

        17 => polynomial::Polynomial::new(vec![
            1, 119, 66, 83, 120, 119,
            22, 197, 83, 249, 41, 143,
            134, 85, 53, 125, 99, 79
        ]),

        _ => unreachable!(),
    }
}

fn divide_msg_poly(mut msg: Polynomial, gen: Polynomial) -> Polynomial {
    msg.shift_by(gen.degree());
    divide_polynoms(msg, &gen)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_full_division_version_1m() {
        // HELLO WORLD, Version 1-M:
        // 16 data codewords
        let message = Polynomial::new(vec![
            32, 91, 11, 120,
            209, 114, 220, 77,
            67, 64, 236, 17,
            236, 17, 236, 17,
        ]);

        // Degree-10 QR Reed-Solomon generator
        let generator = Polynomial::new(vec![
            1, 216, 194, 159, 111, 199,
            94, 95, 113, 157, 193,
        ]);

        let remainder = divide_msg_poly(message, generator.clone());

        assert_eq!(
            remainder,
            polynomial::Polynomial::new( vec![
                196, 35, 39, 119, 235,
                215, 231, 226, 93, 23,
            ])
        );

        // Generator has degree 10
        // Remainder must have degree < 10
        assert!(remainder.degree() < generator.degree());
    }
}
