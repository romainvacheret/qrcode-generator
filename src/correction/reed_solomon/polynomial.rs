use crate::correction::reed_solomon::gf256::{self};


#[derive(PartialEq, Eq, Clone, Debug)]
pub(super) struct Polynomial {
    values : Vec<u8>,
    shift: usize
}

pub(super) fn divide_polynoms(mut fst: Polynomial, snd: &Polynomial) -> Polynomial {
    while fst.degree() >= snd.degree() {
        fst = perform_div_step(fst, snd.clone());
    }

    fst
}

fn perform_div_step(fst: Polynomial, mut snd: Polynomial) -> Polynomial {
    // Exponent alignment
    snd.shift_by(fst.degree() - snd.degree());

    let factor = gf256::div(
        *fst.values.first().unwrap(),
        *snd.values.first().unwrap()
    );
    let scaled = snd.scale(factor);

    fst
        .sub(&scaled)
        .check()
}

impl Polynomial {
    pub fn new(values: Vec<u8>) -> Self {
        Polynomial { values, shift: 0 }
    }

    pub fn degree(&self) -> usize {
        self.shift + self.values.len() - 1
    }

    pub fn shift_by(&mut self, n: usize) {
        self.shift += n;
    }

    fn scale(&self, factor: u8) -> Self {
        Polynomial { 
            values: self.values
                .iter()
                .map(|&v| gf256::mul(v, factor))
                .collect(),
            shift: self.shift 
        }
    }

    fn sub(&self, other: &Polynomial) -> Self {
        assert_eq!(self.degree(), other.degree());

        let mut values = self.values.clone();

        // XOR the coefficients that overlap.
        for (fst, snd) in values.iter_mut().zip(other.values.iter()) {
            *fst = gf256::sub(*fst, *snd);
        }

        // If `other` extends to lower exponents than `self`,
        // append those coefficients.
        if other.shift < self.shift {
            let extra = self.shift - other.shift;

            values.extend_from_slice(
                &other.values[other.values.len() - extra..]
            );
        }

        Polynomial {
            values,
            shift: self.shift.min(other.shift),
        }
    }

    fn check(mut self) -> Self {
        while self.values.len() > 1
            && self.values.first().is_some_and(|&val| val == 0)
        {
            self.values.remove(0);
        }

        self
    }

    pub(super) fn into_values(self) -> Vec<u8> {
        self.values
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exponent_without_shift() {
        // 5x^2 + 7x + 9
        let poly = Polynomial::new(vec![5, 7, 9]);

        assert_eq!(poly.degree(), 2);
    }

    #[test]
    fn test_exponent_with_shift() {
        // Initially:
        // 5x^2 + 7x + 9
        //
        // After mul by x^5:
        // 5x^7 + 7x^6 + 9x^5
        let mut poly = Polynomial::new(vec![5, 7, 9]);

        poly.shift_by(5);

        assert_eq!(poly.degree(), 7);
        assert_eq!(poly.shift, 5);
        assert_eq!(poly.values, vec![5, 7, 9]);
    }

    #[test]
    fn test_scale_polynomial() {
        // Version 1-M generator in normal GF(256) values
        let generator = Polynomial::new(vec![
            1, 216, 194, 159, 111, 199,
            94, 95, 113, 157, 193,
        ]);

        // Scale by 32 in GF(256)
        let scaled = generator.scale(32);

        assert_eq!(
            scaled.values,
            vec![
                32, 2, 101, 10, 97, 197,
                15, 47, 134, 74, 5,
            ]
        );

        // Scaling must not affect the exponent shift
        assert_eq!(scaled.shift, 0);
    }

    #[test]
    fn test_scale_preserves_shift() {
        let mut poly = Polynomial::new(vec![1, 216, 194]);

        poly.shift_by(10);

        let scaled = poly.scale(32);

        assert_eq!(scaled.shift, 10);
        assert_eq!(scaled.values, vec![32, 2, 101]);
    }

    #[test]
    fn test_sub_same_leading_exponent_preserves_tail() {
        // 10x^3 + 20x^2 + 30x + 40
        let fst = Polynomial {
            values: vec![10, 20, 30, 40],
            shift: 0,
        };

        // x^3 + 2x^2
        let snd = Polynomial {
            values: vec![1, 2],
            shift: 2,
        };

        let result = fst.sub(&snd);

        assert_eq!(
            result,
            Polynomial {
                values: vec![11, 22, 30, 40],
                shift: 0,
            }
        );
    }

    #[test]
    fn test_check_removes_leading_zero() {
        // 0x^4 + 5x^3 + 7x^2
        let poly = Polynomial {
            values: vec![0, 5, 7],
            shift: 2,
        };

        let result = poly.check();

        assert_eq!(
            result,
            Polynomial {
                values: vec![5, 7],
                shift: 2,
            }
        );

        assert_eq!(result.degree(), 3);
    }

    #[test]
    fn test_check_removes_multiple_leading_zeros() {
        // 0x^5 + 0x^4 + 5x^3 + 7x^2
        //
        // Should normalize to:
        //
        // 5x^3 + 7x^2
        let poly = Polynomial {
            values: vec![0, 0, 5, 7],
            shift: 2,
        };

        let result = poly.check();

        assert_eq!(
            result,
            Polynomial {
                values: vec![5, 7],
                shift: 2,
            }
        );

        assert_eq!(result.degree(), 3);
    }

    #[test]
    fn test_first_division_step_version_1m() {
        // HELLO WORLD data codewords, already multiplied by x^10.
        //
        // Leading term:
        // 32x^25
        let message = Polynomial {
            values: vec![
                32, 91, 11, 120,
                209, 114, 220, 77,
                67, 64, 236, 17,
                236, 17, 236, 17,
            ],
            shift: 10,
        };

        // Version 1-M generator:
        //
        // x^10 + 216x^9 + ... + 193
        let generator = Polynomial::new(vec![
            1, 216, 194, 159, 111, 199,
            94, 95, 113, 157, 193,
        ]);

        let result = perform_div_step(message, generator);

        // First step:
        //
        // generator shifted to degree 25
        // generator scaled by 32
        // XOR performed
        // leading zero removed
        //
        // New leading degree = 24
        assert_eq!(
            result,
            Polynomial {
                values: vec![
                    89, 110, 114, 176,
                    183, 211, 98, 197,
                    10, 233, 17, 236,
                    17, 236, 17,
                ],
                shift: 10,
            }
        );

        assert_eq!(result.degree(), 24);
    }

    #[test]
    fn test_full_division_version_1m() {
        // HELLO WORLD, Version 1-M:
        // 16 data codewords
        let mut message = Polynomial::new(vec![
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

        message.shift_by(generator.degree());

        let remainder = divide_polynoms(
            message,
            &generator,
        );

        assert_eq!(
            remainder,
            Polynomial {
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
