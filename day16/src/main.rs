use std::{collections::HashSet, fs::read_to_string};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Condition {
    name: String,
    ranges: Vec<(usize, usize)>,
}

#[derive(Debug)]
struct Ticket {
    values: Vec<usize>,
}

impl Ticket {
    fn invalid_for_all<'a>(&self, it: impl Iterator<Item = &'a Condition> + Clone) -> Vec<usize> {
        self.values
            .iter()
            .filter(|&v| {
                it.clone()
                    .map(|c| c.ranges.iter())
                    .flatten()
                    .all(|(min, max)| !(min <= v && v <= max))
            })
            .copied()
            .collect()
    }

    fn valid_conditions<'a>(
        &self,
        it: impl Iterator<Item = &'a Condition> + Clone,
    ) -> Vec<HashSet<Condition>> {
        self.values
            .iter()
            .map(|v| {
                it.clone()
                    .filter(|c| c.ranges.iter().any(|(min, max)| min <= v && v <= max))
                    .cloned()
                    .collect()
            })
            .collect()
    }
}

impl From<&str> for Ticket {
    fn from(x: &str) -> Self {
        Ticket {
            values: x
                .trim()
                .split(',')
                .flat_map(|t| t.parse::<usize>().ok())
                .collect(),
        }
    }
}

fn main() {
    let input = read_to_string("data/input").unwrap();
    let parts: Vec<_> = input.split("\n\n").collect();
    let ranges = parts[0];
    let conditions: Vec<_> = ranges
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|line| {
            let sections: Vec<_> = line.split(':').map(|p| p.trim()).collect();
            let name = sections[0].to_string();
            let ranges = sections[1]
                .split("or")
                .map(|p| {
                    let range: Vec<_> = p.trim().split('-').map(|i| i.trim()).collect();
                    (
                        range[0].parse::<usize>().unwrap(),
                        range[1].parse::<usize>().unwrap(),
                    )
                })
                .collect();
            Condition { name, ranges }
        })
        .collect();

    let my_ticket: Ticket = parts[1].lines().skip(1).next().unwrap().into();

    let others: Vec<_> = parts[2].lines().skip(1).map(Ticket::from).collect();

    dbg!(others
        .iter()
        .flat_map(|t| t.invalid_for_all(conditions.iter()))
        .sum::<usize>());

    let valid: Vec<_> = others
        .into_iter()
        .filter(|t| t.invalid_for_all(conditions.iter()).is_empty())
        .collect();

    dbg!(&conditions);
    dbg!(&my_ticket);

    let mut initial_guess = valid[0].valid_conditions(conditions.iter());
    for t in valid.into_iter() {
        let new_guess = t.valid_conditions(conditions.iter());
        for i in 0..initial_guess.len() {
            initial_guess[i] = initial_guess[i]
                .intersection(&new_guess[i])
                .cloned()
                .collect();
        }
    }

    loop {
        let mut done = true;
        for i in 0..initial_guess.len() {
            // We still have to improve our guesses
            if initial_guess[i].len() != 1 {
                done = false;
                continue;
            }

            let certain = initial_guess[i].iter().next().unwrap().clone();

            for j in 0..my_ticket.values.len() {
                if i == j {
                    continue;
                }
                initial_guess[j].remove(&certain);
            }
        }
        if done {
            break;
        }
    }

    let mut counter = 1;
    for (i, c) in initial_guess.iter().enumerate() {
        if c.iter().next().unwrap().name.contains("departure") {
            counter *= my_ticket.values[i] as u64;
        }
    }

    dbg!(counter);
}
