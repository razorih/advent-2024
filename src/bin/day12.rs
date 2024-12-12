use std::{collections::HashSet, io};

use advent::{grid::Grid, read_input};

/// Expands each region and returns (inside, perimeter) pair.
///
/// This is a basic depth-first graph traverse.
fn expand_region(
    grid: &Grid<char>,
    visited: &mut HashSet<(usize, usize)>,
    start: (usize, usize),
    region: char,
) -> (usize, usize) {
    let mut stack: Vec<(usize, usize)> = Vec::new();
    let dirs: [(isize, isize); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

    stack.push(start);
    visited.insert(start);

    let mut inside = 0;
    let mut perimeter = 0;

    while let Some((col, row)) = stack.pop() {
        // we only discover points within the same region
        // thus, this point is inside
        inside += 1;

        // look for connections
        let entry = grid.entry(col, row);
        for (dc, dr) in dirs {
            match entry.offset(dc, dr) {
                // connected region
                Some((newreg, newcol, newrow)) if newreg == region => {
                    // don't look back
                    if !visited.contains(&(newcol, newrow)) {
                        visited.insert((newcol, newrow));
                        stack.push((newcol, newrow));
                    }
                },
                // out of bounds or other region, this is a fence
                _ => perimeter += 1,
            }
        }
    }

    (inside, perimeter)
}

fn silver(grid: &Grid<char>) -> usize {
    let mut prices: Vec<(usize, usize)> = Vec::new();
    let mut expanded: HashSet<(usize, usize)> = HashSet::new();

    for ((col, row), &reg) in grid.iter_indexed() {
        // this point has already been expanded by expand_region
        if expanded.contains(&(col, row)) {
            continue
        }

        prices.push(
            expand_region(grid, &mut expanded, (col, row), reg)
        );
    }

    prices.into_iter()
        .fold(0, |price, (inside, perimeter)| price + inside*perimeter)
}

fn main() -> io::Result<()> {
    let input = read_input()?;
    let grid = Grid::new(&input, |chr, _| chr);

    println!("silver: {}", silver(&grid));

    Ok(())
}
