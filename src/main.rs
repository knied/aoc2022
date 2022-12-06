use std::fs;
use std::collections::HashSet;

const MARKER_LEN : usize = 14;
fn main() {
    let content = fs::read_to_string("input/day6a.txt")
        .expect("Unable to read input file.");

    let msg = content.chars().collect::<Vec<_>>();
    for idx in 0..msg.len() {
        let unique = msg.iter().skip(idx).take(MARKER_LEN).collect::<HashSet<_>>().len();
        if unique == MARKER_LEN {
            println!("{}", idx+MARKER_LEN);
        }
    }
}
