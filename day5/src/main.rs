use std::{collections::HashSet, fs::read_to_string};

use std::iter::FromIterator;
use std::usize;

fn main() {
    let input = read_to_string("data/input").unwrap();
    let mut parsed: Vec<_> = input
        .lines()
        .map(|l| {
            let rows = &l[0..7].replace('F', "0").replace('B', "1");
            let rows = usize::from_str_radix(rows, 2).unwrap();

            let cols = &l[7..].replace('L', "0").replace('R', "1");
            let cols = usize::from_str_radix(cols, 2).unwrap();

            (rows, cols, 8 * rows + cols)
        })
        .collect();

    parsed.sort_by(|(_, _, id), (_, _, id2)| id2.cmp(id));

    let (min, max) = (parsed[parsed.len() - 1].2, parsed[0].2);

    let ids: HashSet<_> = HashSet::from_iter(parsed.into_iter().map(|s| s.2));

    println!("{:?}", ((min + 1)..max).find(|i| !ids.contains(i)));
}
