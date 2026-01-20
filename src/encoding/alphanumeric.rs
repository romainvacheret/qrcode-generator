use crate::encoding::{to_binary, Encode};

pub struct Alphanumeric;

impl Encode for Alphanumeric {
    fn get_binary(&self) -> Vec<bool> {
        Vec::<bool>::from([false, false, true, false])
    }

    fn encode_text(&self, string: String) -> Option<Vec<bool>> {
        const ALPHA_PAIR_SIZE: usize = 11;
        const ALPHA_SINGLE_SIZE: usize = 6;
        let current_size = string.len();

        self.encode_alpha_values(string).map(|vec| {
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
            result
        })
    }
}

impl Alphanumeric {
    fn get_alpha_value(&self, character: char) -> Option<usize> {
        let characters = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ $%*+-./:";
        return characters.chars().position(|r| r == character);
    }

    fn encode_alpha_pair(&self, char1: char, char2: char) -> Option<usize> {
        let opt_first_encoding = self.get_alpha_value(char1);

        // TODO: handle btter
        if opt_first_encoding.is_none() {
            panic!("ERROR: Invalid char");
        }

        let first_encoding = opt_first_encoding.unwrap();

        return Some(if char2 != '\0' { 
            first_encoding * 45 + self.get_alpha_value(char2)?
        } else { first_encoding });
    }

    fn encode_alpha_values(&self, string: String) -> Option<Vec<usize>> {
        let mut result_vec = Vec::<usize>::new();

        for i in (0..string.len()).step_by(2) {
            let char1 = string.chars().nth(i)?;
            let char2 = if i < string.len() -1 { string.chars().nth(i + 1)? } else { '\0' };
            result_vec.push(self.encode_alpha_pair(char1, char2)?);
        }

        return Some(result_vec);
    }
}


#[cfg(test)]
mod tests {
    use crate::encoding::{Alphanumeric, Encode};

    #[test]
    fn test_encode_text() {
        let expected_value = Vec::from([false,false,true,true,true,false,false,true,true,true,false, 
            true,true,true,false,false,true,true,true,false,false,true,
            false,false,false,false,true,false]);
        let result = Alphanumeric.encode_text(String::from("AC-42")).unwrap();

        assert_eq!(result, expected_value, "Results do no match. Got: {:?} and expected {:?}", result, expected_value);
    }

    #[test]
    fn test_encode_text2() {
        let expected_value = vec![false,true,true,false,false,false,false,true,false,true,true, false,true,true,true,true,false,false,false,true,true,false, true,false,false,false,true,false,true,true,true,false,false, true,false,true,true,false,true,true,true,false,false,false, true,false,false,true,true,false,true,false,true,false,false, false,false,true,true,false,true];
        let result = Alphanumeric.encode_text(String::from("HELLO WORLD")).unwrap();

        assert_eq!(result, expected_value, "Results do no match. Got: {:?} and expected {:?}", result, expected_value);
    }
}
