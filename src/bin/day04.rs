use std::io;

use advent::read_input;

#[derive(Debug)]
struct StringGrid {
    content: Vec<char>,
    len: usize,
}

impl StringGrid {
    fn new(content: &str) -> Self {
        let len = content.lines().next().unwrap().len();

        Self {
            content: content.chars().filter(|c| c.is_ascii_uppercase()).collect(),
            len
        }
    }

    fn at(&self, col: usize, row: usize) -> Option<char> {
        let start = col + row * self.len;
        self.content.get(start).copied()
    }

    fn at_offset(&self, col: usize, row: usize, col_diff: isize, row_diff: isize) -> Option<char> {
        let Some(true_col) = col.checked_add_signed(col_diff) else { return None };
        let Some(true_row) = row.checked_add_signed(row_diff) else { return None };

        if true_col >= self.len { return None }
        if true_row >= self.len { return None }

        self.at(true_col, true_row)
    }

    fn row_len(&self) -> usize {
        self.len
    }
}

/// Check how many valid "XMAS" there are starting from given point
fn check_xmas(grid: &StringGrid, col: usize, row: usize) -> usize {
    if grid.at(col, row) != Some('X') {
        return 0
    }

    let mut correct = [
        0, 0, 0, 0, // hor/vert
        0, 0, 0, 0, // diagonals/antidiag
    ];

    for (diff, expected) in [(1, 'M'), (2, 'A'), (3, 'S')] {
        if grid.at_offset(col, row, diff, 0)  == Some(expected) { correct[0] += 1 }
        if grid.at_offset(col, row, -diff, 0) == Some(expected) { correct[1] += 1 }
        if grid.at_offset(col, row, 0, diff)  == Some(expected) { correct[2] += 1 }
        if grid.at_offset(col, row, 0, -diff) == Some(expected) { correct[3] += 1 }

        if grid.at_offset(col, row, diff, diff)   == Some(expected) { correct[4] += 1 }
        if grid.at_offset(col, row, -diff, -diff) == Some(expected) { correct[5] += 1 }
        if grid.at_offset(col, row, diff, -diff)  == Some(expected) { correct[6] += 1 }
        if grid.at_offset(col, row, -diff, diff)  == Some(expected) { correct[7] += 1 }
    }

    // println!("correct: {:?}", correct);

    correct.into_iter().filter(|&it| it == 3).count()
}

fn silver(input: &str) -> usize {
    let grid = StringGrid::new(input);
    let mut xmases = 0;

    for col in 0..grid.row_len() {
        for row in 0..grid.row_len() { // assume NxN grid :)
            xmases += check_xmas(&grid, col, row)
        }
    }

    xmases
}

fn main() -> io::Result<()> {
    let input = read_input()?;

    println!("silver: {}", silver(&input));

    Ok(())
}
