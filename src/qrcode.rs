use crate::{correction::{self, divide_message_polynomial, generate_format_string, get_generator_polynomial, Correction, NotationMode, Polynomial}, encoding::{self, alphanumeric::Alphanumeric, Encode, Encoding}, masking::{self, Mask}, patterns::PatternHelper, qrcode::version::Version, to_bits, utils::{self, print_as_binary, Matrix, Pos}};

// TODO: reformat with other pad functions encoding/correction
fn pad(size: usize) -> Vec<bool> {
    (0..size).map(|_| false).collect()
}

pub struct QRCode {
    data: Matrix,
    pattern: Matrix,
    message: String,
    version: Version,
    encoding: Encoding,
    correction: Correction
}

impl QRCode {
    pub fn new(message: String, encoding: Encoding, correction: Correction) -> Self {
        let version = version::Version::new(1).unwrap();
        let mut qrcode = QRCode {
            data: Matrix::new(version.get_size()),
            pattern: Matrix::new(version.get_size()),
            message: message,
            version: version,
            encoding: encoding,
            correction: correction,
        };

        qrcode.pattern.display();

        // PatternHelper::new(true).apply_patterns(&mut qrcode.pattern, qrcode.version);
        // TODO: pass version 
        PatternHelper::new(true).apply_patterns2(&mut qrcode.pattern, &qrcode.version);

        qrcode.pattern.display();

        return qrcode;
    }

    fn is_pattern(&self, pos: &Pos) -> bool {
        // self.pattern.display();
        // println!("{:?}", pos);
        return self.pattern.get(pos).unwrap().clone();
    }

    fn get_next_valid_idx(&self, pos: &mut Pos, is_upward: &mut bool) -> bool {
        loop {
            let res = self.data.get_next_idx(pos, is_upward);
            // println!("Res {:?}", res);

            if !res || !self.is_pattern(pos) {
                return res;
            }
        }
    }

    pub fn fill_matrix(&mut self, content: Vec<bool>) {
        // Start in right lower corner
        let mut current = Pos::new(self.data.size - 1, self.data.size - 1);
        let mut is_upward = true;
        let size = content.len();

        self.data.data.get_mut(current.row).unwrap()[current.col] = *content.first().unwrap();

        for (idx, bit) in content.iter().skip(1).enumerate() {
            // println!("HERE {:?} {} {}", current, size, idx);
            let status = self.get_next_valid_idx(&mut current, &mut is_upward);
            println!("{} {} {}", current.row, current.col, *bit);
            if !status {
                panic!("Invalid idx");
            }

            self.data.data.get_mut(current.row).unwrap()[current.col] = *bit;
        }
    }

    pub fn get_padding(&self, count: usize) -> Vec<bool> {
        // From documentation: 236 and 17
        let first = vec![true, true, true, false, true, true, false, false];
        let second = vec![false, false, false, true, false, false, false, true];

        (0..count).flat_map(|idx| if idx % 2 == 0 { first.clone() } else { second.clone() })
            .collect()
    }

    pub fn add_format_string(&mut self, format_string: Vec<bool>) {
        // Each time horizontal then vertical one
        for i in 0..6 {
            // First 5 bits
            self.data.data[8][i] = format_string[i].clone();
            self.data.data[self.data.size - i - 1][8] = format_string[i];

            // 9th bit and following
            self.data.data[8][self.data.size - 6 + i] = format_string[9 + i].clone();
            self.data.data[5 - i][8] = format_string[9 + i];
        }

        // 6th bit
        self.data.data[8][7] = format_string[6].clone();
        self.data.data[self.data.size - 7][8] = format_string[6];

        // 7th bit
        self.data.data[8][self.data.size - 8] = format_string[7].clone();
        self.data.data[8][8] = format_string[7];

        // 8th bit
        self.data.data[8][self.data.size - 7] = format_string[8].clone();
        self.data.data[7][8] = format_string[8];
    }

    fn pad_full_encoding(&self, message: &mut Vec<bool>, max_length: usize) {
        // `max_length` should never be greater than `current_size`
        // Pad with at most 4 zeros or less if reached the max size
        let terminator_size = std::cmp::min(max_length - message.len(), 4);
        let terminator = pad(terminator_size);

        message.extend(terminator);

        let padding_mul_8 = if message.len() < max_length {
            // Make the size a multiple of 8
            pad(8 - message.len() % 8)
        } else {
            vec![]
        };

        message.extend(padding_mul_8);

        let padding = self.get_padding((max_length - message.len()) / 8);

        message.extend(padding);
    }


    pub fn assemble(&mut self) {
        let mut data_string = self.encoding.encode(self.message.to_string(), &self.version);
        self.pad_full_encoding(&mut data_string, 16 * 8);

        // NOTE: data_strint correct content until here 
        println!("SIZE {} {:?}", data_string.len(), data_string);
        utils::print_as_binary(&data_string, 8);

        let decimal = encoding::to_decimal(&data_string);
        let mut poly = Polynomial::new(correction::NotationMode::DECIMAL, decimal);
        println!("Poly {:?}", poly);
        let mut other = get_generator_polynomial(0);

        divide_message_polynomial(&mut poly, &mut other);

        if poly.mode != NotationMode::DECIMAL {
            poly.convert();
        }
        println!("Poly {:?}", poly);

        // let pad_count = max_size.abs_diff(res.len()) / 8;

        let correction_bits: Vec<bool> = poly.values.iter()
            .flat_map(|val| to_bits(*val))
            .collect();

        println!("Correction bits {:?}", correction_bits);

        data_string.extend(correction_bits);
        // ------
        // TODO: detect the most relevant one
        let mask = Mask::MASK2;

        let mut format_string = generate_format_string(&[
            self.correction.to_binary(),
            mask.to_binary()
        ].concat());
        // TODO: should reverse the index in `add_format_string`
        // format_string.reverse();
        // let format_string = vec![true, true, false, false, true, true, false, false, false, true, false, true, true, true, true];
        // let format_string = vec![true, true, true, true, true, true, true, true, true, true, true, true, true, true, true];


        print_as_binary(&data_string, 8);
        println!("data_string {}", data_string.len());
        println!("data_string {:?}", data_string);
        println!("format_string {:?}", format_string);
        // println!("{:?}", self.data);

        self.fill_matrix(data_string);
        mask.apply(&mut self.data.data);
        // TODO: pass version not usize
        PatternHelper::new(false).apply_patterns(&mut self.data, &self.version);
        self.add_format_string(format_string);
    }

    pub fn display(&self) {
        self.data.display();
    }
}

pub mod version {
    use crate::encoding::Encoding;

    pub struct Version(usize);

    impl Version {
        pub fn new(value: usize) -> Option<Self> {
            if (1..=40).contains(&value) {
                Some(Version(value))
            } else {
                None
            }
        }

        pub fn get(&self) -> usize {
            return self.0
        }

        // https://www.thonky.com/qr-code-tutorial/data-encoding
        pub fn get_char_count(&self, encoding: &Encoding) -> usize {
            match encoding {
                Encoding::ALPHANUMERIC => match self.0 {
                    1..=9 => 9,
                    10..=26 => 11,
                    27..=40 => 13,
                    _ => unreachable!()
                }
            }
        }

        pub fn get_size(&self) -> usize {
            match self.0 {
                1 => 21,
                _ => panic!("Not implemented yet")
            }
        }

    }
}
