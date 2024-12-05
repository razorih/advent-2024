use std::io;

use advent::read_input;

/// Order from string "before|after"
#[derive(Debug, PartialEq, Eq)]
struct Order {
    before: usize,
    after: usize,
}

enum Validity {
    Valid,
    Invalid,
    DontCare,
}

impl Order {
    fn new(before: usize, after: usize) -> Self {
        Self { before, after }
    }

    fn validate(&self, before: usize, after: usize) -> Validity {
        // Explicitly valid order
        if self.before == before && self.after == after {
            return Validity::Valid
        }

        // Reverse order, order is invalid
        if self.before == after && self.after == before {
            return Validity::Invalid
        }

        // One or two numbers don't match, this order doesn't tell anything
        Validity::DontCare
    }
}

#[derive(Debug)]
struct Update {
    numbers: Vec<usize>,
}

impl Update {
    fn from_vec(numbers: Vec<usize>) -> Self {
        Self { numbers }
    }

    fn mid(&self) -> usize {
        // see nightly `usize::midpoint`
        self.numbers[self.numbers.len() >> 1]
    }
}

fn parse(input: &str) -> (Vec<Order>, Vec<Update>) {
    let mut orders: Vec<Order> = Vec::new();
    let mut updates: Vec<Update> = Vec::new();

    let mut parsing_orders = true;

    for line in input.lines() {
        // First blank, switch parsing mode
        if line.is_empty() {
            parsing_orders = false;
            continue;
        }

        if parsing_orders {
            let (first, second) = line.split_once('|')
                .map(|(a, b)| (a.parse::<usize>().unwrap(), b.parse::<usize>().unwrap()))
                .unwrap();
            orders.push(Order::new(first, second));
        } else {
            let update: Vec<usize> = line.split(',')
                .map(|n| n.parse().unwrap())
                .collect();
            updates.push(Update::from_vec(update));
        }
    }

    (orders, updates)
}

fn silver(orders: &[Order], updates: &[Update]) -> usize {
    let mut result = 0;

    'updates: for update in updates {
        let mut seems_valid = false;

        // analyze pairs of a number and every number after it
        for i in 0..update.numbers.len() {
            for j in i..update.numbers.len() {
                if i == j { continue; }

                for order in orders {
                    match order.validate(update.numbers[i], update.numbers[j]) {
                        Validity::Valid => seems_valid = true, // found at least one valid rule
                        Validity::Invalid => continue 'updates, // update is invalid, move on
                        Validity::DontCare => continue,
                    }
                }
            }
        }

        if seems_valid { result += update.mid() }
    }

    result
}

fn main() -> io::Result<()> {
    let input = read_input()?;
    let (orders, updates) = parse(&input);

    // println!("orders: {:#?}", orders);
    // println!("updates: {:#?}", updates);

    println!("silver: {}", silver(&orders, &updates));

    Ok(())
}
