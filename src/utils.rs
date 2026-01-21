
pub mod structure {
    // TODO: make it an iterator intead of accessing data
    use std::fmt::Write;

    pub struct Matrix {
        pub data: Vec<Vec<bool>>,
        pub size: usize 
    }

    #[derive(Clone, Debug)]
    pub struct Pos {
        pub row: usize,
        pub col: usize
    }

    impl Matrix {
        pub fn new(size: usize) -> Self {
            return Matrix { data: vec![vec![false; size]; size], size };
        }

        pub fn as_string(&self) -> String {
            let mut out = String::new();

            for row in &self.data {
                for &col in row {
                    let _ = write!(out, "{} ", if col { "#" } else { "-" });
                }
                out.push('\n');
            }
            out
        }

        pub fn display(&self) {
            print!("{}", self.as_string())
        }

        pub fn get(&self, pos: &Pos) -> Option<&bool> {
            return self.data.get(pos.row)?.get(pos.col);
        }


        pub fn get_next_idx(&self, current: &mut Pos, is_upward: &mut bool) -> bool{
            let pos = current.clone();
            let upward = is_upward.clone();

            // Column of the vertical timing pattern
            if current.col == 6 {
                current.col -= 1;
                println!("ENTER {:?} {}", current, upward);
                return true;
            } 

            // println!("Current {:?}", current);
            // If hitting top/bottom
            if (*is_upward && current.row == 0 && current.col % 2 == 1) || (!*is_upward && current.row == (self.data.len() - 1) && current.col % 2 == 1) {
                // println!("Enter");
                *is_upward = !*is_upward;

                if current.col == 0 {
                    *is_upward = upward;
                    *current = pos;
                    return false;
                }
                current.col -= 1;
            
            } else {
                if *is_upward {
                    // Odd column because idx = size - 1
                    if current.col % 2 == 0 {
                        current.col -= 1;
                    } else {
                        current.col += 1;
                        current.row -= 1;
                    }
                } else {
                    if current.col % 2 == 0 && current.col > 6 {
                        current.col -= 1;
                    } else {
                        current.col += 1;
                        current.row += 1;
                    }
                }
                // // Odd column because idx = size - 1
                // if current.col % 2 == 0 {
                //     current.col -= 1;
                // } else {
                //     if *is_upward {
                //         current.col += 1;
                //         current.row -= 1;
                //     } else {
                //         current.col -= 1;
                //     }
                // }
            }

            return true;
        }
    }

    impl Pos {
        pub fn new(row: usize, col: usize) -> Self {
            return Pos { row: row, col: col };
        }
    }
}

pub mod dev {
    pub fn print_as_binary(vec: &Vec<bool>, chunk_size: usize) {
        println!("{}", vec
            .chunks(chunk_size)
            .map(|chunk| chunk.iter()
                .map(|n| if *n == true { "1" } else { "0" })
                .collect::<String>())
            .collect::<Vec<_>>()
            .join(" "));
    }
}

pub mod bin {
    use crate::utils::pad;

    pub fn to_binary(mut val: usize, expected_size: usize) -> Vec<bool> {
        let mut result = Vec::<bool>::new();

        while val > 0 {
            result.push(val % 2 == 1);
            val /=  2;
        }

        let pad_size = expected_size - result.len();
        pad(&mut result, pad_size);
        result.reverse();
        return result;
    }

    fn vec_to_binary(chunk: &[bool]) -> u16 {
        return chunk.iter().fold(0, |acc, &b| (acc << 1) | (b as u16));
    }

    pub fn to_decimal(encoding: &Vec<bool>) -> Vec<u16> {
        return encoding.chunks(8)
            .map(|chunk| vec_to_binary(&chunk))
            .collect();
    }

    pub fn to_bits(n: u16) -> Vec<bool> {
        (0..8)
            .rev() // start from most significant bit
            .map(|i| (n & (1 << i)) != 0)
            .collect()
    }
}



pub fn pad(vect: &mut Vec<bool>, pad_size: usize) {
    vect.extend((0..pad_size).map(|_| false));
}
