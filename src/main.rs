use std::fmt;
use std::fs;

struct Storage {
    stacks: Vec<Vec<char>>,
}

trait Stacks<T> {
    fn put(&mut self, idx: usize, item: T);
    fn take(&mut self, idx: usize) -> Option<T>;
}

impl Stacks<char> for Storage {
    fn put(&mut self, idx: usize, item: char) {
        if idx >= self.stacks.len() {
            self.stacks.resize(idx + 1, Vec::<char>::new())
        }
        self.stacks[idx].push(item);
    }
    fn take(&mut self, idx: usize) -> Option<char> {
        if idx >= self.stacks.len() {
            None
        } else {
            self.stacks[idx].pop()
        }
    }
}

impl std::str::FromStr for Storage {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut ret = Storage {
            stacks: Vec::<Vec<char>>::new(),
        };
        s.lines()
            .rev()
            .skip(1)
            .map(|line| {
                let mut idx: usize = 0;
                line.chars()
                    .collect::<Vec<_>>()
                    .chunks(4)
                    .map(|c| c[1])
                    .all(|item| {
                        if item != ' ' {
                            ret.put(idx, item);
                        }
                        idx += 1;
                        true
                    })
            })
            .all(|_| true);
        Ok(ret)
    }
}

impl std::fmt::Display for Storage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut res = String::new();
        for s in &self.stacks {
            res.push(*s.last().expect("there should be at least one box"));
        }
        write!(f, "{}", res)
    }
}

fn main() {
    let content = fs::read_to_string("input/day5a.txt")
        .expect("Unable to read input file.");

    let parts = content.split("\n\n").collect::<Vec<_>>();

    let mut storage = parts[0].parse::<Storage>().expect("should be storage");
    parts[1]
        .lines()
        .map(|line| {
            line.split_whitespace()
                .collect::<Vec<_>>()
                .chunks(2)
                .map(|c| c[1].parse::<usize>().expect("should be a number"))
                .collect::<Vec<_>>()
        })
        .all(|instr| {
            let count = instr[0];
            let from = instr[1] - 1;
            let to = instr[2] - 1;
            let mut tmp = Vec::<char>::new();
            for _ in 0..count {
                let item =
                    storage.take(from).expect("stack should not be empty");
                tmp.push(item);
            }
            for item in tmp.iter().rev() {
                storage.put(to, *item);
            }
            true
        });

    println!("{}", storage);
}
