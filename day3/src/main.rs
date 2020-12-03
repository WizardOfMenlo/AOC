use core::panic;
use std::fs::File;
use std::io::{self, prelude::*};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    /// Input file
    #[structopt(parse(from_os_str))]
    input: PathBuf,
}

fn get_count(table: &[Vec<bool>], slope_down: usize, slope_right: usize) -> usize {
    let (_, count) = table
        .iter()
        .enumerate()
        .fold((0, 0), |(pos, count), (index, line)| {
            if index % slope_down == 0 {
                (
                    pos + slope_right,
                    if line[pos % line.len()] {
                        count + 1
                    } else {
                        count
                    },
                )
            } else {
                (pos, count)
            }
        });

    count
}

fn main() -> io::Result<()> {
    let args = Opt::from_args();
    let f = File::open(args.input)?;
    let table: Vec<_> = io::BufReader::new(f)
        .lines()
        .flatten()
        .map(|l| {
            l.chars()
                .map(|s| match s {
                    '#' => true,
                    '.' => false,
                    _ => panic!("What"),
                })
                .collect::<Vec<_>>()
        })
        .collect();

    let slopes = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    let counts = slopes
        .iter()
        .map(|(r, d)| get_count(&table, *d, *r))
        .product::<usize>();

    println!("{}", counts);

    Ok(())
}
