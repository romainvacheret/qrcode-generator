use qrcodegen::encoding::Encoding;
use qrcodegen::correction::{divide_message_polynomial, get_generator_polynomial, NotationMode, Polynomial, Correction};
use qrcodegen::qrcode::QRCode;

fn exec_correction() {

    let vect = vec![32, 91, 11, 120, 209, 114, 220, 77, 67, 64, 236, 17, 236, 17, 236, 17];
    let mut poly = Polynomial::new(NotationMode::DECIMAL, vect);
    let mut other = get_generator_polynomial(0);

    divide_message_polynomial(&mut poly, &mut other);
}


pub fn get_padding(count: usize) -> Vec<bool> {
    // From documentation: 236 and 17
    let first = vec![true, true, true, false, true, true, false, false];
    let second = vec![false, false, false, true, false, false, false, true];

    (0..count).flat_map(|idx| if idx % 2 == 0 { first.clone() } else { second.clone() })
        .collect()
}




fn first_diff(a: &str, b: &str) -> Option<usize> {
    a.chars()
        .zip(b.chars())
        .position(|(x, y)| x != y)
        .or_else(|| {
            if a.len() != b.len() {
                Some(a.chars().count().min(b.chars().count()))
            } else {
                None
            }
        })
}
fn show_diff(a: &str, b: &str) {
    let pos = first_diff(a, b).unwrap_or(0);

    println!("A: {}", a);
    println!("B: {}", b);
    println!("   {}^", " ".repeat(pos));
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
    let encoding = Encoding::ALPHANUMERIC;
    // No other correction at the moment
    let correction = Correction::M;
    let mut qrcodee = QRCode::new(message.to_string(), encoding, correction);
    qrcodee.assemble();
    let as_string = qrcodee.as_string();

    // let expected = "# # # # # # # - - # - - # - # # # # # # #\n# - - - - - # - - # # # # - # - - - - - #\n# - # # # - # - # # - - # - # - # # # - #\n# - # # # - # - # - # # - - # - # # # - #\n# - # # # - # - # # - # # - # - # # # - #\n# - - - - - # - # # # - # - # - - - - - #\n# # # # # # # - # - # - # - # # # # # # #\n- - - - - - - - # - - # # - - - - - - - -\n # - # # # # # - - - # - # - # # # # # - -\n# - - - # # - # # - - - # # - - # # # # #\n# - - - - # # # - - # # - - - # - # - - #\n# - - - - - - - # - - - - - - # - - - - -\n# - - # - # # # # - # # - - - - - - # - -\n- - - - - - - - # - # # # # # - - # - # #\n# # # # # # # - - # # - # - # - # # # - #\n# - - - - - # - # # # # # # # # - - # # -\n# - # # # - # - # - # - # - - - - # # # -\n# - # # # - # - # - # - # - - # - # # - -\n# - # # # - # - # - - # - # - - # # - - -\n# - - - - - # - - - - - - - - - - - # - #\n# # # # # # # - # - # # - # - - # - - - -\n".to_string();

    let expected = r"# # # # # # # - - # - - # - # # # # # # # 
# - - - - - # - - # # # # - # - - - - - # 
# - # # # - # - # # - - # - # - # # # - # 
# - # # # - # - # - # # - - # - # # # - # 
# - # # # - # - # # - # # - # - # # # - # 
# - - - - - # - # # # - # - # - - - - - # 
# # # # # # # - # - # - # - # # # # # # # 
- - - - - - - - # - - # # - - - - - - - - 
# - # # # # # - - - # - # - # # # # # - - 
# - - - # # - # # - - - # # - - # # # # # 
# - - - - # # # - - # # - - - # - # - - # 
# - - - - - - - # - - - - - - # - - - - - 
# - - # - # # # # - # # - - - - - - # - - 
- - - - - - - - # - # # # # # - - # - # # 
# # # # # # # - - # # - # - # - # # # - # 
# - - - - - # - # # # # # # # # - - # # - 
# - # # # - # - # - # - # - - - - # # # - 
# - # # # - # - # - # - # - - # - # # - - 
# - # # # - # - # - - # - # - - # # - - - 
# - - - - - # - - - - - - - - - - - # - # 
# # # # # # # - # - # # - # - - # - - - - 
".to_string();

    println!("{}",expected == as_string);
    println!("{} {}", expected.len() , as_string.len());

    print!("===\n{}===\n", expected);
    print!("===\n{}===\n", as_string);

    // println!("{}", first_diff(&expected, &as_string).unwrap());
    // show_diff(&expected, &as_string);
}
