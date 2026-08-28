extern crate qrcodegen;

use qrcodegen::encoding::Encoding;
use qrcodegen::correction::Correction;
use qrcodegen::qrcode::QRCode;


#[test]
fn test_qrcode_1l_alpha() {
    let message = "HELLO WORLD";
    let encoding = Encoding::ALPHANUMERIC;
    let correction = Correction::L;
    let mut qrcodee = QRCode::new(message.to_string(), encoding, correction);
    qrcodee.assemble();

    let result = qrcodee.as_string();
    let expected = r"# # # # # # # - - # - - # - # # # # # # # 
# - - - - - # - # - - # - - # - - - - - # 
# - # # # - # - - # - - - - # - # # # - # 
# - # # # - # - # - - # - - # - # # # - # 
# - # # # - # - - - # # # - # - # # # - # 
# - - - - - # - # # # - # - # - - - - - # 
# # # # # # # - # - # - # - # # # # # # # 
- - - - - - - - - - # # # - - - - - - - - 
# # # # # - # # # # - - # # - # - # - # - 
- - # - # - - # # - # - # # - - # # # # # 
# # - # - - # # # - - # - - - # - # - - # 
- # - # # # - - - # - - - - - # - - - - - 
- - # - - # # # # - - # - - - - - - # - - 
- - - - - - - - # - # # # # # - - # - # # 
# # # # # # # - # # # - # - # - # # # - # 
# - - - - - # - - - # # # # # # - - # # - 
# - # # # - # - # - # - # - - - - # # # - 
# - # # # - # - # - # - # - - # - # # - - 
# - # # # - # - # # - # - # - - # # - - - 
# - - - - - # - # - - - - - - - - - # - # 
# # # # # # # - # # - # - # - - # - - - - 
".to_string();

    assert_eq!(
        expected,
        result,
        "1L Alphanumeric is incorrect for `HELLO WORLD`"
    );
}


#[test]
fn test_qrcode_1m_alpha() {
    let message = "HELLO WORLD";
    let encoding = Encoding::ALPHANUMERIC;
    let correction = Correction::M;
    let mut qrcodee = QRCode::new(message.to_string(), encoding, correction);
    qrcodee.assemble();

    let result = qrcodee.as_string();
    let expected = r"# # # # # # # - - # - - # - # # # # # # # 
# - - - - - # - - # # # # - # - - - - - # 
# - # # # - # - # # - - # - # - # # # - # 
# - # # # - # - # - # # - - # - # # # - # 
# - # # # - # - # # - # # - # - # # # - # 
# - - - - - # - # # # - # - # - - - - - # 
# # # # # # # - # - # - # - # # # # # # # 
- - - - - - - - # - - # # - - - - - - - - 
# - # # # # # - - - # - # - # # # # # - - 
# - # # # # - # # - - - # # - - # # # # # 
- - # - - # # # - - # # - - - # - # - - # 
- - # # - - - - # - - - - - - # - - - - - 
- # # # - # # # # - # # - - - - - - # - - 
- - - - - - - - # - # # # # # - - # - # # 
# # # # # # # - - # # - # - # - # # # - # 
# - - - - - # - # # # # # # # # - - # # - 
# - # # # - # - # - # - # - - - - # # # - 
# - # # # - # - # - # - # - - # - # # - - 
# - # # # - # - # - - # - # - - # # - - - 
# - - - - - # - - - - - - - - - - - # - # 
# # # # # # # - # - # # - # - - # - - - - 
".to_string();

    assert_eq!(expected, result, "1M Alphanumeric is incorrect for `HELLO WORLD`");
}



#[test]
fn test_qrcode_1q_alpha() {
    let message = "HELLO WORLD";
    let encoding = Encoding::ALPHANUMERIC;
    let correction = Correction::Q;
    let mut qrcodee = QRCode::new(message.to_string(), encoding, correction);
    qrcodee.assemble();

    let result = qrcodee.as_string();
    let expected = r"# # # # # # # - # - # - - - # # # # # # # 
# - - - - - # - - - - - # - # - - - - - # 
# - # # # - # - - # # # # - # - # # # - # 
# - # # # - # - - - - # # - # - # # # - # 
# - # # # - # - # # - - - - # - # # # - # 
# - - - - - # - # - # # # - # - - - - - # 
# # # # # # # - # - # - # - # # # # # # # 
- - - - - - - - - - - # # - - - - - - - - 
- # # # # # # # - # # - # - - # # - - - # 
# - - - - # - # # # # - # # - - # # # # # 
- - - - # # # # # - - - - - - # - # - - # 
# - # - # - - - - - # - # - - # - - - - - 
# - # # - - # - - # - # # - - - - - # - - 
- - - - - - - - # # - - # # # - - # - # # 
# # # # # # # - # # - - - - # - # # # - # 
# - - - - - # - # # - - - # # # - - # # - 
# - # # # - # - # # - - - - - - - # # # - 
# - # # # - # - # # - - # - - # - # # - - 
# - # # # - # - # # # # - # - - # # - - - 
# - - - - - # - # - # - - - - - - - # - # 
# # # # # # # - - # # # - # - - # - - - - 
".to_string();

    assert_eq!(
        expected,
        result,
        "1Q Alphanumeric is incorrect for `HELLO WORLD`"
    );
}


#[test]
fn test_qrcode_1h_alpha() {
    // Version 1-H supports at most 10 alphanumeric characters
    let message = "HELLO WORL";
    let encoding = Encoding::ALPHANUMERIC;
    let correction = Correction::H;
    let mut qrcodee = QRCode::new(message.to_string(), encoding, correction);
    qrcodee.assemble();

    let result = qrcodee.as_string();
    let expected = r"# # # # # # # - # - # - - - # # # # # # # 
# - - - - - # - # - # - - - # - - - - - # 
# - # # # - # - # # - - - - # - # # # - # 
# - # # # - # - - # - - - - # - # # # - # 
# - # # # - # - - - # # - - # - # # # - # 
# - - - - - # - # # # - # - # - - - - - # 
# # # # # # # - # - # - # - # # # # # # # 
- - - - - - - - # # # # # - - - - - - - - 
- - # # # - # - # - - - # # # # - - # # # 
# # # # - - - - # - - - # - - # - # # # # 
- - # # - # # # # # - - - # # # - # - - # 
- # # - - - - - # - # - # # - # - - - - - 
# # - # # - # - - - - - # # # - - - # - - 
- - - - - - - - # # # # - # - - - # - # # 
# # # # # # # - - # - - # # # - # # # - - 
# - - - - - # - - - - - # - # # - - # # - 
# - # # # - # - # # - - - - # - - # # # - 
# - # # # - # - # # # # # - # # - # # - - 
# - # # # - # - # - - - - - - - # # - - - 
# - - - - - # - - # # - - - # - - - # - # 
# # # # # # # - - - # # # # # - # - - - - 
".to_string();

    assert_eq!(
        expected,
        result,
        "1H Alphanumeric is incorrect for `HELLO WORL`"
    );
}
