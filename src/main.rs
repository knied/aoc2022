use std::fs;
use std::ops::*;
use std::collections::HashSet;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
struct V2(i32, i32);

impl Add for V2 {
    type Output = V2;

    fn add(self, rhs: Self) -> Self::Output {
        V2(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Sub for V2 {
    type Output = V2;

    fn sub(self, rhs: Self) -> Self::Output {
        V2(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl Div for V2 {
    type Output = V2;

    fn div(self, rhs: Self) -> Self::Output {
        V2(self.0 / rhs.0, self.1 / rhs.1)
    }
}

fn main() {
    let content = fs::read_to_string("input/day9a.txt")
        .expect("Unable to read input file.");

    let movements = content
        .lines()
        .map(|line| {
            let t = line
                .split_whitespace()
                .collect::<Vec<_>>();
            let count = t[1]
                .parse::<u32>()
                .expect("invalid input");
            let direction = match t[0] {
                "L" => V2(-1,0),
                "R" => V2(1,0),
                "U" => V2(0,1),
                "D" => V2(0,-1),
                _ => panic!("invalid input")
            };
            (direction, count)
        })
        .collect::<Vec<_>>();

    let mut visited = HashSet::<V2>::new();
    let mut rope : [V2; 10] = [V2(0,0); 10];
    for m in movements {
        for _ in 0..m.1 {
            rope[0] = rope[0] + m.0;
            for i in 0..9 {
                let d = (rope[i] - rope[i+1]) / V2(2,2);
                if d != V2(0,0) {
                    rope[i+1] = rope[i] - d;
                }
            }
            visited.insert(rope[9]);
        }
    }
    println!("{}", visited.len());
}
