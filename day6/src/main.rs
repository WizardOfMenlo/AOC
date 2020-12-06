use std::fs::read_to_string;
use std::{collections::HashSet, iter::FromIterator};

fn main() {
    let input = read_to_string("data/input").unwrap();
    let count = input
        .split("\n\n")
        .map(|group| {
            let mut it = group.split('\n').filter(|s| !s.trim().is_empty()).map(|s| {
                let s: HashSet<_> = HashSet::from_iter(s.trim().chars());
                s
            });

            let can = it.next().unwrap();

            it.fold(can, |acc, i| {
                // Change to union for part 1
                HashSet::from_iter(acc.intersection(&i).cloned())
            })
            .len()
        })
        .sum::<usize>();

    println!("{}", count);
}
