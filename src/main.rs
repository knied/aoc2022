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

    let res : u32 = content
        .lines()
        .map(|line| {
            let items : Vec<u8> = line.chars().map(|c| c as u8).collect();
            let item_count = items.len();
            let compartment_size = item_count / 2;
            for i in 0..compartment_size {
                let item = items[i];
                for j in compartment_size..item_count {
                    if items[j] == item {
                        return prio(item);
                    }
                }
            }
            panic!("no duplicate item");
        })
        .sum();
    println!("{}", res);
}
