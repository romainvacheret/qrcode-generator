use crate::correction::reed_solomon::polynomial::{DensePolynomial, div_poly};

// TODO: should not be public later
pub mod polynomial;
pub mod gf256;

// At the moment, only handle alphanumeric version 1
pub fn get_generator_polynomial(ecc_count: usize) -> polynomial::Polynomial {
    return match ecc_count {
        7 => polynomial::Polynomial::new(
            polynomial::NotationMode::Alpha,
            vec![0, 87, 229, 146, 149, 238, 102, 21]
        ),
        10 => polynomial::Polynomial::new(
            polynomial::NotationMode::Alpha,
            vec![0, 251, 67, 46, 61, 118, 70, 64, 94, 32, 45]
        ),
        13 => polynomial::Polynomial::new(
            polynomial::NotationMode::Alpha,
            vec![0, 74, 152, 176, 100, 86, 100, 106, 104, 130, 218, 206, 140, 78]
        ),
        17 => polynomial::Polynomial::new(
            polynomial::NotationMode::Alpha,
            vec![0, 43, 139, 206, 78, 43, 239, 123, 206, 214, 147, 24, 99, 150, 39, 243, 163, 136]
        ),
        _ => unreachable!()
    }
}


pub fn get_generator_polynomial2(ecc_count: usize) -> polynomial::DensePolynomial {
    match ecc_count {
        7 => polynomial::DensePolynomial::new(vec![
            1, 127, 122, 154, 164, 11, 68, 117
        ]),

        10 => polynomial::DensePolynomial::new(vec![
            1, 216, 194, 159, 111, 199,
            94, 95, 113, 157, 193
        ]),

        13 => polynomial::DensePolynomial::new(vec![
            1, 137, 73, 227, 17, 177, 17,
            52, 13, 46, 43, 83, 132, 120
        ]),

        17 => polynomial::DensePolynomial::new(vec![
            1, 119, 66, 83, 120, 119,
            22, 197, 83, 249, 41, 143,
            134, 85, 53, 125, 99, 79
        ]),

        _ => unreachable!(),
    }
}

pub fn divide_msg_poly(mut msg: DensePolynomial, gen: DensePolynomial) -> DensePolynomial {
    msg.shift_by(gen.degree());
    div_poly(msg, &gen)
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_full_division_version_1m() {
        // HELLO WORLD, Version 1-M:
        // 16 data codewords
        let message = DensePolynomial::new(vec![
            32, 91, 11, 120,
            209, 114, 220, 77,
            67, 64, 236, 17,
            236, 17, 236, 17,
        ]);

        // Degree-10 QR Reed-Solomon generator
        let generator = DensePolynomial::new(vec![
            1, 216, 194, 159, 111, 199,
            94, 95, 113, 157, 193,
        ]);

        let remainder = divide_msg_poly(message, generator.clone());

        assert_eq!(
            remainder,
            DensePolynomial {
                values: vec![
                    196, 35, 39, 119, 235,
                    215, 231, 226, 93, 23,
                ],
                shift: 0,
            }
        );

        // Generator has degree 10
        // Remainder must have degree < 10
        assert!(remainder.degree() < generator.degree());
    }
}
