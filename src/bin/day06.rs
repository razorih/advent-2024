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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

#[derive(Debug, Clone, Copy)]
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

fn gold(map: &mut Map) -> usize {
    // observations:
    // - guard is in a loop if she returns to the inserted obstacle
    // - obstacles can only be inserted in front of the guard

    let original_guard = map.guard;
    let mut ways = 0;

    // positions of past inserted obstacles
    let mut tried: HashSet<(usize, usize)> = HashSet::new();

    loop {
        // position of obstacle inserted during this iteration
        let mut obstacle: Option<(usize, usize, Direction)> = None;
        // reset map
        map.guard = original_guard;

        // how many iterations ago was an obstacle inserted
        let mut inserted_since: usize = 0;

        // walk until out of bounds or loop is encountered
        // each iteration either turns guard to the right
        // or walks one step forward
        loop {
            let entry = map.grid.entry(map.guard.col, map.guard.row);
            let walk_ind: (isize, isize) = match map.guard.dir {
                Direction::Up => (0, -1),
                Direction::Down => (0, 1),
                Direction::Left => (-1, 0),
                Direction::Right => (1, 0),
            };
            inserted_since += 1;

            // catch scuffed loops
            if inserted_since >= 10_000 {
                ways += 1;
                break
            }

            // println!("{:?}", map.guard);

            // Peek a tile in front the guard
            match entry.offset(walk_ind.0, walk_ind.1) {
                Some((Tile::Empty, col, row)) => {
                    // There may be a inserted obstacle in front of us now
                    // check if we need to turn because of it
                    if let Some((obs_col, obs_row, orig_dir)) = obstacle {
                        if obs_col == col && obs_row == row {
                            // println!("virtual obstacle! turning");

                            // check if we're looping
                            if inserted_since > 1 && orig_dir == map.guard.dir {
                                // println!("saw this virtual obstacle {} turns ago", inserted_since);
                                ways += 1;
                                break
                            }

                            map.guard.dir = map.guard.dir.turn();

                            // moving forward will be handled next iter
                            continue;
                        }
                    }

                    // check if obstacle can be inserted here
                    if obstacle.is_none() && !tried.contains(&(col, row)) {
                        // we haven't tried to insert obstacle here yet
                        // println!("inserted obstacle at ({}, {})", col, row);
                        obstacle = Some((col, row, map.guard.dir));
                        tried.insert((col, row));
                        inserted_since = 0;
                        continue;
                    }

                    // Move guard forward
                    map.guard.col = col;
                    map.guard.row = row;
                },
                Some((Tile::Obstacle, _, _)) => {
                    // println!("obstacle! turning");
                    map.guard.dir = map.guard.dir.turn();
                },
                None => {
                    // guard walked out of bounds
                    // and no obstacle was inserted
                    //
                    // this is identical to exit condition in silver()
                    if obstacle.is_none() {
                        return ways;
                    }

                    // obstacle was inserted, but guard walked out of bounds
                    // try next obstacle position
                    // println!("out of bounds!");
                    break;
                },
                _ => panic!("guard tile needs to be removed"),
            }
        }
    }
}

fn main() -> io::Result<()> {
    let input = read_input()?;
    let mut map = parse(&input);
    let original_guard = map.guard;

    println!("silver: {}", silver(&mut map));

    // reset guard position
    map.guard = original_guard;
    println!("gold: {}", gold(&mut map));

    Ok(())
}
