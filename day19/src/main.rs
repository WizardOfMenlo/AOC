use std::{collections::HashMap, fs::read_to_string};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct RuleId(usize);

#[derive(Debug, Clone)]
struct Rule {
    id: RuleId,
    body: RuleBody,
}

impl From<&str> for Rule {
    fn from(line: &str) -> Self {
        let parts: Vec<_> = line.split(':').collect();
        let id = RuleId(parts[0].parse().unwrap());
        let body = parts[1].trim().into();
        Rule { id, body }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum RuleBody {
    Char(char),
    Concat(Concat),
    Disj(Concat, Concat),
}

impl RuleBody {
    fn matches<'a>(&self, rules: &HashMap<RuleId, Rule>, unparsed: &'a [char]) -> Vec<&'a [char]> {
        if unparsed.is_empty() {
            return vec![];
        }

        match self {
            RuleBody::Char(c) if unparsed[0] == *c => vec![&unparsed[1..]],
            RuleBody::Char(_) => vec![],
            RuleBody::Concat(concat) => concat.matches(rules, unparsed),
            RuleBody::Disj(a, b) => a
                .matches(rules, unparsed)
                .iter()
                .chain(b.matches(rules, unparsed).iter())
                .cloned()
                .collect(),
        }
    }
}

impl From<&str> for RuleBody {
    fn from(line: &str) -> Self {
        let line = line.trim();
        if line.contains('"') {
            let chars: Vec<_> = line.chars().collect();
            let index = chars.iter().position(|p| *p == '"').unwrap();
            return RuleBody::Char(chars[index + 1]);
        }

        if line.contains('|') {
            let secs: Vec<_> = line.split('|').map(Concat::from).collect();
            RuleBody::Disj(secs[0].clone(), secs[1].clone())
        } else {
            RuleBody::Concat(line.into())
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Concat {
    rules: Vec<RuleId>,
}

impl Concat {
    fn matches<'a>(&self, rules: &HashMap<RuleId, Rule>, unparsed: &'a [char]) -> Vec<&'a [char]> {
        match &self.rules[..] {
            [r] => rules.get(r).unwrap().body.matches(rules, unparsed),
            [r1, r2] => {
                let r1 = rules.get(r1).unwrap();
                let r2 = rules.get(r2).unwrap();
                r1.body
                    .matches(rules, unparsed)
                    .iter()
                    .map(|m| r2.body.matches(rules, m))
                    .flatten()
                    .collect()
            }
            [r1, r2, r3] => {
                let r1 = rules.get(r1).unwrap();
                let r2 = rules.get(r2).unwrap();
                let r3 = rules.get(r3).unwrap();
                r1.body
                    .matches(rules, unparsed)
                    .iter()
                    .map(|m| {
                        r2.body
                            .matches(rules, m)
                            .iter()
                            .map(|m| r3.body.matches(rules, m))
                            .flatten()
                            .collect::<Vec<_>>()
                    })
                    .flatten()
                    .collect()
            }

            _ => panic!("Some"),
        }
    }
}

impl From<&str> for Concat {
    fn from(line: &str) -> Self {
        Concat {
            rules: line
                .trim()
                .split_whitespace()
                .map(|i| RuleId(i.parse().unwrap()))
                .collect(),
        }
    }
}

fn main() {
    let input = read_to_string("data/input").unwrap();

    let sections: Vec<_> = input.split("\n\n").collect();

    let rules: HashMap<_, _> = sections[0]
        .lines()
        .map(Rule::from)
        .map(|r| (r.id, r))
        .collect();

    let res = sections[1]
        .lines()
        .map(|l| l.trim())
        .filter(|l| {
            rules
                .get(&RuleId(0))
                .unwrap()
                .body
                .matches(&rules, &l.chars().collect::<Vec<_>>())
                .iter()
                .any(|m| m.is_empty())
        })
        .count();

    dbg!(res);
}
