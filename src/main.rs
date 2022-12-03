use std::fs;

fn prio(item : u8) -> u32 {
    if (item - 'A' as u8).lt(&26) {
        return (item - 'A' as u8) as u32 + 27;
    } else {
        return (item - 'a' as u8) as u32 + 1;
    }
}

fn main() {
    let content = fs::read_to_string("input/day3a.txt")
        .expect("Unable to read input file.");

    let mut sum : u32 = 0;
    let mut lines = content.lines();
    loop {
        let group = lines.by_ref().take(3)
            .map(|line| -> Vec<u32> {
                return line.chars().map(|c| prio(c as u8)).collect()
            });
        let rucksacks : Vec<Vec<u32>> = group.collect();
        if rucksacks.is_empty() {
            break;
        }
        for item in &rucksacks[0] {
            if rucksacks.iter().skip(1).map(|r| r.contains(item))
                .fold(true, |is_badge,in_rucksack| is_badge && in_rucksack) {
                    sum += item;
                    break;
                }
        }
    }
    println!("{}", sum)
}
