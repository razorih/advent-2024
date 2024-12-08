use std::{collections::{HashMap, HashSet}, io};

use advent::{grid::Grid, read_input};

#[derive(Debug, Clone, Copy)]
enum Tile {
    Empty,
    Antenna(char),
}

fn solve(grid: &Grid<Tile>, gold: bool) -> usize {
    // Gather all (frequency, [position])'s into a hashmap
    let mut antennas: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    for (pos, tile) in grid.iter_indexed() {
        if let Tile::Antenna(freq) = tile {
            antennas.entry(*freq)
                .and_modify(|vec| vec.push(pos))
                .or_insert(vec![pos]);
        }
    }

    // println!("{:?}", antennas);

    // set of unique antinode positions
    let mut antinodes: HashSet<(usize, usize)> = HashSet::new();

    for positions in antennas.values() {
        // println!("---");

        // only check pairs of an antenna and every antenna after that
        for i in 0..positions.len()-1 {
            for j in i+1..positions.len() {
                let first = positions[i];
                let second = positions[j];

                // signed distance between two antennas
                let diff = {
                    let col_diff = second.0 as isize - first.0 as isize;
                    let row_diff = second.1 as isize - first.1 as isize;

                    (col_diff, row_diff)
                };

                // two antinodes:
                // - first->second
                // - second->first

                // in gold, keep adding antinodes until out of bounds
                let mut n = if gold { 1 } else { 2 };
                let mut m = if gold { 1 } else { 2 };

                let entry = grid.entry(first.0, first.1);
                while let Some((_, anti_col, anti_row)) = entry.offset(n*diff.0, n*diff.1) {
                    antinodes.insert((anti_col, anti_row));
                    if gold { n += 1; } else { break }
                }

                let entry = grid.entry(second.0, second.1);
                while let Some((_, anti_col, anti_row)) = entry.offset(-(m*diff.0), -(m*diff.1)) {
                    antinodes.insert((anti_col, anti_row));
                    if gold { m += 1; } else { break }
                }

                // println!("{:?}", diff);
            }
        }
    }

    // println!("antinodes: {:?}", antinodes);
    antinodes.len()
}

fn main() -> io::Result<()> {
    let input = read_input()?;
    let grid: Grid<Tile> = Grid::new(&input, |chr, _| {
        match chr {
            '.' => Tile::Empty,
            c if c.is_ascii_alphanumeric() => Tile::Antenna(c),
            _ => panic!("invalid tile!"),
        }
    });

    println!("silver: {}", solve(&grid, false));
    println!("gold: {}", solve(&grid, true));

    Ok(())
}
