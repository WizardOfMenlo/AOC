use regex::Regex;
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

struct Format {
    target: char,
    min: usize,
    max: usize,
}

fn main() -> io::Result<()> {
    let args = Opt::from_args();
    let f = File::open(args.input)?;
    let re = Regex::new("(\\d+)-(\\d+) (\\w): (\\w+)").unwrap();

    let res = io::BufReader::new(f)
        .lines()
        .flatten()
        .map(|l| {
            let captures = re.captures(&l).unwrap();
            let f = Format {
                min: captures
                    .get(1)
                    .map(|s| s.as_str().parse::<usize>().unwrap())
                    .unwrap(),
                max: captures
                    .get(2)
                    .map(|s| s.as_str().parse::<usize>().unwrap())
                    .unwrap(),

                target: captures
                    .get(3)
                    .map(|s| s.as_str().chars().next().unwrap())
                    .unwrap(),
            };
            (f, captures.get(4).unwrap().as_str().to_string())
        })
        .filter(|(f, s)| {
            /* Part 2
            let first = s.chars().skip(f.min - 1).next().unwrap();
            let second = s.chars().skip(f.max - 1).next().unwrap();
            (first == f.target) ^ (second == f.target
            */

            let count = s.chars().filter(|&c| c == f.target).count();
            f.min <= count && count <= f.max
        })
        .count();

    println!("Final Result {}", res);

    Ok(())
}
