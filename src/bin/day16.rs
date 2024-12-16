use std::{collections::{BinaryHeap, HashSet}, io};

use advent::{grid::Grid, read_input};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Wall, Empty, Start, End,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Dir {
    North, South, East, West,
}

impl Dir {
    /// Get a list of available directions from given facing
    fn avail_dirs(&self) -> [Self; 3] {
        match self {
            Dir::North => [Dir::West, Dir::North, Dir::East],
            Dir::South => [Dir::East, Dir::South, Dir::West],
            Dir::East => [Dir::North, Dir::East, Dir::South],
            Dir::West => [Dir::South, Dir::West, Dir::North],
        }
    }

    fn as_offset(&self) -> (isize, isize) {
        match self {
            Dir::North => ( 0, -1),
            Dir::South => ( 0,  1),
            Dir::East =>  ( 1,  0),
            Dir::West =>  (-1,  0),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Node {
    // cumulative cost until this node
    cost: usize,
    // metadata
    pos: (usize, usize),
    dir: Dir,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Visited {
    pos: (usize, usize),
    dir: Dir,
}

impl From<Node> for Visited {
    fn from(value: Node) -> Self {
        Self { pos: value.pos, dir: value.dir }
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // reversed order for min-heap
        self.cost.cmp(&other.cost).reverse()
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn silver(grid: &Grid<Tile>) -> usize {
    // uniform cost search

    let start = grid.find_one_pos_by(|it| it == Tile::Start).unwrap();

    let node = Node { cost: 0, pos: start, dir: Dir::East };
    let mut frontier: BinaryHeap<Node> = BinaryHeap::new();
    let mut expanded: HashSet<Visited> = HashSet::new();

    frontier.push(node);

    while let Some(node) = frontier.pop() {
        //println!("frontier has: {}", frontier.len());
        let curr = grid.entry(node.pos.0, node.pos.1);

        // reached end, return current cumulative cost
        if curr.at_offset(0, 0) == Some(Tile::End) {
            return node.cost
        }

        // mark current as visited
        expanded.insert(node.into());

        // discover next nodes
        for next_dir in node.dir.avail_dirs() {
            let (cost, moves) = if next_dir == node.dir {
                (1, true)
            } else {
                (1000, false)
            };

            let (dc, dr) = next_dir.as_offset();
            if let Some((next_tile, next_col, next_row)) = curr.offset(dc, dr) {
                if next_tile == Tile::Empty || next_tile == Tile::End {
                    let newnode = Node {
                        cost: node.cost + cost,
                        pos: if moves { (next_col, next_row) } else { node.pos },
                        dir: next_dir,
                    };

                    if !expanded.contains(&newnode.into()) {
                        frontier.push(newnode);
                    }
                }
            }
        }
    }

    unreachable!()
}

fn main() -> io::Result<()> {
    let input = read_input()?;
    let grid = Grid::new(&input, |chr, _| {
        match chr {
            '#' => Tile::Wall,
            '.' => Tile::Empty,
            'S' => Tile::Start,
            'E' => Tile::End,
            _ => panic!("invalid tile"),
        }
    });

    println!("silver: {}", silver(&grid));

    Ok(())
}

