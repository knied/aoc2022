use std::fs;
use std::ops::Range;

fn merge<T: std::cmp::Ord>(r0: Range<T>, r1: Range<T>) -> Range<T> {
    Range {
        start: std::cmp::min::<T>(r0.start, r1.start),
        end: std::cmp::max::<T>(r0.end, r1.end),
    }
}

fn contained(r0: &Range<u32>, r1: &Range<u32>) -> bool {
    r0.contains(&r1.start) && r0.contains(&(r1.end - 1))
}

fn main() {
    let content = fs::read_to_string("input/day4a.txt")
        .expect("Unable to read input file.");

    let count = content
        .lines()
        .map(|line| -> bool {
            let ranges = line
                .split(',')
                .map(|elf| {
                    elf.split('-')
                        .map(|s| s.parse::<u32>().expect("must be number"))
                        .map(|n| Range::<u32> {
                            start: n,
                            end: n + 1,
                        })
                        .reduce(merge)
                        .expect("must be valid ranges")
                })
                .collect::<Vec<_>>();
            contained(&ranges[0], &ranges[1])
                || contained(&ranges[1], &ranges[0])
        })
        .fold(
            0_u32,
            |count, fully_contained| {
                if fully_contained {
                    count + 1
                } else {
                    count
                }
            },
        );
    println!("{}", count);
}
