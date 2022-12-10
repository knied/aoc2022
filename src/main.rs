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
                    tokens[1].parse::<i32>().expect("should be an i32")),
                _ => panic!("invalid instruction")
            }
        })
        .collect::<Vec<_>>();

    let mut x : i32 = 1;
    let mut cycle : i32 = 0;
    let mut next_sample : i32 = 20;
    let mut result : i32 = 0;
    instr
        .iter().for_each(|i| {
            let mut next_x = x;
            match i {
                Instr::Noop => {
                    cycle += 1;
                    println!("{}: noop (x = {}))", cycle, x);
                },
                Instr::Addx(v) => {
                    cycle += 2;
                    next_x += v;
                    println!("{}: addx {} (x = {} -> {}))", cycle, v, x, next_x);
                }
            };
            
            while cycle >= next_sample {
                result += next_sample * x;
                println!("{}: sample {} (x = {}))", cycle, next_sample, x);
                next_sample += 40;
            }
            x = next_x;
        });
    println!("result = {}", result);
}
