use std::fs;

enum Instr {
    Noop,
    Addx(i32),
}

fn main() {
    let content = fs::read_to_string("input/day10a.txt")
        .expect("Unable to read input file.");

    let instr = content
        .lines()
        .map(|line| {
            let tokens = line.split_whitespace().collect::<Vec<_>>();
            match tokens[0] {
                "noop" => Instr::Noop,
                "addx" => Instr::Addx(
                    tokens[1].parse::<i32>().expect("should be an i32"),
                ),
                _ => panic!("invalid instruction"),
            }
        })
        .collect::<Vec<_>>();

    let mut x: i32 = 1;
    let mut cycle: i32 = 0;
    instr.iter().for_each(|i| {
        let mut next_x = x;
        let cycles = match i {
            Instr::Noop => 1,
            Instr::Addx(v) => {
                next_x += v;
                2
            }
        };
        for _ in 0..cycles {
            if x - 1 <= cycle && x + 1 >= cycle {
                print!("#");
            } else {
                print!(".");
            }
            cycle += 1;
            if cycle >= 40 {
                println!();
                cycle = 0;
            }
        }
        x = next_x;
    });
}
