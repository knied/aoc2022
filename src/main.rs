use std::fs;

fn main() {
    let content = fs::read_to_string("input/day1a.txt")
        .expect("Unable to read input file.");
    let mut elf_list = Vec::<i32>::new();
    let mut elf_calories : i32 = 0;
    for line in content.split("\n") {
        if let Some(calories) = line.parse::<i32>().ok() {
            elf_calories += calories;
        } else {
            //println!("elf calories: {}", elf_calories);
            elf_list.push(elf_calories);
            elf_calories = 0;
        }
    }
    elf_list.sort_by(|a, b| b.cmp(a));
    let mut sum : i32 = 0;
    for i in 0..3 {
        let calories = elf_list[i];
        println!("elf: {}", calories);
        sum += calories;
    }
    println!("top three elves carry: {}", sum);
}
