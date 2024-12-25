use std::{collections::{BinaryHeap, HashSet}, io};

use advent::{grid::Grid, read_input};

#[derive(Debug, Clone, Copy, Default)]
enum Tile {
    #[default]
    Safe,
    Corrupted,
}

fn parse(input: &str, count: usize) -> Grid<Tile> {
    let mut grid: Grid<Tile> = Grid::empty(71, 71);

    for line in input.lines().take(count) {
        if line.is_empty() { continue }
        let (col, row) = line.split_once(',').unwrap();
        let col: usize = col.parse().unwrap();
        let row: usize = row.parse().unwrap();

        if let Some(space) = grid.at_mut(col, row) {
            *space = Tile::Corrupted;
        }
    }

    grid
}

#[derive(Debug, Clone, Copy)]
struct Node {
    // position
    pos: (usize, usize),
    // current search cost (steps walked)
    cost: usize,
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}
impl Eq for Node {}

// custom Ord implementation so that we can build a min-heap
impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost.cmp(&other.cost).reverse()
    }
}
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn silver(grid: &Grid<Tile>) -> usize {
    // basic uniform cost search
    let mut frontier: BinaryHeap<Node> = BinaryHeap::new();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    let start = (0, 0);

    frontier.push(Node { pos: start, cost: 0});

    while let Some(node) = frontier.pop() {
        if node.pos == (70, 70) {
            return node.cost
        }

        visited.insert(node.pos);

        let dirs: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        let here = grid.entry(node.pos.0, node.pos.1);
        for (dc, dr) in dirs {
            let Some((Tile::Safe, col, row)) = here.offset(dc, dr) else { continue };

            let in_frontier = frontier.iter().any(|node| node.pos == (col, row));
            if !visited.contains(&(col, row)) && !in_frontier {
                frontier.push(Node { pos: (col, row), cost: node.cost + 1 });
            }
        }
    }

    unreachable!()
}

fn main() -> io::Result<()> {
    let input = read_input()?;
    let grid = parse(&input, 1024);

    println!("silver: {}", silver(&grid));

    Ok(())
}
