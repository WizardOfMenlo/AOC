use std::io;
use std::{collections::HashMap, fs::read_to_string};

fn in_range(s: impl ToString, lower: usize, upper: usize) -> bool {
    let value: usize = match s.to_string().parse().ok() {
        Some(value) => value,
        None => return false,
    };

    lower <= value && value <= upper
}

fn main() -> io::Result<()> {
    let input = read_to_string("data/input.txt")?;

    let required_keys = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

    let passports: Vec<_> = input
        .split("\r\n\r\n")
        .map(|l| {
            l.split_whitespace()
                .map(|f| f.trim())
                .filter(|f| !f.is_empty())
                .map(|field| {
                    let pair: Vec<_> = field.split(':').collect();
                    (pair[0], pair[1])
                })
                .collect::<HashMap<_, _>>()
        })
        .collect();

    dbg!(&passports);

    let valid_passports = passports
        .into_iter()
        .filter(|p| required_keys.iter().all(|key| p.contains_key(key)))
        .collect::<Vec<_>>();

    println!("Valid lines {}", valid_passports.len());

    let pass_all = valid_passports
        .iter()
        .filter(|p| {
            p.iter()
                .map(|(key, value)| (key.to_string(), value.to_string()))
                .all(|(key, value)| match key.as_str() {
                    "byr" => in_range(value, 1920, 2002),
                    "iyr" => in_range(value, 2010, 2020),
                    "eyr" => in_range(value, 2020, 2030),
                    "hgt" => {
                        if value.ends_with("cm") {
                            in_range(value.trim_end_matches("cm"), 150, 193)
                        } else if value.ends_with("in") {
                            in_range(value.trim_end_matches("in"), 59, 76)
                        } else {
                            false
                        }
                    }
                    "hcl" => {
                        value.len() == 7
                            && value.starts_with('#')
                            && value.chars().skip(1).all(|c| c.is_alphanumeric())
                    }
                    "ecl" => {
                        ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&value.as_str())
                    }
                    "pid" => value.len() == 9 && in_range(value, 0, 999999999),
                    "cid" => true,
                    _ => false,
                })
        })
        .count();

    dbg!(pass_all);

    Ok(())
}
