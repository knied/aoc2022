use std::fs;

fn main() {
    let content = fs::read_to_string("input/day1a.txt")
        .expect("Unable to read input file.");
    let mut max_calories : i32 = 0;
    let mut elf_calories : i32 = 0;
    for line in content.split("\n") {
        if let Some(calories) = line.parse::<i32>().ok() {
            elf_calories += calories;
        } else {
            println!("elf calories: {}", elf_calories);
            if elf_calories > max_calories {
                max_calories = elf_calories;
            }
            elf_calories = 0;
        }
    }
    println!("max calories: {}", max_calories);
}
