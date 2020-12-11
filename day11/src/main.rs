use std::fs::read_to_string;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum States {
    Free,
    Occupied,
    Floor,
}

fn adj_occ(table: &Vec<Vec<States>>, row: usize, col: usize) -> usize {
    let (row, col) = (row as i64, col as i64);
    let indexes = [
        (row - 1, col - 1),
        (row, col - 1),
        (row + 1, col - 1),
        (row - 1, col),
        (row + 1, col),
        (row - 1, col + 1),
        (row, col + 1),
        (row + 1, col + 1),
    ];

    let mut count = 0;
    for &(i, j) in indexes.iter() {
        if i < 0 || j < 0 {
            continue;
        }

        let cell = table
            .get(i as usize)
            .and_then(|row| row.get(j as usize))
            .copied()
            .unwrap_or(States::Floor);
        count += (cell == States::Occupied) as usize;
    }

    count
}

fn seen_occ(table: &Vec<Vec<States>>, row: usize, col: usize) -> usize {
    let num_rows = table.len();
    let num_cols = table[0].len();

    let possible_ways = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    let mut res = 0;
    for (dir_r, dir_c) in possible_ways.iter() {
        let mut current_row = row as isize + dir_r;
        let mut current_col = col as isize + dir_c;

        while 0 <= current_row
            && current_row < num_rows as isize
            && 0 <= current_col
            && current_col < num_cols as isize
        {
            match table[current_row as usize][current_col as usize] {
                States::Floor => {}
                States::Occupied => {
                    res += 1;
                    break;
                }
                States::Free => break,
            }

            current_row += dir_r;
            current_col += dir_c;
        }
    }

    res
}

fn main() {
    let input = read_to_string("data/input").unwrap();
    let table: Vec<_> = input
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| {
            l.trim()
                .chars()
                .map(|c| match c {
                    '#' => States::Occupied,
                    'L' => States::Free,
                    '.' => States::Floor,
                    _ => panic!("Heh"),
                })
                .collect::<Vec<_>>()
        })
        .collect();

    let mut iter_table = table.clone();

    loop {
        let mut new_table = iter_table.clone();
        for (i, row) in iter_table.iter().enumerate() {
            for (j, val) in row.iter().enumerate() {
                let new_val = match val {
                    States::Floor => States::Floor,
                    States::Free => {
                        let adj = adj_occ(&iter_table, i, j);
                        if adj == 0 {
                            States::Occupied
                        } else {
                            States::Free
                        }
                    }
                    States::Occupied => {
                        let adj = adj_occ(&iter_table, i, j);
                        if adj >= 4 {
                            States::Free
                        } else {
                            States::Occupied
                        }
                    }
                };

                new_table[i][j] = new_val;
            }
        }
        if new_table == iter_table {
            println!(
                "{}",
                iter_table
                    .iter()
                    .flatten()
                    .filter(|&v| *v == States::Occupied)
                    .count()
            );
            break;
        }
        iter_table = new_table;
    }

    iter_table = table.clone();

    loop {
        let mut new_table = iter_table.clone();
        for (i, row) in iter_table.iter().enumerate() {
            for (j, val) in row.iter().enumerate() {
                let new_val = match val {
                    States::Floor => States::Floor,
                    States::Free => {
                        let adj = seen_occ(&iter_table, i, j);
                        if adj == 0 {
                            States::Occupied
                        } else {
                            States::Free
                        }
                    }
                    States::Occupied => {
                        let adj = seen_occ(&iter_table, i, j);
                        if adj >= 5 {
                            States::Free
                        } else {
                            States::Occupied
                        }
                    }
                };

                new_table[i][j] = new_val;
            }
        }
        if new_table == iter_table {
            println!(
                "{}",
                iter_table
                    .iter()
                    .flatten()
                    .filter(|&v| *v == States::Occupied)
                    .count()
            );
            break;
        }
        iter_table = new_table;
    }
}
