use crate::utils::pad_until;

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
fn generate_error_code_bits(format: &Vec<bool>) -> Vec<bool> {
    // From documentation, corresponds to x^10 + x^8 + x^5 + x^4 + x^2 + x + 1
    let polynomial = vec![
        true, false, true, false, false, 
        true, true, false, true, true, true
    ];
    let mut result = format.clone();
    pad_until(&mut result , 15);
    // TODO refactor
    result =  result.into_iter().skip_while(|&b| !b).collect();

    println!("Format {:?}", result);
    println!("Gen {:?}\n", polynomial);

    while result.len() > 10 {
        let mut poly = polynomial.clone();
        pad_until(&mut poly, result.len());
        println!("Padded gen {:?}", poly);
        result = result.iter()
            .zip(poly.iter())
            .map(|(&x, &y)| x ^ y)
            .collect();
        println!("Xored format {:?}", result);
        result =  result.into_iter().skip_while(|&b| !b).collect();
        println!("Trimmed format {:?} {}\n", result, result.len());
    }

    // pad(&mut result, 10);
    while result.len() < 10 {
        result.insert(0, false);
    }

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
pub(crate) fn generate_format_string(format: &Vec<bool>) -> Vec<bool> {
    let error_code_bits = generate_error_code_bits(&format);
    let mut result = [format.clone(), error_code_bits].concat();
    // From documentation: 101010000010010
    let mask_string = vec![
        true, false, true, false, true, 
        false, false, false, false, false, 
        true, false, false, true, false
    ];
    // TODO: refactor the XOR operation
    result = result.iter()
        .zip(mask_string.iter())
        .map(|(&x, &y)| x ^ y)
        .collect();

    return result;
}

#[cfg(test)]
mod tests {
    use crate::format::{generate_error_code_bits, generate_format_string};


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
    pub fn test_generate_format_string_l4() {
        // For L and Mask 4
        let format = vec![false, true, true, false, false];
        let result = generate_format_string(&format);
        let expected_result = vec![true, true, false, false, true, true, false, false, false, true, false, true, true, true, true];

        assert_eq!(result, expected_result);
    }

    #[test]
    pub fn test_generate_format_string_m2() {
        // For M and Mask 2
        let format = vec![false, false, false, true, false];
        let result = generate_format_string(&format);
        let expected_result = vec![true, false, true, true, true, true, false, false, true, true, true, true, true, false, false];

        assert_eq!(result, expected_result);
    }

}
