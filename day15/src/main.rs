use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
enum SeenNums {
    Once(usize),
    Twice(usize, usize),
}

const INPUT: &[usize] = &[10, 16, 6, 0, 1, 17];

fn get_elements_in_seq_up_to(lim: usize) {
    let mut seen = HashMap::new();
    let mut last = 0;
    for i in 1..=lim {
        let next = if i <= INPUT.len() {
            INPUT[i - 1]
        } else {
            match seen.get(&last) {
                None => panic!("This shouldn't happen"),
                Some(SeenNums::Once(_)) => 0,
                Some(SeenNums::Twice(pos1, pos2)) => pos2 - pos1,
            }
        };
        seen.insert(
            next,
            match seen.get(&next) {
                None => SeenNums::Once(i),
                Some(SeenNums::Once(last)) => SeenNums::Twice(*last, i),
                Some(SeenNums::Twice(_, pos2)) => SeenNums::Twice(*pos2, i),
            },
        );
        last = next;
    }
    dbg!(last);
}

fn main() {
    get_elements_in_seq_up_to(2020);
    get_elements_in_seq_up_to(30000000);
}
