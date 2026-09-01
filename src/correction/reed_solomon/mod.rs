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
