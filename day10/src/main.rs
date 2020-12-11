use std::fs::read_to_string;

fn number_of_choices(v: &[usize]) -> usize {
    let mut cache = vec![0; v[v.len() - 1] + 1];

    for &i in v.iter() {
        // Set up initial values
        if i <= 3 {
            cache[i] += 1;

            for x in (1..i).rev() {
                cache[i] += cache[x];
            }

            continue;
        }

        cache[i] += cache[i - 3];
        cache[i] += cache[i - 2];
        cache[i] += cache[i - 1];
    }

    cache[cache.len() - 1]
}

fn main() {
    let input = read_to_string("data/input").unwrap();
    let mut joltages: Vec<_> = input
        .lines()
        .filter(|s| !s.trim().is_empty())
        .map(|l| l.parse::<usize>())
        .flatten()
        .collect();

    joltages.push(joltages.iter().max().map(|x| x + 3).unwrap());

    let mut vec_for_part_2 = joltages.clone();

    joltages.sort_by(|a, b| b.cmp(a));

    let mut curr_jolt = 0;

    let mut diffs = Vec::new();
    while !joltages.is_empty() {
        let next = joltages.pop().unwrap();
        let diff = next - curr_jolt;
        diffs.push(diff);
        curr_jolt = next;
    }

    println!(
        "1 Diff: {}, 3 Diff: {}",
        diffs.iter().filter(|&i| *i == 1).count(),
        diffs.iter().filter(|&i| *i == 3).count()
    );

    vec_for_part_2.sort();

    println!("{}", number_of_choices(&vec_for_part_2))
}
