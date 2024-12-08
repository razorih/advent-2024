use std::{io, str::FromStr};

use advent::read_input;

#[derive(Debug)]
struct Equation {
    result: usize,
    operands: Vec<usize>,
}

impl FromStr for Equation {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((result, operands)) = s.split_once(':') else {
            return Err(": not found")
        };

        let Ok(result) = result.parse() else {
            return Err("failed to parse equation result")
        };

        let operands = operands.trim()
            .split_ascii_whitespace()
            .filter_map(|num| num.parse().ok())
            .collect();

        Ok(Equation { result, operands })
    }
}

fn parse(input: &str) -> Vec<Equation> {
    input.lines()
        .filter_map(|line| line.parse().ok())
        .collect()
}

fn silver(equations: &[Equation]) -> usize {
    let mut result = 0;

    for eq in equations {
        let ops = eq.operands.as_slice();

        // keep a stack of (total, [remaining_numbers])
        let mut stack: Vec<(usize, &[usize])> = Vec::new();

        // seed the stack
        stack.push((ops[0], &ops[1..]));
        stack.push((ops[0], &ops[1..]));

        while let Some((total, rem)) = stack.pop() {
            // found solution!
            if rem.len() == 0 && total == eq.result {
                result += eq.result;
                break
            }

            // total went over the result, not possible
            if total > eq.result {
                continue
            }

            // ran out of operands, this combination is not possible
            if rem.len() == 0 {
                continue
            }

            // discover next combinations
            stack.push((rem[0] + total, &rem[1..]));
            stack.push((rem[0] * total, &rem[1..]));
        }
    }

    result
}

fn main() -> io::Result<()> {
    let input = read_input()?;
    let equations = parse(&input);

    // println!("eqs: {:?}", equations);
    println!("silver: {}", silver(&equations));

    Ok(())
}
