// TODO: make it an iterator intead of accessing data
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

    pub fn display(&self) {
        for row in &self.data {
            for &col in row {
                print!("{} ", if col { "#" } else { "-" });
            }
            println!();
        }
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

pub fn print_as_binary(vec: &Vec<bool>, chunk_size: usize) {
    println!("{}", vec
        .chunks(chunk_size)
        .map(|chunk| chunk.iter()
            .map(|n| if *n == true { "1" } else { "0" })
            .collect::<String>())
        .collect::<Vec<_>>()
        .join(" "));
}
