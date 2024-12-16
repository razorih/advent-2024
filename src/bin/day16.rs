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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Node {
    // cumulative cost until this node
    cost: usize,
    // metadata
    pos: (usize, usize),
    dir: Dir,
    // gold only, list of all past coordinates visited on this path
    past: Vec<(usize, usize)>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Visited {
    pos: (usize, usize),
    dir: Dir,
}

impl From<&Node> for Visited {
    fn from(value: &Node) -> Self {
        Self { pos: value.pos, dir: value.dir }
    }
}

// custom comparator for Node since it contains a lot of metadata
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

fn solve(grid: &Grid<Tile>) -> (usize, usize) {
    // uniform cost search

    let start = grid.find_one_pos_by(|it| it == Tile::Start).unwrap();

    let node = Node { cost: 0, pos: start, dir: Dir::East, past: vec![start] };
    let mut frontier: BinaryHeap<Node> = BinaryHeap::new();
    let mut expanded: HashSet<Visited> = HashSet::new();

    frontier.push(node);

    let mut global_min = usize::MAX;

    let mut paths = Vec::new();
    while let Some(node) = frontier.pop() {
        let curr = grid.entry(node.pos.0, node.pos.1);

        // reached end
        if curr.at_offset(0, 0) == Some(Tile::End) {
            // UCS will always find global minimum first?
            if node.cost <= global_min {
                global_min = node.cost;
                paths.push(node); // record this node's path for the future
            }
            continue;
        }

        // mark current as visited
        expanded.insert((&node).into());

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
                    // create new node for next iterations
                    let newnode = Node {
                        cost: node.cost + cost,
                        pos: if moves { (next_col, next_row) } else { node.pos },
                        dir: next_dir,
                        past: {
                            let mut cloned = node.past.clone();
                            if moves {
                                cloned.push((next_col, next_row));
                            }
                            cloned
                        },
                    };

                    // expand frontier if we haven't been there before
                    if !expanded.contains(&(&newnode).into()) {
                        frontier.push(newnode);
                    }
                }
            }
        }
    }

    // now, "paths" contains all paths with minimum cost with associated paths
    // merge them all into a set and calculate number of unique coordinates
    let mut uniq: HashSet<(usize, usize)> = HashSet::new();
    for path in paths.into_iter() {
        uniq.extend(path.past.into_iter());
    }

    (global_min, uniq.len())
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

    let (silver, gold) = solve(&grid);
    println!("silver: {}", silver);
    println!("gold: {}", gold);

    Ok(())
}

