use std::fs::read_to_string;

fn main() {
    let input = read_to_string("data/input").unwrap();
    let mut lines = input.lines();
    let timestamp = lines.next().unwrap().parse::<usize>().unwrap();
    let ids: Vec<_> = lines
        .next()
        .unwrap()
        .split(',')
        .map(|i| {
            if i.contains('x') {
                None
            } else {
                Some(i.parse::<usize>().unwrap())
            }
        })
        .collect();

    println!(
        "{:?}",
        ids.iter()
            .flatten()
            .map(|i| (i - (timestamp % i), i))
            .min_by(|a, b| a.0.cmp(&b.0))
            .unwrap()
    );

    // Eq t === i (n_i)
    let eq_pars: Vec<_> = ids
        .iter()
        .enumerate()
        .map(|(i, n_i)| (n_i, i))
        .filter_map(|i| i.0.map(|n_i| (n_i as i64, (n_i - (i.1 % n_i)) as i64)))
        .collect();

    dbg!(chinese_remainder(&eq_pars[..]));
}

fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

fn mod_inv(x: i64, n: i64) -> Option<i64> {
    let (g, x, _) = egcd(x, n);
    if g == 1 {
        Some((x % n + n) % n)
    } else {
        None
    }
}

fn chinese_remainder(it: &[(i64, i64)]) -> Option<i64> {
    dbg!(it);
    let prod = it.iter().map(|(n_i, _)| n_i).product::<i64>();

    let mut sum = 0;

    for (modulus, residue) in it.iter() {
        let p = prod / modulus;
        sum += residue * mod_inv(p, *modulus)? * p
    }

    Some(sum % prod)
}
