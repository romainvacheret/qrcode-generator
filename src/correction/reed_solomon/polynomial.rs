use crate::correction::reed_solomon::gf256::{self, get_log_antilog};


#[derive(PartialEq, Eq, Clone, Debug)]
pub enum NotationMode {
    Decimal,
    Alpha
}


#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Polynomial {
    pub mode: NotationMode,
    pub exponent: u16,
    pub values : Vec<u16>
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct DensePolynomial {
    pub values : Vec<u8>,
    pub shift: usize
}

pub fn div_poly(mut fst: DensePolynomial, snd: &DensePolynomial) -> DensePolynomial {
    while fst.degree() >= snd.degree() {
        fst = div_step(fst, snd.clone());
    }

    fst
}

fn div_step(fst: DensePolynomial, mut snd: DensePolynomial) -> DensePolynomial {
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

impl DensePolynomial {
    pub fn new(values: Vec<u8>) -> Self {
        DensePolynomial { values, shift: 0 }
    }

    pub fn degree(&self) -> usize {
        self.shift + self.values.len() - 1
    }

    pub fn shift_by(&mut self, n: usize) {
        self.shift += n;
    }

    fn scale(&self, factor: u8) -> Self {
        DensePolynomial { 
            values: self.values
                .iter()
                .map(|&v| gf256::mul(v, factor))
                .collect(),
            shift: self.shift 
        }
    }

fn sub(&self, other: &DensePolynomial) -> Self {
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

    DensePolynomial {
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
}

impl Polynomial {
    pub fn new(notation: NotationMode, values: Vec<u16>) -> Self {
        return Polynomial { mode: notation, exponent: values.len() as u16, values: values}
    }

    pub fn convert(&mut self) {
        let idx = if self.mode == NotationMode::Alpha {1} else {3};
        let log_antilog = get_log_antilog();

        println!("convertion {}", log_antilog[5][idx]);
        self.values.iter_mut().for_each(|v| *v = log_antilog[usize::from(*v)][idx]);
        self.mode = if self.mode == NotationMode::Alpha { NotationMode::Decimal } else { NotationMode::Alpha }
    }

    pub fn xor(&mut self, other: &Self) {
        self.values
            .iter_mut()
            .enumerate()
            // Only xor up to size of generator
            .take_while(|(idx, _v)| *idx < other.values.len())
            .for_each(|(idx, v)| { *v ^= other.values[idx]; if *v > 255 { *v %= 255 }; });

        let len_diff = other.values.len() as i8 - self.values.len() as i8;
        if len_diff > 0 {
            // Each iteration increases the size, idx related to initial one
            let initial_len = self.values.len();
            for i  in 0..len_diff {
                self.values.push(other.values[initial_len + usize::try_from(i).unwrap()]);
            }
        }
    }

    // TODO: ensure is in alpha notation
    
    pub fn to_string(&self) -> String {
        return self.values
            .iter()
            .enumerate()
            .map(|(idx, v)| format!("{}{}x^{}", 
                if self.mode == NotationMode::Alpha { "a^" } else { "" } , 
                v, self.exponent - idx as u16 - 1))
            .collect::<Vec<String>>()
            .join("+");
    }
}

pub fn divide_message_polynomial(message: &mut Polynomial, generator: &Polynomial) {
    let mut generator_copy = generator.clone();

    if message.mode !=NotationMode::Decimal {
        message.convert();
    }

    if generator_copy.mode !=NotationMode::Alpha {
        generator_copy.convert();
    }

    println!("Step 0");
    let nb_codewords  = (generator_copy.values.len() - 1) as u16;
    message.exponent += nb_codewords;
    // let diff_exponent = message.exponent - generator_copy.exponent;
    // generator_copy.exponent += diff_exponent;
    // let iterations = message.values.len() as u16 - nb_codewords + 1;
    let iterations = message.values.len() + 1;
    println!("message {}", message.to_string());
    println!("generator {}", generator_copy.to_string());
    // Correct until here
    
    for idx in 1..iterations {
    // for idx in 1..4 {
        generator_copy = if idx > 1 { generator.clone() } else { generator_copy };
        let diff_exponent = message.exponent - generator_copy.exponent;
        generator_copy.exponent += diff_exponent;
        println!("Step {}a", idx);
        
        // Coefficient to be converted
        let first_coef = message.values[0];

        // If leading coefficient is zero no division to perform
        if first_coef == 0 {
            message.values.remove(0);
            message.exponent -= 1;
            continue;
        }

        let first_alpha = get_log_antilog()[usize::from(first_coef)][3];

        generator_copy.values.iter_mut().for_each(|v| {
            *v += first_alpha;
            if *v > 255 { *v %= 255};
        });

        generator_copy.convert();
        println!("generator {}", generator_copy.to_string());
        println!("message {}", message.to_string());
        println!("Step {}b", idx);
        message.xor(&generator_copy);
        println!("message {}", message.to_string());
        message.values.remove(0);
        message.exponent -= 1;
        println!("message {}", message.to_string());
        generator_copy.convert();
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exponent_without_shift() {
        // 5x^2 + 7x + 9
        let poly = DensePolynomial::new(vec![5, 7, 9]);

        assert_eq!(poly.degree(), 2);
    }

    #[test]
    fn test_exponent_with_shift() {
        // Initially:
        // 5x^2 + 7x + 9
        //
        // After mul by x^5:
        // 5x^7 + 7x^6 + 9x^5
        let mut poly = DensePolynomial::new(vec![5, 7, 9]);

        poly.shift_by(5);

        assert_eq!(poly.degree(), 7);
        assert_eq!(poly.shift, 5);
        assert_eq!(poly.values, vec![5, 7, 9]);
    }

    #[test]
    fn test_scale_polynomial() {
        // Version 1-M generator in normal GF(256) values
        let generator = DensePolynomial::new(vec![
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
        let mut poly = DensePolynomial::new(vec![1, 216, 194]);

        poly.shift_by(10);

        let scaled = poly.scale(32);

        assert_eq!(scaled.shift, 10);
        assert_eq!(scaled.values, vec![32, 2, 101]);
    }

    #[test]
    fn test_sub_same_leading_exponent_preserves_tail() {
        // 10x^3 + 20x^2 + 30x + 40
        let fst = DensePolynomial {
            values: vec![10, 20, 30, 40],
            shift: 0,
        };

        // x^3 + 2x^2
        let snd = DensePolynomial {
            values: vec![1, 2],
            shift: 2,
        };

        let result = fst.sub(&snd);

        assert_eq!(
            result,
            DensePolynomial {
                values: vec![11, 22, 30, 40],
                shift: 0,
            }
        );
    }

    #[test]
    fn test_check_removes_leading_zero() {
        // 0x^4 + 5x^3 + 7x^2
        let poly = DensePolynomial {
            values: vec![0, 5, 7],
            shift: 2,
        };

        let result = poly.check();

        assert_eq!(
            result,
            DensePolynomial {
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
        let poly = DensePolynomial {
            values: vec![0, 0, 5, 7],
            shift: 2,
        };

        let result = poly.check();

        assert_eq!(
            result,
            DensePolynomial {
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
        let message = DensePolynomial {
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
        let generator = DensePolynomial::new(vec![
            1, 216, 194, 159, 111, 199,
            94, 95, 113, 157, 193,
        ]);

        let result = div_step(message, generator);

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
            DensePolynomial {
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
        let mut message = DensePolynomial::new(vec![
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

        message.shift_by(generator.degree());

        let remainder = div_poly(
            message,
            &generator,
        );

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
