use std::fs::read_to_string;

fn main() {
    let input = read_to_string("data/input").unwrap();
    let inputs: Vec<usize> = input
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| l.parse::<usize>())
        .flatten()
        .collect();

    dbg!(inputs.len());

    let mut target = 0;
    for i in 25..inputs.len() {
        let mut found = false;
        let curr = inputs[i];
        for j in i - 25..i {
            for k in i - 25..i {
                if inputs[j] + inputs[k] == curr {
                    found = true;
                }
            }
        }
        if !found {
            println!("{}", curr);
            target = curr;
            break;
        }
    }

    for i in 0..inputs.len() {
        for j in (i + 1)..(inputs.len() - i + 1) {
            let subseq = &inputs[i..j];
            let sum = subseq.iter().sum::<usize>();
            if sum == target {
                println!(
                    "Found {}",
                    subseq.iter().min().unwrap() + subseq.iter().max().unwrap()
                );
            }
            if sum > target {
                break;
            }
        }
    }
}
