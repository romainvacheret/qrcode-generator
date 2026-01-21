use std::vec;

use crate::utils::structure::{Matrix, Pos};
use crate::qrcode::version::Version;

/// Struct used to apply the patterns to matrices.
/// There are two cases in which they need to be applied.
/// First, for the preparation to ensure that the data will not 
/// be placed on bits that should be used by patterns (on QRCode::are_patterns).
/// Second, place the actual patterns to complete the QR Code (on QRCode::matrix).
/// All values that should be `false` if we simply represented the patterns are 
/// replaced by `is_prep` which is true for the first case and false in the second.
pub struct PatternHelper {
    is_prep: bool
}

impl PatternHelper {
    pub fn new(is_prep: bool ) -> Self {
        return PatternHelper { is_prep: is_prep };
    }

    fn set_square_ring(&self, matrix: &mut Matrix, top_left: &Pos, width: usize, value: bool) {
        for i in 0..width {
            // top-horizontal
            matrix.data[top_left.row][top_left.col + i] = value;
            // bottom-horizontal
            matrix.data[top_left.row + width - 1][top_left.col + i] = value;
            // left-vertical
            matrix.data[top_left.row + i][top_left.col] = value;
            // right-vertical
            matrix.data[top_left.row + i][top_left.col + width - 1] = value;
        }
    }

    fn set_finder(&self, matrix: &mut Matrix, top_left: &Pos) {
        self.set_square_ring(matrix, top_left, 7, true);
        self.set_square_ring(matrix, &Pos::new(top_left.row + 1, top_left.col + 1), 5, self.is_prep);
        self.set_square_ring(matrix, &Pos::new(top_left.row + 2, top_left.col + 2), 3, true);
        matrix.data[top_left.row + 3][top_left.col + 3] = true;
    }

    fn set_finders(&self, matrix: &mut Matrix) {
        self.set_finder(matrix, &Pos::new(0, 0));
        self.set_finder(matrix, &Pos::new(matrix.data.len() - 7, 0));
        self.set_finder(matrix, &Pos::new(0, matrix.data.len() - 7));
    }

    fn set_line(&self, matrix: &mut Matrix, values: &Vec<bool>, left: &Pos, is_vertical: bool) {
        if is_vertical {
            for (idx, &value) in values.iter().enumerate() {
                matrix.data[left.row + idx][left.col] = value;
            }
        } else {
            for (idx, &value) in values.iter().enumerate() {
                matrix.data[left.row][left.col + idx] = value;
            }
        }
    }

    fn set_separators(&self, matrix: &mut Matrix) {
        // top left
        self.set_line(matrix, &vec![self.is_prep; 7], &Pos::new(0, 7), true);
        self.set_line(matrix, &vec![self.is_prep; 8], &Pos::new(7, 0), false);
        // bottom left
        self.set_line(matrix, &vec![self.is_prep; 7], &Pos::new(matrix.data.len() - 7, 7), true);
        self.set_line(matrix, &vec![self.is_prep; 8], &Pos::new(matrix.data.len() - 8, 0), false);
        // top right
        self.set_line(matrix, &vec![self.is_prep; 7], &Pos::new(0, matrix.data.len() - 8), true);
        self.set_line(matrix, &vec![self.is_prep; 8], &Pos::new(7, matrix.data.len() - 8), false);
    }

    fn set_timing(&self, matrix: &mut Matrix) {
        // Takes all the space between the two (finder patters and separators)
        // Each of the two groups is respectively of width 7 and 1
        let timing_length = matrix.data.len() - (8 * 2);
        let timing_values = if self.is_prep { 
            vec![true; timing_length]
        } else {
            (0..timing_length)
                .map(|idx| idx % 2 == 0)
                .collect::<Vec<bool>>()
        };

        self.set_line(matrix, &timing_values, &Pos::new(8, 6), true);
        self.set_line(matrix, &timing_values, &Pos::new(6, 8), false);
    }

    // TODO: add alignment patterns for versions > 1
    // TODO: handle errors if out of bounds
    pub fn apply_patterns(&self, matrix: &mut Matrix, version: &Version) {
        self.set_finders(matrix);
        self.set_separators(matrix);
        self.set_timing(matrix);

        // Dark module
        matrix.data[4 * version.get() + 9][8] = true;
    }

    pub fn apply_patterns2(&self, matrix: &mut Matrix, version: &Version) {
        self.apply_patterns(matrix, version);

        self.set_line(matrix, &vec![self.is_prep; 8], &Pos::new(0, 8), true);
        self.set_line(matrix, &vec![self.is_prep; 9], &Pos::new(8, 0), false);
        // bottom left
        self.set_line(matrix, &vec![self.is_prep; 8], &Pos::new(matrix.data.len() - 8, 8), true);
        // self.set_line(matrix, &vec![self.is_prep; 8], &Pos::new(matrix.data.len() - 9, 0), false);
        // top right
        // self.set_line(matrix, &vec![self.is_prep; 8], &Pos::new(0, matrix.data.len() - 9), true);
        self.set_line(matrix, &vec![self.is_prep; 8], &Pos::new(8, matrix.data.len() - 8), false);
    }
}
