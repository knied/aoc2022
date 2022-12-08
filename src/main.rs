use std::fs;

fn score1(i: usize, heights: &Vec<i32>) -> (bool, usize) {
    let height = heights[i];
    let left_oversee = heights
        .iter()
        .take(i)
        .rev()
        .take_while(|h| **h < height)
        .count();
    let left = if left_oversee < i {
        (false, left_oversee + 1) // +1 for the tree that is blocking our view
    } else {
        (true, left_oversee)
    };

    let right_oversee = heights
        .iter()
        .skip(i + 1)
        .take_while(|h| **h < height)
        .count();
    let right = if right_oversee < (heights.len() - i - 1) {
        (false, right_oversee + 1) // +1 for the tree that is blocking our view
    } else {
        (true, right_oversee)
    };
    (left.0 || right.0, left.1 * right.1)
}

type Heights = Vec<Vec<i32>>;
fn score2(r: usize, c: usize, heights: &Heights) -> (bool, usize) {
    let row = &heights[r];
    let h = score1(c, row);
    let col = heights.iter().map(|row| row[c]).collect::<Vec<_>>();
    let v = score1(r, &col);
    (h.0 || v.0, h.1 * v.1)
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
    let mut best_score: usize = 0;
    for r in 0..heights.len() {
        for c in 0..heights[r].len() {
            let (visible, score) = score2(r, c, &heights);
            best_score = std::cmp::max(best_score, score);
            if visible {
                count += 1;
                print!("{}", heights[r][c]);
            } else {
                print!(" ");
            }
        }
        println!();
    }
    println!("{} of {}", count, heights.len() * heights[0].len());
    println!("best score: {}", best_score);
}
