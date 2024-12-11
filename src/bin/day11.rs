use std::io;

use advent::read_input;

fn split_number(num: usize) -> (usize, usize) {
    // todo: real algo
    let num = num.to_string();
    let (upper, lower) = num.split_at(num.len() / 2);
    (upper.parse().unwrap(), lower.parse().unwrap())
}

fn has_even_digits(num: usize) -> bool {
    (num.ilog10() + 1) % 2 == 0
}

fn blink(stones: &mut Vec<usize>) {
    // current stone index being handled
    let mut i = 0;

    loop {
        if i == stones.len() {
            return
        }

        match stones[i] {
            0 => {
                stones[i] = 1;
                i += 1;
            },
            even if has_even_digits(even) => {
                let split = split_number(even);
                stones[i] = split.0;
                stones.insert(i + 1, split.1);
                i += 2; // skip handling just inserted number
            },
            other => {
                stones[i] = other * 2024;
                i += 1;
            },
        }
    }
}

fn silver(stones: &mut Vec<usize>) -> usize {
    for _ in 0..25 {
        blink(stones);
    }

    stones.len()
}

fn main() -> io::Result<()> {
    let input = read_input()?;
    let mut stones: Vec<usize> = input.trim()
        .split(' ')
        .map(|n| n.parse().unwrap())
        .collect();

    println!("silver: {}", silver(&mut stones));

    Ok(())
}
