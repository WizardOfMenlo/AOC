use itertools::Itertools;
use std::fs::File;
use std::io::{self, prelude::*};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    /// Input file
    #[structopt(parse(from_os_str))]
    input: PathBuf,

    #[structopt(default_value = "3")]
    folds: usize,

    #[structopt(default_value = "2020")]
    target: i32,
}

fn main() -> io::Result<()> {
    let args = Opt::from_args();
    let file = File::open(&args.input)?;
    let lines: Vec<i32> = io::BufReader::new(file)
        .lines()
        .flatten()
        .map(|s| s.parse::<i32>())
        .flatten()
        .collect();

    let res = lines
        .iter()
        .enumerate()
        .flat_map(|(index, val)| {
            (0..(args.folds - 1))
                .map(|x| lines.iter().skip(index + x))
                .multi_cartesian_product()
                .find(|cs| cs.iter().map(|&x| x).sum::<i32>() == args.target - val)
                .map(|cs| (cs, *val))
        })
        .next();

    match res {
        Some((cs, val)) => println!(
            "Found {}, {:?}. Product = {}",
            val,
            cs,
            val * cs.iter().map(|&i| i).product::<i32>()
        ),
        None => println!("Sad times"),
    }

    Ok(())
}
