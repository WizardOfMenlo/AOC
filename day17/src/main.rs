use itertools::iproduct;
use std::{collections::HashSet, fs::read_to_string};

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Coordinate {
    x: isize,
    y: isize,
    z: isize,
}

fn neighbours(co: Coordinate) -> impl Iterator<Item = Coordinate> {
    iproduct!((-1..=1), (-1..=1), (-1..=1))
        .filter(|(x, y, z)| !(*x == 0 && *y == 0 && *z == 0))
        .map(move |(a, b, c)| (co.x + a, co.y + b, co.z + c))
        .map(|(a, b, c)| Coordinate { x: a, y: b, z: c })
}

fn adj_occ(table: &HashSet<Coordinate>, co: Coordinate) -> usize {
    neighbours(co).filter(|n| table.contains(n)).count()
}

fn main() {
    let input = read_to_string("data/input").unwrap();
    let mut table: HashSet<_> = input
        .lines()
        .filter(|l| !l.trim().is_empty())
        .enumerate()
        .flat_map(|(x, l)| {
            l.trim()
                .chars()
                .enumerate()
                .flat_map(move |(y, c)| match c {
                    '#' => Some(Coordinate {
                        x: x as isize,
                        y: y as isize,
                        z: 0,
                    }),
                    '.' => None,
                    _ => panic!("Heh"),
                })
        })
        .collect();

    dbg!(&table);

    for _ in 0..6 {
        let mut new_table = table.clone();
        for &active in &table {
            let adj = adj_occ(&table, active);
            if adj == 2 || adj == 3 {
                new_table.insert(active);
            }
        }

        // Check inactive neighbours
        for i in table
            .iter()
            .flat_map(|a| neighbours(*a))
            .filter(|n| !table.contains(n))
        {
            if adj_occ(&table, i) == 3 {
                new_table.insert(i);
            }
        }

        table = new_table;
    }

    dbg!(table.len());
}

// */
