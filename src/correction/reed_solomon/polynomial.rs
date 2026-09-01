use crate::correction::reed_solomon::gf256::get_log_antilog;


#[derive(PartialEq, Eq, Clone, Debug)]
pub enum NotationMode {
    Decimal,
    Alpha
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
        let idx = if self.mode == NotationMode::Alpha {1} else {3};
        let log_antilog = get_log_antilog();

        println!("convertion {}", log_antilog[5][idx]);
        self.values.iter_mut().for_each(|v| *v = log_antilog[usize::from(*v)][idx]);
        self.mode = if self.mode == NotationMode::Alpha { NotationMode::Decimal} else { NotationMode::Alpha }
    }

    pub fn xor(&mut self, other: &Polynomial) {
        self.values
            .iter_mut()
            .enumerate()
            // Only xor up to size of generator
            .take_while(|(idx, _v)| *idx < other.values.len())
            .for_each(|(idx, v)| { *v ^= other.values[idx]; if *v > 255 { *v %= 255 }; });

        let len_diff = other.values.len() as i8 - self.values.len() as i8;
        if len_diff > 0 {
            // Each iteration increases the size, idx related to initial one
            let initial_len = self.values.len();
            for i  in 0..len_diff {
                self.values.push(other.values[initial_len + usize::try_from(i).unwrap()]);
            }
        }
    }

    
    pub fn to_string(&self) -> String {
        return self.values
            .iter()
            .enumerate()
            .map(|(idx, v)| format!("{}{}x^{}", 
                if self.mode == NotationMode::Alpha { "a^" } else { "" } , 
                v, self.exponent - idx as u16 - 1))
            .collect::<Vec<String>>()
            .join("+");
    }
    
}

pub fn divide_message_polynomial(message: &mut Polynomial, generator: &Polynomial) {
    let mut generator_copy = generator.clone();

    if message.mode !=NotationMode::Decimal {
        message.convert();
    }

    if generator_copy.mode !=NotationMode::Alpha {
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
        
        // Coefficient to be converted
        let first_coef = message.values[0];

        // If leading coefficient is zero no division to perform
        if first_coef == 0 {
            message.values.remove(0);
            message.exponent -= 1;
            continue;
        }

        let first_alpha = get_log_antilog()[usize::from(first_coef)][3];

        generator_copy.values.iter_mut().for_each(|v| {
            *v += first_alpha;
            if *v > 255 { *v %= 255};
        });

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
