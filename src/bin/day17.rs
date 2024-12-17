use std::io;

use advent::read_input;

#[derive(Debug, Clone, Copy)]
enum Op {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

impl Op {
    fn as_u64(&self) -> u64 {
        match self {
            Op::Adv => 0,
            Op::Bxl => 1,
            Op::Bst => 2,
            Op::Jnz => 3,
            Op::Bxc => 4,
            Op::Out => 5,
            Op::Bdv => 6,
            Op::Cdv => 7,
        }
    }
}

impl From<u8> for Op {
    fn from(value: u8) -> Self {
        match value {
            0 => Op::Adv,
            1 => Op::Bxl,
            2 => Op::Bst,
            3 => Op::Jnz,
            4 => Op::Bxc,
            5 => Op::Out,
            6 => Op::Bdv,
            7 => Op::Cdv,
            _ => panic!("invalid op"),
        }
    }
}


struct Vm {
    // registers
    a: u64,
    b: u64,
    c: u64,
    // instruction pointer
    ip: u64,
    // operations
    ops: Vec<Op>,
}

impl Vm {
    fn new(registers: (u64, u64, u64), ops: Vec<Op>) -> Self {
        Self {
            a: registers.0,
            b: registers.1,
            c: registers.2,
            ip: 0,
            ops,
        }
    }

    fn combo(&self, combo: u64) -> u64 {
        match combo {
            0..=3 => combo,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            7 => panic!("reserved combo op"),
            _ => panic!("invalid combo"),
        }
    }

    fn tick(&mut self) -> bool {
        if self.ip >= self.ops.len() as u64 {
            return false
        }

        let op = self.ops[self.ip as usize];
        let literal = self.ops[self.ip as usize + 1].as_u64();
        let combo = self.combo(literal);

        match op {
            Op::Adv => {
                self.a = self.a / 2_u64.pow(combo as u32);
                self.ip += 2;
            },
            Op::Bxl => {
                self.b = self.b ^ literal;
                self.ip += 2;
            },
            Op::Bst => {
                self.b = combo % 8;
                self.ip += 2;
            },
            Op::Jnz => {
                if self.a == 0 { self.ip += 2; return true }
                self.ip = literal;
            },
            Op::Bxc => {
                self.b = self.b ^ self.c;
                self.ip += 2;
            },
            Op::Out => {
                print!("{},", combo % 8);
                self.ip += 2;
            },
            Op::Bdv => {
                self.b = self.a / 2_u64.pow(combo as u32);
                self.ip += 2;
            },
            Op::Cdv => {
                self.c = self.a / 2_u64.pow(combo as u32);
                self.ip += 2;
            },
        }

        true
    }
}

fn parse(input: &str) -> ((u64, u64, u64), Vec<Op>) {
    let (regs, ops) = input.split_once("\n\n").unwrap();

    let regs: Vec<u64> = regs.lines().map(|line| {
        let val = line.split_whitespace().last().unwrap();
        val.parse::<u64>().unwrap()
    }).collect();
    let regs = (regs[0], regs[1], regs[2]);

    // skip "Program: " prefix
    let ops = ops[8..].trim().split(',')
        .map(|op| Op::from(op.parse::<u8>().unwrap()))
        .collect();

    (regs, ops)
}

fn silver(reg: (u64, u64, u64), ops: Vec<Op>) {
    let mut vm = Vm::new(reg, ops);

    // tick vm until it halts
    while vm.tick() {}
}

fn main() -> io::Result<()> {
    let input = read_input()?;
    let (registers, ops) = parse(&input);

    println!("silver");
    silver(registers, ops);

    Ok(())
}
