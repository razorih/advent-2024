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

/// Check if given levels are valid.
/// Returns invalid position if not
fn check(levels: &[usize]) -> (bool, Option<usize>) {
    // look at first two elements to determine direction
    let dir = if levels[0] < levels[1] {
        Levels::Increasing
    } else {
        Levels::Decreasing
    };

    for (i, w) in levels.windows(2).enumerate() {
        if !dir.is_valid(w[0], w[1]) {
            return (false, Some(i));
        }
    }

    (true, None)
}

fn gold(input: &str) -> usize {
    let mut safe: usize = 0;

    for report in input.lines() {
        let mut levels: Vec<usize> = report.split_ascii_whitespace()
            .map_while(|num| num.parse::<usize>().ok())
            .collect();

        if let (true, _) = check(&levels) {
            println!("safe without removal: {:?}", &levels);
            safe += 1;
            continue
        }

        if let (false, Some(invalid_i)) = check(&levels) {
            // remove potentially bad levels and check again
            let mut forwards = levels.clone();
            let mut backwards = levels.clone();
            levels.remove(invalid_i);
            forwards.remove(invalid_i + 1); // index is always valid due to .windows(2) in check()
            backwards.remove(invalid_i.saturating_sub(1));

            if let (true, _) = check(&levels) {
                println!("safe with removal: {:?} (removed index {})", &levels, invalid_i);
                safe += 1;
            } else if let (true, _) = check(&forwards) {
                println!("safe with forward removal: {:?}", &forwards);
                safe += 1;
            } else if let (true, _) = check(&backwards) {
                println!("safe with backwards removal: {:?}", &backwards);
                safe += 1;
            } else {
                println!("truly unsafe: {:?}", &levels);
            }
        }
    }

    safe
}

fn silver(input: &str) -> usize {
    let mut safe: usize = 0;

    for report in input.lines() {
        let levels: Vec<usize> = report.split_ascii_whitespace()
            .map_while(|num| num.parse::<usize>().ok())
            .collect();

        if let (true, _) = check(&levels) {
            safe += 1;
        }
    }

    safe
}

fn main() -> io::Result<()> {
    let input = read_input()?;

    println!("silver: {}", silver(&input));
    println!("gold: {}", gold(&input));

    Ok(())
}
