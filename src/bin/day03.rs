use std::io;

use advent::read_input;

fn silver(input: &str) -> i64 {
    let mut result = 0;

    'mul: for (mul_start, _) in input.match_indices("mul(") {
        let mut num_buff = String::new();

        // mul operands
        let mut first: Option<i64> = None;
        let mut second: Option<i64> = None;
        
        // skip "mul(" portion of the string
        for chr in input[mul_start+4..].chars() {
            if chr.is_ascii_digit() {
                num_buff.push(chr);
                continue;
            }

            if chr == ',' {
                if !num_buff.is_empty() {
                    first = num_buff.parse().ok();
                    num_buff.clear();
                }
                continue;
            }

            if chr == ')' {
                if !num_buff.is_empty() {
                    second = num_buff.parse().ok();
                    num_buff.clear();

                    // multiply if we successfully parsed both numbers
                    if let (Some(a), Some(b)) = (first, second) {
                        result += a * b;
                    }
                }

                // this mul() op is ready, move on
                continue 'mul;
            }

            // println!("invalid mul(), got {}", chr);
            continue 'mul;
        }
    }

    result
} 

fn main() -> io::Result<()> {
    let input = read_input()?;

    println!("silver: {}", silver(&input));

    Ok(())
}
