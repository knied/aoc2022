use std::fs;

fn prio(item: &u8) -> u32 {
    if (item - b'A').lt(&26) {
        (item - b'A') as u32 + 27
    } else {
        (item - b'a') as u32 + 1
    }
}

fn main() {
    let content = fs::read_to_string("input/day3a.txt")
        .expect("Unable to read input file.");

    let mut sum: u32 = 0;
    let mut lines = content.lines();

    loop {
        let group = lines
            .by_ref()
            .take(3)
            .map(|line| line.chars().map(|c| c as u8).collect());
        let rucksacks: Vec<Vec<u8>> = group.collect();
        if rucksacks.is_empty() {
            break;
        }
        for item in &rucksacks[0] {
            if rucksacks
                .iter()
                .skip(1)
                .map(|r| r.contains(item))
                .all(|in_rucksack| in_rucksack)
            {
                sum += prio(item);
                break;
            }
        }
    }
    println!("{}", sum)
}
