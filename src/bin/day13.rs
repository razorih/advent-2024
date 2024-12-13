use std::io;

use advent::read_input;

#[derive(Debug)]
struct Machine {
    a: (usize, usize),
    b: (usize, usize),
    prize: (usize, usize),
}

// scuffed state machine based parser
fn parse(input: &str) -> Vec<Machine> {
    let mut machines = Vec::new();

    let mut a = (0, 0);
    let mut b = (0, 0);

    // parse `nn` from expr `A DELIM nn`
    fn parse_expr<const DELIM: char>(op: &str) -> usize {
        let Some((_, num)) = op.split_once(DELIM) else { panic!() };
        num.parse().unwrap()
    }

    enum State { A, B, Prize }
    let mut state = State::A;

    for line in input.lines() {
        if line.is_empty() {
            state = State::A;
            continue;
        }

        match state {
            State::A => {
                let Some((x, y)) = line[10..].split_once(", ") else { panic!() };
                a = (parse_expr::<'+'>(x), parse_expr::<'+'>(y));

                state = State::B;
            },
            State::B => {
                let Some((x, y)) = line[10..].split_once(", ") else { panic!() };
                b = (parse_expr::<'+'>(x), parse_expr::<'+'>(y));

                state = State::Prize;
            },
            State::Prize => {
                let Some((x, y)) = line[7..].split_once(", ") else { panic!() };
                let prize = (parse_expr::<'='>(x), parse_expr::<'='>(y));

                machines.push(Machine { a, b, prize });
                state = State::A;
            },
        }
    }

    machines
}

fn solve_machine(machine: &Machine) -> Option<usize> {
    // find integers (u, v) such that
    // { u*ax + v*bx == px
    // { u*bx + v*by == py
    // i.e.
    // [ax bx]*[u] = [px]
    // [ay by] [v]   [py]

    // extract and cast everything for math stuff
    let (ax, ay) = (machine.a.0 as f64, machine.a.1 as f64);
    let (bx, by) = (machine.b.0 as f64, machine.b.1 as f64);
    let (px, py) = (machine.prize.0 as f64, machine.prize.1 as f64);

    // solve via Cramer's rule
    let det = ax*by - bx*ay;
    debug_assert_ne!(det, 0.0);

    // determinants of matrices where one column is replaced with prize vector
    let det_p_left = px*by - bx*py;
    let det_p_right = ax*py - px*ay;

    let u = det_p_left / det;
    let v = det_p_right / det;

    // maybe working integer-ishness check
    if u.fract() < 1e-10 && v.fract() < 1e-10 {
        Some((3.*u + v) as usize)
    } else {
        None
    }
}

fn silver(machines: &[Machine]) -> usize {
    let mut total = 0;
    for machine in machines {
        if let Some(tokens) = solve_machine(&machine) {
            total += tokens;
        }
    }

    total
}

fn main() -> io::Result<()> {
    let input = read_input()?;
    let machines = parse(&input);

    println!("silver: {}", silver(&machines));

    Ok(())
}
