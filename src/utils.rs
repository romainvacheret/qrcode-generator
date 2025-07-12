pub struct Matrix {
    pub data: Vec<Vec<bool>>
}

pub struct Pos {
    pub row: usize,
    pub col: usize
}

impl Matrix {
    pub fn new(size: usize) -> Self {
        return Matrix { data: vec![vec![false; size]; size] };
    }

    pub fn display(&self) {
        for row in &self.data {
            for &col in row {
                print!("{} ", if col { "#" } else { " " });
            }
            println!();
        }
    }
}

impl Pos {
    pub fn new(row: usize, col: usize) -> Self {
        return Pos { row: row, col: col };
    }
}
