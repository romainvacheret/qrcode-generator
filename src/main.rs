use crate::correction::{divide_message_polynomial, get_generator_polynomial, NotationMode, Polynomial};

mod masking;
mod utils;
mod patterns;
mod encoding;
mod correction;
mod qrcode;

fn exec_correction() {

    let vect = vec![32, 91, 11, 120, 209, 114, 220, 77, 67, 64, 236, 17, 236, 17, 236, 17];
    let mut poly = Polynomial::new(correction::NotationMode::DECIMAL, vect);
    let mut other = get_generator_polynomial(0);

    divide_message_polynomial(&mut poly, &mut other);
}

fn to_bits(n: u16) -> Vec<bool> {
    (0..8)
        .rev() // start from most significant bit
        .map(|i| (n & (1 << i)) != 0)
        .collect()
}


pub fn get_padding(count: usize) -> Vec<bool> {
    // From documentation: 236 and 17
    let first = vec![true, true, true, false, true, true, false, false];
    let second = vec![false, false, false, true, false, false, false, true];

    (0..count).flat_map(|idx| if idx % 2 == 0 { first.clone() } else { second.clone() })
        .collect()
}

fn main() {
    // let mut matrix = Matrix::new(21 as usize);
    //
    // PatternHelper::new(false).apply_patterns(&mut matrix, 1);
    // matrix.display();
    //
    //
    // let message = "hello world!";
    // let encoding = encoding::Encoding::ALPHANUMERIC;
    // let opt_res = encoding::alphanumeric::encode_alpha(message.to_string());
    // let res = opt_res.unwrap();
    // let decimal = encoding::to_decimal(&res);
    // let mut poly = Polynomial::new(correction::NotationMode::DECIMAL, decimal);
    // let mut other = get_generator_polynomial(0);
    //
    // divide_message_polynomial(&mut poly, &mut other);
    //
    // if poly.mode != NotationMode::DECIMAL {
    //     poly.convert();
    // }
    //
    // let correction: Vec<bool> = poly.values.iter()
    //     .flat_map(|val| to_bits(*val))
    //     .collect();
    //
    //
    //
    // let res = get_padding(6);
    // println!("{} {:?}", res.len(), res);

    let message = "HELLO WORLD";
    // No other encoding available at the moment
    let encoding = encoding::Encoding::ALPHANUMERIC;
    // No other correction at the moment
    let correction = correction::Correction::M;
    let mut qrcodee = qrcode::QRCode::new(message.to_string(), encoding, correction);
    qrcodee.assemble();
    qrcodee.display();
}
