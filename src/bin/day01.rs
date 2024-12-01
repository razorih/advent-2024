use std::{io, str::FromStr};

use advent::read_input;

#[derive(Debug)]
struct Lists {
    left: Vec<usize>,
    right: Vec<usize>,
}

impl FromStr for Lists {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut left_list: Vec<usize> = Vec::new();
        let mut right_list: Vec<usize> = Vec::new();

        for line in s.lines() {
            if let Some((left, right)) = line.split_once("   ") {
                let left: usize = left.parse().unwrap();
                let right: usize = right.parse().unwrap();

                left_list.push(left);
                right_list.push(right);
            }
        }

        left_list.sort_unstable();
        right_list.sort_unstable();
        
        Ok(Lists { left: left_list, right: right_list })
    }
}

fn silver(lists: &Lists) -> usize {
    let mut distance: usize = 0;
    let length = lists.left.len();

    for i in 0..length {
        distance += lists.left[i].abs_diff(lists.right[i]);
    }

    distance
}

fn gold(lists: &Lists) -> usize {
    let mut similarity: usize = 0;
    let length = lists.left.len();

    let mut skip_index: usize = 0;
    for i in 0..length {
        let current = lists.left[i];
        let mut first = true;

        let mut in_other: usize = 0;
        for j in skip_index..length {
            if lists.right[j] == current {
                in_other += 1;

                // if this is the first time we see `current` number
                // it means that everything below index `j` are smaller than `current`.
                // thus in the future we can skip directly to this index 
                if first {
                    skip_index = j;
                    first = false;
                }
            }

            // lists are sorted, can't find current number anymore
            if lists.right[j] > current {
                break;
            }
        }

        similarity += current * in_other;
    }

    similarity
}

fn main() -> io::Result<()> {
    let input = read_input()?;
    let lists: Lists = input.parse().unwrap();

    //println!("{:?}", lists);
    println!("silver: {}", silver(&lists));
    println!("gold: {}", gold(&lists));

    Ok(())
}
