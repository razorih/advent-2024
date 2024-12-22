use std::{fmt::Debug, io::{self, Read, Write}, str::FromStr};

use advent::read_input;

#[derive(Debug)]
struct Robot {
    pos: (usize, usize),
    vel: (isize, isize),
}

fn parse(input: &str) -> Vec<Robot> {
    let mut robots = Vec::new();

    fn parse_pair<T>(pair: &str) -> (T, T)
    where
        T: FromStr + Debug,
        T::Err: Debug,
    {
        let (first, second) = pair.split_once(',').unwrap();
        (first.parse().unwrap(), second.parse().unwrap())
    }

    for line in input.lines() {
        if line.is_empty() { continue; }

        let (p, v) = line.split_once(' ').unwrap();
        let (_, p) = p.split_once('=').unwrap();
        let (_, v) = v.split_once('=').unwrap();

        robots.push(Robot {
            pos: parse_pair::<usize>(p),
            vel: parse_pair::<isize>(v),
        });
    }

    robots
}

fn simulate_robots(robots: &mut [Robot], width: usize, height: usize) {
    for robot in robots {
        // assume that velocity < grid_size
        let (x, y) = (robot.pos.0 as isize, robot.pos.1 as isize);
        let (vx, vy) = robot.vel;

        // calculate new position and find positive modulo
        // so that 0 <= nx < width
        let nx = (x + vx).rem_euclid(width as isize);
        let ny = (y + vy).rem_euclid(height as isize);

        robot.pos = (nx as usize, ny as usize);
    }
}

#[allow(dead_code)]
fn pause() {
    io::stdin().read(&mut [0]).unwrap();
}

#[allow(dead_code)]
fn print_robots(robots: &[Robot], width: usize, height: usize) {
    // take stdout lock for faster printing
    let mut lock = io::stdout().lock();
    for row in 0..height {
        for col in 0..width {
            // unfortunate position check
            let has_robot = robots.into_iter().any(|robot| robot.pos == (col, row));
            if has_robot {
                let _ = write!(lock, "█");
            } else {
                let _ = write!(lock, " ");
            }
        }
        let _ = write!(lock, "\n");
    }
    let _ = write!(lock, "\n\n");
}

// whether running a small test grid
const TEST: bool = false;

fn silver(robots: &mut [Robot]) -> usize {
    let (width, height) = if TEST { (11, 7) } else { (101, 103) };

    for _second in 0..100 {
        simulate_robots(robots, width, height);
        //print_robots(robots, width, height);
        //pause()
    }

    // find each robot's quadrant
    let mut quads: [usize; 4] = [0, 0, 0, 0];

    for robot in robots {
        let (x, y) = robot.pos;
        if x < width / 2 && y < height / 2 {
            quads[0] += 1; // top-left
        } else if x < width / 2 && y > height / 2 {
            quads[1] += 1; // top-right
        } else if x > width / 2 && y < height / 2 {
            quads[2] += 1; // bottom-right
        } else if x > width / 2 && y > height / 2 {
            quads[3] += 1; // bottom-left
        }
    }

    quads.into_iter().product()
}

fn main() -> io::Result<()> {
    let input = read_input()?;
    let mut robots = parse(&input);

    println!("silver: {}", silver(&mut robots));
    Ok(())
}

