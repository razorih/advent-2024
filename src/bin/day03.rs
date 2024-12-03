use std::io;

use advent::read_input;

/// Try to parse a single `mul(xx,yy)` instruction.
///
/// Returns [`None`] if instruction is not valid
fn try_parse_mul(input: &str) -> Option<i64> {
    let mut num_buff = String::new();
    let mut first: Option<i64> = None;

    // skip "mul(" portion of the string
    for chr in input[4..].chars() {
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
                let second: Option<i64> = num_buff.parse().ok();
                num_buff.clear();

                // mul() op is ready, return result if valid
                if let (Some(a), Some(b)) = (first, second) {
                    return Some(a * b);
                }
            }

            // invalid operands
            // either or both operands failed to parse
            // or num_buff were empty (invalid digits)
            return None
        }

        // println!("invalid mul(), got {}", chr);
        return None
    }

    unreachable!()
}

fn gold(input: &str) -> i64 {
    let mut result = 0;
    let mut enabled = true; // are mul() ops enabled?

    for (i, chr) in input.char_indices() {
        if chr == 'm' {
            if let Some("mul(") = input.get(i..i+4) {
                if enabled {
                    result += try_parse_mul(&input[i..]).unwrap_or(0);
                }
            }
        }

        if chr == 'd' {
            if let Some("do()") = input.get(i..i+4) {
                enabled = true;
            }

            if let Some("don't()") = input.get(i..i+7) {
                enabled = false;
            }
        }
    }

    result
} 

fn silver(input: &str) -> i64 {
    let mut result = 0;

    for (mul_start, _) in input.match_indices("mul(") {
        result += try_parse_mul(&input[mul_start..]).unwrap_or(0);
    }

    result
}

fn main() -> io::Result<()> {
    let input = read_input()?;

    println!("silver: {}", silver(&input));
    println!("gold: {}", gold(&input));

    Ok(())
}
