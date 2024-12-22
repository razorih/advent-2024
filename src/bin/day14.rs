use std::{fmt::Debug, io::{self, Read, Write}, str::FromStr};

use advent::read_input;

#[derive(Debug)]
struct Robot {
    pos: (usize, usize),
    vel: (isize, isize),
}

fn parse(input: &str) -> Vec<Robot> {
    let mut robots = Vec::new();

    // parse "T,T" pair
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
        let (x, y) = (robot.pos.0 as isize, robot.pos.1 as isize);
        let (vx, vy) = robot.vel;

        // calculate new position and find first positive modulo nx
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

fn print_robots(robots: &[Robot], width: usize, height: usize) {
    // we'll be printing one character at a time,
    // take stdout lock so it's fast
    let mut lock = io::stdout().lock();
    for row in 0..height {
        for col in 0..width {
            // unfortunate position check
            let has_robot = robots.into_iter().any(|robot| robot.pos == (col, row));
            let marker = if has_robot { "█" } else { " " }.as_bytes();

            let _ = lock.write(marker);
        }
        let _ = lock.write(b"\n");
    }
    let _ = lock.write(b"\n\n");
}

fn calculate_robot_variance(robots: &[Robot]) -> (f64, f64) {
    // theory: robots clumped together => small variance
    // calculate column and row variance separately
    let n = robots.len() as f64;
    let pos_sum = robots.into_iter().fold((0, 0), |acc, robot| (acc.0 + robot.pos.0, acc.1 + robot.pos.1));

    let mean = (pos_sum.0 as f64 / n, pos_sum.1 as f64 / n);
    let var = robots.into_iter().fold((0.0, 0.0), |acc, robot| {
        (
            acc.0 + (robot.pos.0 as f64 - mean.0)*(robot.pos.0 as f64 - mean.0),
            acc.1 + (robot.pos.1 as f64 - mean.1)*(robot.pos.1 as f64 - mean.1),
        )
    });
    let var = (var.0 / (n-1.), var.1 / (n-1.));

    var
}

fn solve<const GOLD: bool>(robots: &mut [Robot]) -> usize {
    let (width, height) = (101, 103);
    let total_seconds = if GOLD { 20_000 } else { 100 };

    for second in 0..total_seconds {
        simulate_robots(robots, width, height);

        if GOLD {
            let (var_x, _var_y) = calculate_robot_variance(robots);

            // noisy grid has column variance around 800
            if var_x < 350. {
                println!("suspicious x variance! var: {var_x}, second: {}", second + 1);
                // visually see if this is a tree, just press ctrl-c when you see it
                print_robots(robots, width, height);
            }
        }
    }

    // silver only
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

    println!("silver: {}", solve::</* false */true>(&mut robots));
    Ok(())
}

