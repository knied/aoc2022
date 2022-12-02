use std::fs;

fn main() {
    let content = fs::read_to_string("input/day2a.txt")
        .expect("Unable to read input file.");

    let res : usize = content
        .lines()
        .map(|line| {
            let c : Vec<usize> = line.split_whitespace()
                .map(|s| s.chars().map(|c| c as usize).sum::<usize>())
                .collect();
            let elf = c[0] - ('A' as usize);
            //let me = c[1] - ('X' as usize);
            let me = ((c[1] - ('X' as usize) + 2) % 3 + elf) % 3;
            return (((4 - elf) % 3 + me) % 3) * 3 + me + 1;
        }).sum();

    println!("{}", res);
}
