#[derive(PartialEq, Eq, Hash, Debug)]
pub enum Mask {
    MASK0,
    MASK1,
    MASK2,
    MASK3,
    MASK4,
    MASK5,
    MASK6,
    MASK7
}

impl Mask {
    // u8 is enough because the biggest QR Code is 177 * 177
    fn compute_mask0(row: u8, col: u8) -> bool { return (row + col) % 2 == 0 }
    fn compute_mask1(row: u8, _col: u8) -> bool { return row  % 2 == 0 }
    fn compute_mask2(_row: u8, col: u8) -> bool { return col  % 3 == 0 }
    fn compute_mask3(row: u8, col: u8) -> bool { return (row + col)  % 3 == 0 }
    fn compute_mask4(row: u8, col: u8) -> bool { return ((row / 2) + (col / 3))  % 2 == 0 }
    fn compute_mask5(row: u8, col: u8) -> bool { return (row * col % 2) + (row * col % 3) == 0 }
    fn compute_mask6(row: u8, col: u8) -> bool { return ((row * col % 2) + (row * col % 3)) % 2 == 0 }
    fn compute_mask7(row: u8, col: u8) -> bool { return (((row + col) % 2) + (row * col % 3)) % 2 == 0 }

    pub fn apply(&self, matrix: &mut Vec<Vec<bool>>) {
        let func = match self {
            Mask::MASK0 =>  Self::compute_mask0,
            Mask::MASK1 =>  Self::compute_mask1,
            Mask::MASK2 =>  Self::compute_mask2,
            Mask::MASK3 =>  Self::compute_mask3,
            Mask::MASK4 =>  Self::compute_mask4,
            Mask::MASK5 =>  Self::compute_mask5,
            Mask::MASK6 =>  Self::compute_mask6,
            Mask::MASK7 =>  Self::compute_mask7
        };

        for i in 0..matrix.len() {
            for j in 0..matrix[i].len() {
                matrix[i][j] = func(i as u8, j as u8)
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::masking::Mask;

    pub fn test_apply_mask(mask: &Mask) {
        let expected: HashMap<Mask, Vec<Vec<bool>>> = HashMap::from([
        (Mask::MASK0, vec![
            vec![true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true],
            vec![false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false],
            vec![true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true],
            vec![false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false],
            vec![true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true],
            vec![false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false],
            vec![true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true],
            vec![false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false],
            vec![true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true],
            vec![false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false],
            vec![true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true],
            vec![false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false],
            vec![true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true],
            vec![false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false],
            vec![true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true],
            vec![false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false],
            vec![true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true],
            vec![false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false],
            vec![true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true],
            vec![false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false],
            vec![true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true]]),

        (Mask::MASK1, vec![vec![true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true],
            vec![false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false],
            vec![true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true],
            vec![false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false],
            vec![true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true],
            vec![false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false],
            vec![true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true],
            vec![false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false],
            vec![true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true],
            vec![false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false],
            vec![true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true],
            vec![false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false],
            vec![true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true],
            vec![false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false],
            vec![true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true],
            vec![false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false],
            vec![true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true],
            vec![false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false],
            vec![true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true],
            vec![false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false],
            vec![true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true]]),
        (Mask::MASK2,  vec![vec![true, false, false, true, false, false, true, false, false, true, false, false, true, false, false, true, false, false, true, false, false],
            vec![true, false, false, true, false, false, true, false, false, true, false, false, true, false, false, true, false, false, true, false, false],
            vec![true, false, false, true, false, false, true, false, false, true, false, false, true, false, false, true, false, false, true, false, false],
            vec![true, false, false, true, false, false, true, false, false, true, false, false, true, false, false, true, false, false, true, false, false],
            vec![true, false, false, true, false, false, true, false, false, true, false, false, true, false, false, true, false, false, true, false, false],
            vec![true, false, false, true, false, false, true, false, false, true, false, false, true, false, false, true, false, false, true, false, false],
            vec![true, false, false, true, false, false, true, false, false, true, false, false, true, false, false, true, false, false, true, false, false],
            vec![true, false, false, true, false, false, true, false, false, true, false, false, true, false, false, true, false, false, true, false, false],
            vec![true, false, false, true, false, false, true, false, false, true, false, false, true, false, false, true, false, false, true, false, false],
            vec![true, false, false, true, false, false, true, false, false, true, false, false, true, false, false, true, false, false, true, false, false],
            vec![true, false, false, true, false, false, true, false, false, true, false, false, true, false, false, true, false, false, true, false, false],
            vec![true, false, false, true, false, false, true, false, false, true, false, false, true, false, false, true, false, false, true, false, false],
            vec![true, false, false, true, false, false, true, false, false, true, false, false, true, false, false, true, false, false, true, false, false],
            vec![true, false, false, true, false, false, true, false, false, true, false, false, true, false, false, true, false, false, true, false, false],
            vec![true, false, false, true, false, false, true, false, false, true, false, false, true, false, false, true, false, false, true, false, false],
            vec![true, false, false, true, false, false, true, false, false, true, false, false, true, false, false, true, false, false, true, false, false],
            vec![true, false, false, true, false, false, true, false, false, true, false, false, true, false, false, true, false, false, true, false, false],
            vec![true, false, false, true, false, false, true, false, false, true, false, false, true, false, false, true, false, false, true, false, false],
            vec![true, false, false, true, false, false, true, false, false, true, false, false, true, false, false, true, false, false, true, false, false],
            vec![true, false, false, true, false, false, true, false, false, true, false, false, true, false, false, true, false, false, true, false, false],
            vec![true, false, false, true, false, false, true, false, false, true, false, false, true, false, false, true, false, false, true, false, false]]),
        (Mask::MASK3, vec![vec![true, false, false, true, false, false, true, false, false, true, false, false, true, false, false, true, false, false, true, false, false],
            vec![false, false, true, false, false, true, false, false, true, false, false, true, false, false, true, false, false, true, false, false, true],
            vec![false, true, false, false, true, false, false, true, false, false, true, false, false, true, false, false, true, false, false, true, false],
            vec![true, false, false, true, false, false, true, false, false, true, false, false, true, false, false, true, false, false, true, false, false],
            vec![false, false, true, false, false, true, false, false, true, false, false, true, false, false, true, false, false, true, false, false, true],
            vec![false, true, false, false, true, false, false, true, false, false, true, false, false, true, false, false, true, false, false, true, false],
            vec![true, false, false, true, false, false, true, false, false, true, false, false, true, false, false, true, false, false, true, false, false],
            vec![false, false, true, false, false, true, false, false, true, false, false, true, false, false, true, false, false, true, false, false, true],
            vec![false, true, false, false, true, false, false, true, false, false, true, false, false, true, false, false, true, false, false, true, false],
            vec![true, false, false, true, false, false, true, false, false, true, false, false, true, false, false, true, false, false, true, false, false],
            vec![false, false, true, false, false, true, false, false, true, false, false, true, false, false, true, false, false, true, false, false, true],
            vec![false, true, false, false, true, false, false, true, false, false, true, false, false, true, false, false, true, false, false, true, false],
            vec![true, false, false, true, false, false, true, false, false, true, false, false, true, false, false, true, false, false, true, false, false],
            vec![false, false, true, false, false, true, false, false, true, false, false, true, false, false, true, false, false, true, false, false, true],
            vec![false, true, false, false, true, false, false, true, false, false, true, false, false, true, false, false, true, false, false, true, false],
            vec![true, false, false, true, false, false, true, false, false, true, false, false, true, false, false, true, false, false, true, false, false],
            vec![false, false, true, false, false, true, false, false, true, false, false, true, false, false, true, false, false, true, false, false, true],
            vec![false, true, false, false, true, false, false, true, false, false, true, false, false, true, false, false, true, false, false, true, false],
            vec![true, false, false, true, false, false, true, false, false, true, false, false, true, false, false, true, false, false, true, false, false],
            vec![false, false, true, false, false, true, false, false, true, false, false, true, false, false, true, false, false, true, false, false, true],
            vec![false, true, false, false, true, false, false, true, false, false, true, false, false, true, false, false, true, false, false, true, false]]),
        (Mask::MASK4, vec![vec![true, true, true, false, false, false, true, true, true, false, false, false, true, true, true, false, false, false, true, true, true],
            vec![true, true, true, false, false, false, true, true, true, false, false, false, true, true, true, false, false, false, true, true, true],
            vec![false, false, false, true, true, true, false, false, false, true, true, true, false, false, false, true, true, true, false, false, false],
            vec![false, false, false, true, true, true, false, false, false, true, true, true, false, false, false, true, true, true, false, false, false],
            vec![true, true, true, false, false, false, true, true, true, false, false, false, true, true, true, false, false, false, true, true, true],
            vec![true, true, true, false, false, false, true, true, true, false, false, false, true, true, true, false, false, false, true, true, true],
            vec![false, false, false, true, true, true, false, false, false, true, true, true, false, false, false, true, true, true, false, false, false],
            vec![false, false, false, true, true, true, false, false, false, true, true, true, false, false, false, true, true, true, false, false, false],
            vec![true, true, true, false, false, false, true, true, true, false, false, false, true, true, true, false, false, false, true, true, true],
            vec![true, true, true, false, false, false, true, true, true, false, false, false, true, true, true, false, false, false, true, true, true],
            vec![false, false, false, true, true, true, false, false, false, true, true, true, false, false, false, true, true, true, false, false, false],
            vec![false, false, false, true, true, true, false, false, false, true, true, true, false, false, false, true, true, true, false, false, false],
            vec![true, true, true, false, false, false, true, true, true, false, false, false, true, true, true, false, false, false, true, true, true],
            vec![true, true, true, false, false, false, true, true, true, false, false, false, true, true, true, false, false, false, true, true, true],
            vec![false, false, false, true, true, true, false, false, false, true, true, true, false, false, false, true, true, true, false, false, false],
            vec![false, false, false, true, true, true, false, false, false, true, true, true, false, false, false, true, true, true, false, false, false],
            vec![true, true, true, false, false, false, true, true, true, false, false, false, true, true, true, false, false, false, true, true, true],
            vec![true, true, true, false, false, false, true, true, true, false, false, false, true, true, true, false, false, false, true, true, true],
            vec![false, false, false, true, true, true, false, false, false, true, true, true, false, false, false, true, true, true, false, false, false],
            vec![false, false, false, true, true, true, false, false, false, true, true, true, false, false, false, true, true, true, false, false, false],
            vec![true, true, true, false, false, false, true, true, true, false, false, false, true, true, true, false, false, false, true, true, true]]),
        (Mask::MASK5, vec![vec![true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true],
            vec![true, false, false, false, false, false, true, false, false, false, false, false, true, false, false, false, false, false, true, false, false],
            vec![true, false, false, true, false, false, true, false, false, true, false, false, true, false, false, true, false, false, true, false, false],
            vec![true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true],
            vec![true, false, false, true, false, false, true, false, false, true, false, false, true, false, false, true, false, false, true, false, false],
            vec![true, false, false, false, false, false, true, false, false, false, false, false, true, false, false, false, false, false, true, false, false],
            vec![true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true],
            vec![true, false, false, false, false, false, true, false, false, false, false, false, true, false, false, false, false, false, true, false, false],
            vec![true, false, false, true, false, false, true, false, false, true, false, false, true, false, false, true, false, false, true, false, false],
            vec![true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true],
            vec![true, false, false, true, false, false, true, false, false, true, false, false, true, false, false, true, false, false, true, false, false],
            vec![true, false, false, false, false, false, true, false, false, false, false, false, true, false, false, false, false, false, true, false, false],
            vec![true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true],
            vec![true, false, false, false, false, false, true, false, false, false, false, false, true, false, false, false, false, false, true, false, false],
            vec![true, false, false, true, false, false, true, false, false, true, false, false, true, false, false, true, false, false, true, false, false],
            vec![true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true],
            vec![true, false, false, true, false, false, true, false, false, true, false, false, true, false, false, true, false, false, true, false, false],
            vec![true, false, false, false, false, false, true, false, false, false, false, false, true, false, false, false, false, false, true, false, false],
            vec![true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true],
            vec![true, false, false, false, false, false, true, false, false, false, false, false, true, false, false, false, false, false, true, false, false],
            vec![true, false, false, true, false, false, true, false, false, true, false, false, true, false, false, true, false, false, true, false, false]]),
        (Mask::MASK6, vec![vec![true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true],
            vec![true, true, true, false, false, false, true, true, true, false, false, false, true, true, true, false, false, false, true, true, true],
            vec![true, true, false, true, true, false, true, true, false, true, true, false, true, true, false, true, true, false, true, true, false],
            vec![true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true],
            vec![true, false, true, true, false, true, true, false, true, true, false, true, true, false, true, true, false, true, true, false, true],
            vec![true, false, false, false, true, true, true, false, false, false, true, true, true, false, false, false, true, true, true, false, false],
            vec![true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true],
            vec![true, true, true, false, false, false, true, true, true, false, false, false, true, true, true, false, false, false, true, true, true],
            vec![true, true, false, true, true, false, true, true, false, true, true, false, true, true, false, true, true, false, true, true, false],
            vec![true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true],
            vec![true, false, true, true, false, true, true, false, true, true, false, true, true, false, true, true, false, true, true, false, true],
            vec![true, false, false, false, true, true, true, false, false, false, true, true, true, false, false, false, true, true, true, false, false],
            vec![true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true],
            vec![true, true, true, false, false, false, true, true, true, false, false, false, true, true, true, false, false, false, true, true, true],
            vec![true, true, false, true, true, false, true, true, false, true, true, false, true, true, false, true, true, false, true, true, false],
            vec![true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true],
            vec![true, false, true, true, false, true, true, false, true, true, false, true, true, false, true, true, false, true, true, false, true],
            vec![true, false, false, false, true, true, true, false, false, false, true, true, true, false, false, false, true, true, true, false, false],
            vec![true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true],
            vec![true, true, true, false, false, false, true, true, true, false, false, false, true, true, true, false, false, false, true, true, true],
            vec![true, true, false, true, true, false, true, true, false, true, true, false, true, true, false, true, true, false, true, true, false]]),
        (Mask::MASK7, vec![vec![true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true],
            vec![false, false, false, true, true, true, false, false, false, true, true, true, false, false, false, true, true, true, false, false, false],
            vec![true, false, false, false, true, true, true, false, false, false, true, true, true, false, false, false, true, true, true, false, false],
            vec![false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false],
            vec![true, true, true, false, false, false, true, true, true, false, false, false, true, true, true, false, false, false, true, true, true],
            vec![false, true, true, true, false, false, false, true, true, true, false, false, false, true, true, true, false, false, false, true, true],
            vec![true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true],
            vec![false, false, false, true, true, true, false, false, false, true, true, true, false, false, false, true, true, true, false, false, false],
            vec![true, false, false, false, true, true, true, false, false, false, true, true, true, false, false, false, true, true, true, false, false],
            vec![false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false],
            vec![true, true, true, false, false, false, true, true, true, false, false, false, true, true, true, false, false, false, true, true, true],
            vec![false, true, true, true, false, false, false, true, true, true, false, false, false, true, true, true, false, false, false, true, true],
            vec![true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true],
            vec![false, false, false, true, true, true, false, false, false, true, true, true, false, false, false, true, true, true, false, false, false],
            vec![true, false, false, false, true, true, true, false, false, false, true, true, true, false, false, false, true, true, true, false, false],
            vec![false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false],
            vec![true, true, true, false, false, false, true, true, true, false, false, false, true, true, true, false, false, false, true, true, true],
            vec![false, true, true, true, false, false, false, true, true, true, false, false, false, true, true, true, false, false, false, true, true],
            vec![true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true],
            vec![false, false, false, true, true, true, false, false, false, true, true, true, false, false, false, true, true, true, false, false, false],
            vec![true, false, false, false, true, true, true, false, false, false, true, true, true, false, false, false, true, true, true, false, false]])
    ]);
        let mut matrix = vec![vec![false; 21]; 21];
        let expected_value = expected.get(&mask);

        mask.apply(&mut matrix);
        assert_eq!(Some(&matrix), expected_value, "Mask {:?} gives unexpected result", mask);
    }

    #[test]
    fn test_apply_mask_all() {
        let masks = [Mask::MASK0, Mask::MASK1, Mask::MASK2, Mask::MASK3,
            Mask::MASK4, Mask::MASK5, Mask::MASK6, Mask::MASK7];
        for mask in masks {
            test_apply_mask(&mask);
        }
    }
}
