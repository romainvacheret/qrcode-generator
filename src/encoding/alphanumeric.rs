use crate::{encoding::{EncodedData, to_binary}, error::Error};

pub(super) fn encode(string: &str) -> Result<EncodedData, Error> {
    const ALPHA_PAIR_SIZE: usize = 11;
    const ALPHA_SINGLE_SIZE: usize = 6;
    let current_size = string.len();

    encode_alpha_values(string).map(|vec| {
        let mut result = Vec::<bool>::new(); 
        for (idx, &val) in vec.iter().enumerate() {
            // If is last element and odd number of chars
            let size = if (idx == vec.len() - 1) && (current_size % 2 == 1) {
                ALPHA_SINGLE_SIZE
            } else {
                ALPHA_PAIR_SIZE
            };

            result = [result, to_binary(val, size)].concat();
        }

        EncodedData { count: string.len(), bits: result }
    })
}

fn get_alpha_value(character: char) -> Result<usize, Error> {
    const CHARACTERS: &str = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ $%*+-./:";

    CHARACTERS.chars()
        .position(|r| r == character)
        .ok_or(Error::InvalidCharacter)
}

fn encode_alpha_pair(char1: char, char2: char) -> Result<usize, Error> {
    let first_encoding = get_alpha_value(char1)?;

    Ok(if char2 == '\0' {
        first_encoding
    } else {
        first_encoding * 45 + get_alpha_value(char2)?
    })
}

fn encode_alpha_values(string: &str) -> Result<Vec<usize>, Error> {
    let mut result_vec = Vec::<usize>::new();

    for i in (0..string.len()).step_by(2) {
        let char1 = string.chars().nth(i).ok_or(Error::IndexOutOfBounds)?;
        let char2 = if i < string.len() -1 { 
            string.chars().nth(i + 1).ok_or(Error::IndexOutOfBounds)? 
        } else { '\0' };

        result_vec.push(encode_alpha_pair(char1, char2)?);
    }

    Ok(result_vec)
}


// TODO: improve tests
#[cfg(test)]
mod tests {
    use crate::encoding::alphanumeric;

    #[test]
    fn test_encode_text() {
        let expected_value = Vec::from([
            false, false, true, true, true, false, false, true, true, 
            true, false, true, true, true, false, false, true, true, 
            true, false, false, true, false, false, false, false, true, false]);
        let result = alphanumeric::encode("AC-42").expect("Expected to succeed");

        // TODO: update the tests with EncodedData
        assert_eq!(
            result.bits, 
            expected_value, 
            "Results do no match. Got: {:?} and expected {:?}", 
            result.bits, 
            expected_value
        );
    }

    #[test]
    fn test_encode_text2() {
        let expected_value = vec![
            false, true, true, false, false, false, false, true, false, true, true,  
            false, true, true, true, true, false, false, false, true, true, false,  
            true, false, false, false, true, false, true, true, true, false, false,  
            true, false, true, true, false, true, true, true, false, false, false,  
            true, false, false, true, true, false, true, false, true, false, false,  
            false, false, true, true, false, true];
        let result = alphanumeric::encode("HELLO WORLD").expect("Expected to succeed");

        // TODO: update the tests with EncodedData
        assert_eq!(
            result.bits, 
            expected_value, 
            "Results do no match. Got: {:?} and expected {:?}", 
            result.bits, 
            expected_value
        );
    }
}
