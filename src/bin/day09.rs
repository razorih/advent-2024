use std::{io, iter::repeat_n};

use advent::read_input;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum File {
    Id(usize),
    Empty,
}

impl File {
    fn is_empty(&self) -> bool {
        match self {
            File::Id(_) => false,
            File::Empty => true,
        }
    }
}

fn parse(input: &str) -> Vec<File> {
    let mut files = Vec::new();
    let mut latest_id: usize = 0;
    let mut parsing_id = true; // is next number we get a file id or empty?
    let num_iter = input.trim()
        .chars()
        .map(|chr| chr.to_digit(10).unwrap());

    for num in num_iter {
        // create `num` times repeating iterator for file id or empty
        let extender = if parsing_id {
            let temp = repeat_n(File::Id(latest_id), num as usize);
            latest_id += 1;
            parsing_id = false;
            temp
        } else {
            parsing_id = true;
            repeat_n(File::Empty, num as usize)
        };

        files.extend(extender);
    }

    files
}

fn silver(files: &mut [File]) -> usize {
    // meet-in-the-middle cursors
    let mut head = 0;
    let mut tail = files.len() - 1;

    'outer: loop {
        while !files[head].is_empty() {
            if head == tail { break 'outer }
            head += 1;
        }

        while files[tail].is_empty() {
            if head == tail { break 'outer }
            tail -= 1;
        }

        files.swap(head, tail);
    }


    let mut checksum: usize = 0;
    for (i, file) in files.iter().enumerate() {
        match file {
            File::Id(id) => checksum += id * i,
            File::Empty => break, // fuse, no files after first empty
        }
    }

    checksum
}

fn main() -> io::Result<()> {
    let input = read_input()?;
    let mut files = parse(&input);

    println!("silver: {}", silver(&mut files));

    Ok(())
}
