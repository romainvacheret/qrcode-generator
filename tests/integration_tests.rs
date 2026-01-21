extern crate qrcodegen;

use qrcodegen::encoding::Encoding;
use qrcodegen::correction::Correction;
use qrcodegen::qrcode::QRCode;


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

    assert_eq!(expected, result, "1M Alphanumeric is incorrect for `HELLO WORLD`");
}
