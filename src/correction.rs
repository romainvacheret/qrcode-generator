use std::{collections::HashMap, fs::File, io::{self, BufRead, BufReader}, iter::Map, sync::Once};

#[derive(Hash, Eq, PartialEq)]
pub enum Correction {
    L,
    M,
    Q,
    H
}

impl Correction {
    pub fn to_binary(&self) -> Vec<bool> {
        return match self {
            Correction::L => vec![false, true],
            Correction::M => vec![false, false],
            Correction::Q => vec![true, true],
            Correction::H => vec![true, false]
        }
    }
    
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum NotationMode {
    DECIMAL,
    ALPHA
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Polynomial {
    pub mode: NotationMode,
    pub exponent: u16,
    pub values : Vec<u16>
}

impl Polynomial {
    pub fn new(notation: NotationMode, values: Vec<u16>) -> Self {
        return Polynomial { mode: notation, exponent: values.len() as u16, values: values}
    }

    pub fn convert(&mut self) {
        let idx = if self.mode == NotationMode::ALPHA {1} else {3};
        let log_antilog = get_log_antilog();

        println!("convertion {}", log_antilog[5][idx]);
        self.values.iter_mut().for_each(|v| *v = log_antilog[usize::from(*v)][idx]);
        self.mode = if self.mode == NotationMode::ALPHA { NotationMode::DECIMAL} else { NotationMode::ALPHA }
    }

    pub fn xor(&mut self, other: &Polynomial) {
        self.values
            .iter_mut()
            .enumerate()
            // Only xor until size of generator
            .take_while(|(idx, _v)| *idx < other.values.len())
            .for_each(|(idx, v)| { *v ^= other.values[idx]; if *v > 255 { *v %= 255 }; });

        let len_diff = other.values.len() as i8 - self.values.len() as i8;
        if len_diff > 0 {
            // Each iteration increases the size, idx related to inital length
            let initial_len = self.values.len();
            for i  in 0..len_diff {
                self.values.push(other.values[initial_len + usize::try_from(i).unwrap()]);
                // self.values.push(other.values[self.values.len() + usize::try_from(i).unwrap()]);
            }
        }
    }

    
    pub fn to_string(&self) -> String {
        return self.values
            .iter()
            .enumerate()
            .map(|(idx, v)| format!("{}{}x^{}", 
                if self.mode == NotationMode::ALPHA { "a^" } else { "" } , 
                v, self.exponent - idx as u16 - 1))
            .collect::<Vec<String>>()
            .join("+");
    }
    
}

fn get_log_antilog() -> &'static Vec<Vec<u16>> {
    static INIT: Once = Once::new();
    static mut DATA: Option<Vec<Vec<u16>>> = None;

    INIT.call_once(|| {
        let file = File::open("log-antilog.csv").expect("Cannot open file");
        let reader = BufReader::new(file);

        let content = reader
            .lines()
            .skip(1) // skip header
            .filter_map(Result::ok)
            .map(|line| line.split(',')
                            .map(|s| s.parse::<u16>().unwrap())
                            .collect::<Vec<u16>>())
            .collect::<Vec<Vec<u16>>>();

        unsafe { DATA = Some(content) };
    });

    unsafe { DATA.as_ref().unwrap() }
}


// At the moment, only handle 1M
pub fn get_generator_polynomial(size: u16) -> Polynomial {
    return Polynomial::new(NotationMode::ALPHA,vec![0, 251, 67, 46, 61, 118, 70, 64, 94, 32, 45]);
}


pub fn divide_message_polynomial(message: &mut Polynomial, generator: &Polynomial) {
    let mut generator_copy = generator.clone();
    if message.mode != NotationMode::DECIMAL {
        message.convert();
    }

    if generator_copy.mode != NotationMode::ALPHA {
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
        // TODO: Unnecessary to convert the entire polynomial
        message.convert();
        println!("message {}", message.to_string());
        println!("generator {}", generator_copy.to_string());
        // generator_copy.values.iter_mut().for_each(|v| {*v += message.values[0]; if *v == 255 {println!("256 {}", idx)}; if *v > 255 { *v %= 255 }; });
        generator_copy.values.iter_mut().for_each(|v| {*v += message.values[0]; if *v > 255 { *v %= 255}; });
        // Convert back after previous conversion for 1st element
        message.convert();
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

// TODO: reformat with exact same function in encoding
fn pad(vect: &mut Vec<bool>, expected_size: usize) {
    // TODO: Current size should never be greater than expected_size
    // but should still handle possible error
    for _ in 0..(expected_size - vect.len()) {
        vect.push(false);
    }
}

/// Generate error code bits for the format.
///
/// `format` should be 5 bits long starting with error code level then
/// mask pattern. 
/// Eg. 01 (indicator for error correction level L)
///     100 (binary for 4, i.e. mask pattern 4)
///     => 01100
///
/// Followed steps:
///     - Pad the format to reach 15 bits
///     - Remove the zeros on the left of the format
///     - Perform division
///         - Pad the generator to have the same length as the format
///             - Reset the generator to default after each iteration 
///         - XOR format to the generator
///         - Remove the zeros on the left of the format
///         - Repeat until the format is no more than 10 bits long 
///     - Pad the format if less than 10 bits long
pub fn generate_error_code_bits(format: &Vec<bool>) -> Vec<bool> {
    // From documentation, corresponds to x^10 + x^8 + x^5 + x^4 + x^2 + x + 1
    let polynomial = vec![true, false, true, false, false, true, true, false, true, true, true];
    let mut result = format.clone();
    pad(&mut result , 15);
    // TODO refactor
    result =  result.into_iter().skip_while(|&b| !b).collect();

    println!("Format {:?}", result);
    println!("Gen {:?}\n", polynomial);

    while result.len() > 10 {
        let mut poly = polynomial.clone();
        pad(&mut poly, result.len());
        println!("Padded gen {:?}", poly);
        result = result.iter()
            .zip(poly.iter())
            .map(|(&x, &y)| x ^ y)
            .collect();
        println!("Xored format {:?}", result);
        result =  result.into_iter().skip_while(|&b| !b).collect();
        println!("Trimmed format {:?} {}\n", result, result.len());
    }

    pad(&mut result, 10);


    return result;
}

/// Generate the format string for the correction.
///
/// `format` should be 5 bits long starting with error code level then
/// mask pattern. 
/// Eg. 01 (indicator for error correction level L)
///     100 (binary for 4, i.e. mask pattern 4)
///     => 01100
///
/// Followed steps:
///     - Get the error code bits
///     - XOR with the generator
pub fn generate_format_string(format: &Vec<bool>) -> Vec<bool> {
    let error_code_bits = generate_error_code_bits(&format);
    let mut result = [format.clone(), error_code_bits].concat();
    // From documentation: 101010000010010
    let mask_string = vec![true, false, true, false, true, false, false, false, false, false, true, false, false, true, false];
    // TODO: refactor the XOR operation
    result = result.iter()
        .zip(mask_string.iter())
        .map(|(&x, &y)| x ^ y)
        .collect();

    return result;
}

#[cfg(test)]
mod tests {
    use crate::correction::{self, divide_message_polynomial, generate_error_code_bits, generate_format_string, get_generator_polynomial, Polynomial};

    #[test]
    pub fn test_divide_message_polynomial() {
        let vect = vec![32, 91, 11, 120, 209, 114, 220, 77, 67, 64, 236, 17, 236, 17, 236, 17];
        let mut poly = Polynomial::new(correction::NotationMode::DECIMAL, vect);
        let mut other = get_generator_polynomial(0);
        let expected_result = Polynomial::new(correction::NotationMode::DECIMAL, vec![196, 35, 39, 119, 235, 215, 231, 226, 93, 23]);

        divide_message_polynomial(&mut poly, &mut other);

        assert_eq!(poly, expected_result);
    }

    #[test]
    pub fn test_generate_error_code_bits() {
        // For L and Mask 4
        let format = vec![false, true, true, false, false];
        let result = generate_error_code_bits(&format);
        // The expected result is incorrect imo
        let expected_result = vec![true, false, false, false, true, true, true, true, false, true];

        assert_eq!(result, expected_result);
    }

    #[test]
    pub fn test_generate_format_string() {
        // For L and Mask 4
        let format = vec![false, true, true, false, false];
        let result = generate_format_string(&format);
        let expected_result = vec![true, true, false, false, true, true, false, false, false, true, false, true, true, true, true];

        assert_eq!(result, expected_result);
    }
}
