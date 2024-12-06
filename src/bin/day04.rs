use std::io;

use advent::{grid::Grid, read_input};

/// Check how many valid "XMAS" there are starting from given point
fn check_xmas(grid: &Grid<char>, col: usize, row: usize) -> usize {
    if grid.at(col, row) != Some('X') {
        return 0
    }

    let mut correct = [
        0, 0, 0, 0, // horizontals/verticals
        0, 0, 0, 0, // diagonals/antidiagonals
    ];

    let entry = grid.entry(col, row);
    for (diff, expected) in [(1, 'M'), (2, 'A'), (3, 'S')] {
        if entry.at_offset(diff, 0)  == Some(expected) { correct[0] += 1 }
        if entry.at_offset(-diff, 0) == Some(expected) { correct[1] += 1 }
        if entry.at_offset(0, diff)  == Some(expected) { correct[2] += 1 }
        if entry.at_offset(0, -diff) == Some(expected) { correct[3] += 1 }

        if entry.at_offset(diff, diff)   == Some(expected) { correct[4] += 1 }
        if entry.at_offset(-diff, -diff) == Some(expected) { correct[5] += 1 }
        if entry.at_offset(diff, -diff)  == Some(expected) { correct[6] += 1 }
        if entry.at_offset(-diff, diff)  == Some(expected) { correct[7] += 1 }
    }

    correct.into_iter().filter(|&it| it == 3).count()
}

fn check_mas(grid: &Grid<char>, col: usize, row: usize) -> bool {
    // x-mases are centered around 'A's
    if grid.at(col, row) != Some('A') {
        return false
    }

    let entry = grid.entry(col, row);
    let diag = [entry.at_offset(1, 1), entry.at_offset(-1, -1)];
    let antidiag = [entry.at_offset(1, -1), entry.at_offset(-1, 1)];

    let is_valid = |diag: [Option<char>; 2]| diag == [Some('S'), Some('M')] || diag == [Some('M'), Some('S')];

    is_valid(diag) && is_valid(antidiag)
}

fn gold(grid: &Grid<char>) -> usize {
    let mut mases = 0;

    for col in 0..grid.width() {
        for row in 0..grid.height() {
            if check_mas(&grid, col, row) { mases += 1 }
        }
    }

    mases
}

fn silver(grid: &Grid<char>) -> usize {
    let mut xmases = 0;

    for col in 0..grid.width() {
        for row in 0..grid.height() {
            xmases += check_xmas(&grid, col, row)
        }
    }

    xmases
}

fn main() -> io::Result<()> {
    let input = read_input()?;
    let grid = Grid::new(&input, |c, _| c);

    println!("silver: {}", silver(&grid));
    println!("gold: {}", gold(&grid));

    Ok(())
}
