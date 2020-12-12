use num::complex::Complex64;
use std::fs::read_to_string;
#[derive(Debug, Copy, Clone)]
enum Action {
    North(f64),
    South(f64),
    East(f64),
    West(f64),
    Forward(f64),
    RRight(f64),
    RLeft(f64),
}

fn main() {
    let input = read_to_string("data/input").unwrap();
    let instr: Vec<_> = input
        .lines()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|l| {
            let arg = l[1..].parse::<f64>().unwrap();
            match &l[0..1] {
                "N" => Action::North(arg),
                "S" => Action::South(arg),
                "E" => Action::East(arg),
                "W" => Action::West(arg),
                "F" => Action::Forward(arg),
                "L" => Action::RLeft(arg),
                "R" => Action::RRight(arg),
                _ => panic!("Shouldn't happen"),
            }
        })
        .collect();

    let mut curr_pos = (0f64, 0f64, 0f64);
    for i in &instr {
        match i.clone() {
            Action::North(amount) => curr_pos.1 += amount,
            Action::South(amount) => curr_pos.1 -= amount,
            Action::East(amount) => curr_pos.0 += amount,
            Action::West(amount) => curr_pos.0 -= amount,
            Action::Forward(amount) => {
                let c = Complex64::new(0f64, curr_pos.2).exp().scale(amount);
                curr_pos.0 += c.re;
                curr_pos.1 += c.im;
            }
            Action::RRight(angle) => {
                let angle = angle.to_radians();
                curr_pos.2 -= angle
            }
            Action::RLeft(angle) => {
                let angle = angle.to_radians();
                curr_pos.2 += angle
            }
        }
    }
    println!("Curr: {:?}", curr_pos);

    let mut ship_pos = Complex64::new(0.0, 0.0);
    let mut waypoint_pos = Complex64::new(10.0, 1.0);
    for i in &instr {
        match i.clone() {
            Action::North(amount) => waypoint_pos += Complex64::new(0f64, amount),
            Action::South(amount) => waypoint_pos -= Complex64::new(0f64, amount),
            Action::East(amount) => waypoint_pos += Complex64::new(amount, 0f64),
            Action::West(amount) => waypoint_pos -= Complex64::new(amount, 0f64),
            Action::Forward(amount) => {
                ship_pos += waypoint_pos.scale(amount);
            }
            Action::RRight(angle) => {
                let angle = angle.to_radians();
                let rot = Complex64::new(0f64, -angle).exp();
                waypoint_pos *= rot;
            }
            Action::RLeft(angle) => {
                let angle = angle.to_radians();
                let rot = Complex64::new(0f64, angle).exp();
                waypoint_pos *= rot;
            }
        }
    }
    println!("Ship: {}", ship_pos);
    println!("Ship: {}", ship_pos.re + ship_pos.im);
}
