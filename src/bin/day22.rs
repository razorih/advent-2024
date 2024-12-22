use std::io;

use advent::read_input;

fn parse(input: &str) -> Vec<u64> {
    input.lines()
        .map_while(|num| num.parse().ok())
        .collect()
}

#[inline]
fn mix(value: u64, secret: u64) -> u64 { value ^ secret }

#[inline]
fn prune(secret: u64) -> u64 { secret % 16777216 }

fn tick(secret: u64) -> u64 {
    let secret = mix(secret * 64, secret);
    let secret = prune(secret);

    let secret = mix(secret / 32, secret);
    let secret = prune(secret);

    let secret = mix(secret * 2048, secret);
    let secret = prune(secret);

    secret
}

fn main() -> io::Result<()> {
    let input = read_input()?;
    let mut numbers = parse(&input);
    
    //let mut test = 123_u64;
    //for _ in 0..10 {
    //    let new = tick(test);
    //    println!("{:>16} -> {:<16}", test, new);
    //    test = new;
    //}

    for num in numbers.iter_mut() {
        for _ in 0..2000 {
            *num = tick(*num);
        }
    }

    println!("silver: {}", numbers.into_iter().sum::<u64>());

    Ok(())
}
