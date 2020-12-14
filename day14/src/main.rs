use std::{collections::HashMap, fs::read_to_string};

struct Mask {
    zero_mask: usize,
    one_mask: usize,
    x_mask: usize,
}

impl Mask {
    fn apply_to_value(&self, mut x: usize) -> usize {
        x |= self.one_mask;
        x &= !self.zero_mask;
        x
    }

    fn possible_addresses(&self, address: usize) -> Vec<usize> {
        let base = address | self.one_mask;
        // Check if the mask at that is 1
        (0..64)
            .filter(|i| (1 << i) & self.x_mask > 0)
            .fold(vec![base], |acc, i| {
                acc.into_iter()
                    .flat_map(|e| vec![e | (1 << i), e & !(1 << i)])
                    .collect()
            })
    }
}

struct Set {
    address: usize,
    val: usize,
}

enum Instr {
    Mask(Mask),
    Set(Set),
}

impl From<&str> for Instr {
    fn from(input: &str) -> Self {
        if input.contains("mask") {
            Instr::Mask(parse_mask(input))
        } else {
            Instr::Set(parse_set(input))
        }
    }
}

fn parse_mask(l: &str) -> Mask {
    let mask_values: Vec<_> = ['0', '1', 'X']
        .iter()
        .map(|val| {
            l.replace("mask = ", "")
                .trim()
                .chars()
                .map(|i| if i == *val { 1 } else { 0 })
                .fold(0, |acc, x| 2 * acc + x)
        })
        .collect();

    Mask {
        zero_mask: mask_values[0],
        one_mask: mask_values[1],
        x_mask: mask_values[2],
    }
}

fn parse_set(l: &str) -> Set {
    let parts: Vec<_> = l.split("=").map(|p| p.trim()).collect();
    let val = parts[1].parse::<usize>().unwrap();
    let address = parts[0]
        .replace("mem[", "")
        .replace(']', "")
        .trim()
        .parse::<usize>()
        .unwrap();
    Set { address, val }
}

fn first_part<'a>(mut it: impl Iterator<Item = &'a Instr>) -> usize {
    let mut mem = HashMap::new();
    let mut curr_mask = match it.next() {
        Some(Instr::Mask(mask)) => mask,
        _ => panic!("Invalid Input"),
    };
    for i in it {
        match i {
            Instr::Mask(mask) => curr_mask = mask,
            Instr::Set(set) => {
                mem.insert(set.address, curr_mask.apply_to_value(set.val));
            }
        }
    }

    mem.values().sum()
}

fn second_part<'a>(mut it: impl Iterator<Item = &'a Instr>) -> usize {
    let mut mem = HashMap::new();
    let mut curr_mask = match it.next() {
        Some(Instr::Mask(mask)) => mask,
        _ => panic!("Invalid Input"),
    };
    for i in it {
        match i {
            Instr::Mask(mask) => curr_mask = mask,
            Instr::Set(set) => curr_mask
                .possible_addresses(set.address)
                .into_iter()
                .for_each(|a| {
                    mem.insert(a, set.val);
                }),
        }
    }

    mem.values().sum()
}

fn main() {
    let instr: Vec<_> = read_to_string("data/input")
        .unwrap()
        .lines()
        .map(Instr::from)
        .collect();

    dbg!(first_part(instr.iter()));

    dbg!(second_part(instr.iter()));
}
