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


fn main() -> io::Result<()> {
    let input = read_input()?;
    let lists: Lists = input.parse().unwrap();

    //println!("{:?}", lists);
    println!("silver: {}", silver(&lists));

    Ok(())
}
