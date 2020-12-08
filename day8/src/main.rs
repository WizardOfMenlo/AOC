#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum Instruction {
    Nop(isize),
    Jump(isize),
    Acc(isize)
}

use std::fs::read_to_string;
use std::collections::HashSet;

fn run(instr: &[Instruction]) -> bool {
    let mut acc = 0;
    let mut index= 0;
    let mut seen = HashSet::new();
    loop {
        // We are done
        if index >= instr.len() {
            println!("Finished {}", acc);
            return true;
        }

        let curr_instr = instr[index].clone();
        // We have entered a loop
        if seen.contains(&index) {
            println!("{}", acc);
            return false;
        }
        seen.insert(index);
        match curr_instr {
            Instruction::Nop(_) => { index += 1; }
            Instruction::Jump(i) => if i >= 0 {
                index += i as usize
            } else {
                index -= -i as usize
            }
            Instruction::Acc(i) => { acc += i; index += 1;}
        }
    }
}

fn main() {
    let input = read_to_string("data/input").unwrap();
    let mut instr : Vec<_> = input.lines().filter(|l| !l.trim().is_empty()).map(|l| {
         let l = l.trim();
         
         let mut parts = l.split_whitespace();
         let instr = parts.next().unwrap();
         let op = parts.next().and_then(|i| i.parse::<isize>().ok());

         match instr {
            "jmp" => Instruction::Jump(op.unwrap()),
            "acc" => Instruction::Acc(op.unwrap()),
            "nop" => Instruction::Nop(op.unwrap()),
            _ => panic!("Eh?")
         }
    }).collect();

    // Part 1
    run(&instr[..]);

    for i in 0..instr.len() {
        let curr_instr = instr[i];
        let replace = match curr_instr {
            Instruction::Jump(i) => Instruction::Nop(i),
            Instruction::Nop(i) => Instruction::Jump(i),
            Instruction::Acc(_) => continue,
        };

        instr[i] = replace;
        if run(&instr[..]) {
            break;
        }
        instr[i] = curr_instr;
    }
}
