use std::fs;

fn visible1(i: usize, heights: &Vec<i32>) -> bool {
    let height = heights[i];
    let left = heights
        .iter()
        .take(i)
        .fold(-1, |max, h| std::cmp::max(max, *h));
    if left < height {
        return true;
    }
    let right = heights
        .iter()
        .skip(i + 1)
        .take(heights.len() - i - 1)
        .fold(-1, |max, h| std::cmp::max(max, *h));
    if right < height {
        return true;
    }
    false
}

type Heights = Vec<Vec<i32>>;
fn visible2(r: usize, c: usize, heights: &Heights) -> bool {
    // horizontal
    let row = &heights[r];
    if visible1(c, row) {
        return true;
    }
    let col = heights.iter().map(|row| row[c]).collect::<Vec<_>>();
    if visible1(r, &col) {
        return true;
    }
    false
}

fn main() {
    let content = fs::read_to_string("input/day8a.txt")
        .expect("Unable to read input file.");

    let heights = content
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c as i32 - b'0' as i32)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut count: usize = 0;
    for r in 0..heights.len() {
        for c in 0..heights[r].len() {
            if visible2(r, c, &heights) {
                count += 1;
                print!("{}", heights[r][c]);
            } else {
                print!(" ");
            }
        }
        println!();
    }
    println!("{} of {}", count, heights.len() * heights[0].len());
}
