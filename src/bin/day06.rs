use std::{collections::HashSet, io};

use advent::{grid::Grid, read_input};

#[derive(Debug, Clone, Copy)]
enum Tile {
    Empty,
    Obstacle,
    // special case for grabbing guard's initial position, handled separately
    Guard((usize, usize, Direction))
}

impl Tile {
    fn from_char(chr: char) -> Self {
        match chr {
            '#' => Self::Obstacle,
            '.' => Self::Empty,
            _ => panic!("invalid tile"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    /// Get new direction when turning to the right
    fn turn(self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
            Self::Right => Self::Down,
        }
    }
}

#[derive(Debug)]
struct Guard {
    col: usize,
    row: usize,
    dir: Direction,
}

#[derive(Debug)]
struct Map {
    grid: Grid<Tile>,
    guard: Guard,
}

fn parse(input: &str) -> Map {
    let mut grid: Grid<Tile> = Grid::new(input, |chr, (col, row)| {
        match chr {
            // assuming that all guards start facing up
            '^' => Tile::Guard((col, row, Direction::Up)),
            _ => Tile::from_char(chr)
        }
    });

    // unfortunate limitation of grid implementation
    // grab guard position manually and replace with empty tile
    let mut guard: Option<Guard> = None;
    for t in &mut grid.content {
        match t {
            Tile::Guard(pos) => {
                guard = Some(Guard { col: pos.0, row: pos.1, dir: pos.2 });
                *t = Tile::Empty
            }
            _ => {}
        }
    };

    Map { grid, guard: guard.unwrap() }
} 

fn silver(map: &mut Map) -> usize {
    let mut visited_coords: HashSet<(usize, usize)> = HashSet::new();
    visited_coords.insert((map.guard.col, map.guard.row));

    let mut entry = map.grid.entry(map.guard.col, map.guard.row);
    let mut count = 1; // how many steps have we taken before hitting an obstacle

    loop {
        // walking direction index
        let walk_ind: (isize, isize) = match map.guard.dir {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        };

        // Peek next tile in guard's path
        match entry.offset(walk_ind.0*count, walk_ind.1*count) {
            Some((Tile::Empty, col, row)) => {
                count += 1;
                visited_coords.insert((col, row));
            },
            Some((Tile::Obstacle, newcol, newrow)) => {
                entry = map.grid.entry(
                    // hack: get guard's position before the obstacle
                    // this index is always valid
                    newcol.checked_add_signed(-walk_ind.0).unwrap(),
                    newrow.checked_add_signed(-walk_ind.1).unwrap(),
                );

                // turn guard and reset step counter
                map.guard.dir = map.guard.dir.turn();
                count = 1;
            },
            None => break, // guard walked out of bounds, we're done
            _ => panic!("guard tile needs to be removed"),
        }
    }

    visited_coords.len()
}

fn main() -> io::Result<()> {
    let input = read_input()?;
    let mut map = parse(&input);

    println!("silver: {}", silver(&mut map));

    Ok(())
}
