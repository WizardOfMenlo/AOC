#![feature(map_first_last)]

use std::{
    collections::{BTreeSet, HashMap, HashSet},
    fs::read_to_string,
};

use regex::Regex;

fn main() {
    let input = read_to_string("data/input").unwrap();
    let re = Regex::new("(\\d+) ([a-z\\s]+) bags?").unwrap();
    let bags: HashMap<_, _> = input
        .lines()
        .filter(|s| !s.trim().is_empty())
        .map(|s| {
            let mut it = s.trim().split("contain");
            let ty = it.next().unwrap().replace("bags", "").trim().to_string();
            let mult = it.next().unwrap();
            let contain: Vec<_> = mult
                .split(',')
                .filter(|segment| segment.trim() != "no other bags.")
                .map(|segment| {
                    let segment = segment.trim();
                    let caps = re.captures(segment).unwrap();
                    let num = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();
                    let name = caps.get(2).unwrap().as_str().trim().to_string();
                    (num, name)
                })
                .collect();

            (ty, contain)
        })
        .collect();

    let mut set = HashMap::new();
    for (ty, contain) in &bags {
        contain
            .iter()
            .cloned()
            .for_each(|(_, c_ty)| set.entry(c_ty).or_insert_with(Vec::new).push(ty.clone()));
    }

    let mut targets = BTreeSet::new();
    targets.insert("shiny gold".to_string());
    let mut seen_so_far = HashSet::new();

    while !targets.is_empty() {
        let next = targets.pop_first().unwrap();
        let can_contain_next = set.get(&next).cloned().unwrap_or_default();
        seen_so_far.insert(next);
        targets.extend(
            can_contain_next
                .into_iter()
                .filter(|s| !seen_so_far.contains(s)),
        );
    }

    println!("{:?}", seen_so_far.len());
    println!("{}", bag_count("shiny gold".to_string(), &bags))
}

fn bag_count(bag: String, bag_rules: &HashMap<String, Vec<(usize, String)>>) -> usize {
    let mut res = 1; // self
    if let Some(contained_elements) = bag_rules.get(&bag) {
        for (count, contained_bag) in contained_elements {
            res += count * bag_count(contained_bag.clone(), bag_rules);
        }
    }

    res
}
