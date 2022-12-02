use std::fs;

enum HandSign {
    Rock,
    Paper,
    Scissor,
}

// This looks aweful :D
fn score(elf : &Option<HandSign>, me : &Option<HandSign>) -> Option<i32> {
    match elf {
        Some(HandSign::Rock) => match me {
            Some(HandSign::Rock) => Some(3 + 1), // draw
            Some(HandSign::Paper) => Some(6 + 2), // win
            Some(HandSign::Scissor) => Some(0 + 3), // lose
            None => None,
        },
        Some(HandSign::Paper) => match me {
            Some(HandSign::Rock) => Some(0 + 1), // lose
            Some(HandSign::Paper) => Some(3 + 2), // draw
            Some(HandSign::Scissor) => Some(6 + 3), // win
            None => None,
        },
        Some(HandSign::Scissor) => match me {
            Some(HandSign::Rock) => Some(6 + 1), // win
            Some(HandSign::Paper) => Some(0 + 2), // lose
            Some(HandSign::Scissor) => Some(3 + 3), // draw
            None => None,
        },
        None => None,
    }
}

fn main() {
    let content = fs::read_to_string("input/day2a.txt")
        .expect("Unable to read input file.");

    let res : Option<i32> = content
        .lines()
        .map(|line| -> Option<i32> {
            let round : Vec<Option<HandSign>>= line.split_whitespace()
                .map(|sign| -> Option<HandSign> {
                    match sign {
                        "A" => Some(HandSign::Rock), // rock
                        "B" => Some(HandSign::Paper), // paper
                        "C" => Some(HandSign::Scissor), // scissor

                        "X" => Some(HandSign::Rock), // rock
                        "Y" => Some(HandSign::Paper), // paper
                        "Z" => Some(HandSign::Scissor), // scissor

                        _ => None,
                    }
                }).collect();
            return score(&round[0], &round[1]);
        })
        .sum();
    println!("{:?}", res);
}
