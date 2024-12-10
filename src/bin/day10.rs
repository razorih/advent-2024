use std::{collections::{HashSet, VecDeque}, io};

use advent::{grid::Grid, read_input};

fn count_trailheads(grid: &Grid<u32>, start: (usize, usize)) -> usize {
    let mut score = 0;

    let mut queue = VecDeque::new();
    queue.push_front(start);
    let mut explored = HashSet::new();

    while let Some((col, row)) = queue.pop_back() {
        if explored.contains(&(col, row)) {
            continue
        }

        explored.insert((col, row));
        let current = grid.entry(col, row);
        // unwrap here is ok since we only discover in-bounds points
        let current_height = current.at_offset(0, 0).unwrap();

        // check for trailhead end
        if current_height == 9 {
            score += 1;
            continue;
        }

        // discover trail continuations
        let dirs: [(isize, isize); 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)];
        for (dc, dr) in dirs {
            match current.offset(dc, dr) {
                Some((next_height, next_col, next_row)) => {
                    if next_height == current_height + 1 {
                        queue.push_front((next_col, next_row));
                    }
                },
                None => continue,
            }
        }
    }

    score
}

fn silver(grid: &Grid<u32>) -> usize {
    let mut trailheads = 0;

    for (position, height) in grid.iter_indexed() {
        if *height == 0 {
            trailheads += count_trailheads(grid, position);
        }
    }

    trailheads
}

fn main() -> io::Result<()> {
    let input = read_input()?;
    let grid = Grid::new(&input, |c, _| c.to_digit(10).unwrap());

    println!("silver: {}", silver(&grid));

    Ok(())
}
