use std::io;

use advent::read_input;

#[derive(Debug)]
enum Levels {
    Increasing,
    Decreasing,
}

impl Levels {
    fn is_valid(&self, prev: usize, curr: usize) -> bool {
        // check adjacent level difference
        let diff = curr.abs_diff(prev);
        if diff < 1 || diff > 3 {
            return false
        }

        // check all increasing or decreasing
        match self {
            Levels::Increasing => curr > prev,
            Levels::Decreasing => curr < prev,
        }
    }
}

fn silver(input: &str) -> usize {
    let mut safe: usize = 0;

    for report in input.lines() {
        let levels: Vec<usize> = report.split_ascii_whitespace()
            .map_while(|num| num.parse::<usize>().ok())
            .collect();

        // look at first two elements to determine direction
        let dir = if levels[0] < levels[1] {
            Levels::Increasing
        } else {
            Levels::Decreasing
        };

        let is_safe = levels.windows(2).all(|win| dir.is_valid(win[0], win[1]));
        if is_safe {
            safe += 1;
        }
    }

    safe
}

fn main() -> io::Result<()> {
    let input = read_input()?;

    println!("silver: {}", silver(&input));

    Ok(())
}
